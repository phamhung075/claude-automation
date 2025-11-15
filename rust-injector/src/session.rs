use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Represents a Claude Code session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeSession {
    pub session_id: String,
    pub project_id: String,
    pub project_path: String,
    pub created_at: u64,
    pub first_message: Option<String>,
    pub model: Option<String>,
    pub jsonl_path: PathBuf,
}

/// Entry in the JSONL session file
#[derive(Debug, Clone, Deserialize)]
pub struct JsonlEntry {
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
    pub cwd: Option<String>,
    pub message: Option<JsonlMessage>,
    pub timestamp: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JsonlMessage {
    pub role: Option<String>,
    pub content: Option<serde_json::Value>,
}

/// Session detector - finds Claude Code sessions on the system
pub struct SessionDetector {
    claude_dir: PathBuf,
}

impl SessionDetector {
    /// Create a new session detector
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().context("Could not find home directory")?;
        let claude_dir = home.join(".claude");

        if !claude_dir.exists() {
            log::warn!("Claude directory not found at: {:?}", claude_dir);
        }

        Ok(Self { claude_dir })
    }

    /// List all projects in ~/.claude/projects
    pub fn list_projects(&self) -> Result<Vec<String>> {
        let projects_dir = self.claude_dir.join("projects");

        if !projects_dir.exists() {
            return Ok(Vec::new());
        }

        let mut projects = Vec::new();

        for entry in fs::read_dir(&projects_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    projects.push(name.to_string());
                }
            }
        }

        Ok(projects)
    }

    /// Get all sessions for a specific project
    pub fn get_project_sessions(&self, project_id: &str) -> Result<Vec<ClaudeSession>> {
        let project_dir = self.claude_dir.join("projects").join(project_id);

        if !project_dir.exists() {
            anyhow::bail!("Project directory not found: {}", project_id);
        }

        // Try to get project path from first JSONL file
        let project_path = self
            .get_project_path_from_jsonl(&project_dir)
            .unwrap_or_else(|_| self.decode_project_path(project_id));

        let mut sessions = Vec::new();

        for entry in fs::read_dir(&project_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                if let Some(session_id) = path.file_stem().and_then(|s| s.to_str()) {
                    let metadata = fs::metadata(&path)?;
                    let created_at = metadata
                        .created()
                        .or_else(|_| metadata.modified())
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);

                    let (first_message, model) = self.extract_first_message_and_model(&path);

                    sessions.push(ClaudeSession {
                        session_id: session_id.to_string(),
                        project_id: project_id.to_string(),
                        project_path: project_path.clone(),
                        created_at,
                        first_message,
                        model,
                        jsonl_path: path,
                    });
                }
            }
        }

        // Sort by creation time (newest first)
        sessions.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(sessions)
    }

    /// Get ALL sessions across all projects
    pub fn get_all_sessions(&self) -> Result<HashMap<String, Vec<ClaudeSession>>> {
        let mut all_sessions = HashMap::new();

        for project_id in self.list_projects()? {
            match self.get_project_sessions(&project_id) {
                Ok(sessions) => {
                    if !sessions.is_empty() {
                        all_sessions.insert(project_id, sessions);
                    }
                }
                Err(e) => {
                    log::warn!("Failed to get sessions for project {}: {}", project_id, e);
                }
            }
        }

        Ok(all_sessions)
    }

    /// Read project path from JSONL files
    fn get_project_path_from_jsonl(&self, project_dir: &PathBuf) -> Result<String> {
        for entry in fs::read_dir(project_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                let file = fs::File::open(&path)?;
                let reader = BufReader::new(file);

                if let Some(Ok(first_line)) = reader.lines().next() {
                    let entry: JsonlEntry = serde_json::from_str(&first_line)?;
                    if let Some(cwd) = entry.cwd {
                        return Ok(cwd);
                    }
                }
            }
        }

        anyhow::bail!("Could not find project path in JSONL files")
    }

    /// Decode project directory name to path (fallback)
    fn decode_project_path(&self, encoded: &str) -> String {
        encoded.replace('-', "/")
    }

    /// Extract first user message and model from JSONL
    fn extract_first_message_and_model(&self, jsonl_path: &PathBuf) -> (Option<String>, Option<String>) {
        let file = match fs::File::open(jsonl_path) {
            Ok(file) => file,
            Err(_) => return (None, None),
        };

        let reader = BufReader::new(file);
        let mut model = None;

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(entry) = serde_json::from_str::<JsonlEntry>(&line) {
                    // Capture model if present
                    if model.is_none() && entry.model.is_some() {
                        model = entry.model;
                    }

                    // Find first user message
                    if let Some(message) = entry.message {
                        if message.role.as_deref() == Some("user") {
                            if let Some(content) = message.content {
                                let content_str = match content {
                                    serde_json::Value::String(s) => s,
                                    serde_json::Value::Array(arr) => {
                                        // Handle array content (e.g., text blocks)
                                        arr.iter()
                                            .filter_map(|v| v.get("text").and_then(|t| t.as_str()))
                                            .collect::<Vec<_>>()
                                            .join("\n")
                                    }
                                    _ => continue,
                                };

                                // Skip system caveat messages
                                if content_str.contains("Caveat: The messages below were generated") {
                                    continue;
                                }

                                // Skip command output
                                if content_str.starts_with("<command-name>") {
                                    continue;
                                }

                                return (Some(content_str), model);
                            }
                        }
                    }
                }
            }
        }

        (None, model)
    }
}

impl Default for SessionDetector {
    fn default() -> Self {
        Self::new().expect("Failed to create SessionDetector")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_detector() {
        let detector = SessionDetector::new().unwrap();
        let projects = detector.list_projects().unwrap();
        println!("Found {} projects", projects.len());

        for project_id in projects.iter().take(3) {
            println!("\nProject: {}", project_id);
            if let Ok(sessions) = detector.get_project_sessions(project_id) {
                println!("  Sessions: {}", sessions.len());
                for session in sessions.iter().take(3) {
                    println!("    - {} ({})", session.session_id, session.project_path);
                }
            }
        }
    }

    #[test]
    fn test_get_all_sessions() {
        let detector = SessionDetector::new().unwrap();
        let all_sessions = detector.get_all_sessions().unwrap();

        println!("Total projects with sessions: {}", all_sessions.len());
        for (project_id, sessions) in all_sessions.iter() {
            println!("  {}: {} sessions", project_id, sessions.len());
        }
    }
}

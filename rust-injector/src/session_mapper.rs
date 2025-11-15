use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningClaudeSession {
    pub session_id: String,
    pub pid: u32,
    pub project_path: String,
    pub command: String,
    pub terminal_info: Option<TerminalInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalInfo {
    pub terminal_pid: u32,
    pub terminal_name: String,
    pub terminal_cmd: String,
}

pub struct SessionMapper;

impl SessionMapper {
    /// Find all running Claude processes with their session IDs
    pub fn map_sessions_to_processes() -> Result<Vec<RunningClaudeSession>> {
        let mut mapped = Vec::new();

        // Get all running Claude processes
        let processes = crate::ProcessDetector::find_running_claude_processes()?;

        for process in processes {
            // Try to extract session info from process
            if let Some(session_info) = Self::extract_session_from_process(&process) {
                // Try to find the terminal
                let terminal_info = Self::find_terminal_for_process(process.pid);

                mapped.push(RunningClaudeSession {
                    session_id: session_info.session_id,
                    pid: process.pid,
                    project_path: session_info.project_path,
                    command: process.command,
                    terminal_info,
                });
            }
        }

        Ok(mapped)
    }

    /// Find a specific session by ID
    pub fn find_session_by_id(session_id: &str) -> Result<Option<RunningClaudeSession>> {
        let sessions = Self::map_sessions_to_processes()?;
        Ok(sessions.into_iter().find(|s| s.session_id == session_id))
    }

    /// Extract session information from process command line
    fn extract_session_from_process(process: &crate::RunningProcess) -> Option<SessionInfo> {
        // Method 1: Check /proc/PID/cwd for working directory
        #[cfg(target_os = "linux")]
        {
            let cwd = crate::ProcessDetector::get_process_cwd(process.pid)?;

            // Try to find session files in ~/.claude/projects/
            let session_id = Self::find_session_for_cwd(&cwd)?;

            Some(SessionInfo {
                session_id,
                project_path: cwd,
            })
        }

        #[cfg(not(target_os = "linux"))]
        {
            None
        }
    }

    /// Find session ID for a given working directory
    fn find_session_for_cwd(cwd: &str) -> Option<String> {
        let home = dirs::home_dir()?;
        let claude_dir = home.join(".claude/projects");

        if !claude_dir.exists() {
            return None;
        }

        // Read all project directories
        for project_entry in fs::read_dir(&claude_dir).ok()? {
            let project_path = project_entry.ok()?.path();

            // Read all session files
            for session_entry in fs::read_dir(&project_path).ok()? {
                let session_path = session_entry.ok()?.path();

                if session_path.extension()? != "jsonl" {
                    continue;
                }

                // Read the session file
                let content = fs::read_to_string(&session_path).ok()?;

                // Check if this session matches the CWD
                if content.contains(cwd) {
                    let session_id = session_path
                        .file_stem()?
                        .to_str()?
                        .to_string();
                    return Some(session_id);
                }
            }
        }

        None
    }

    /// Find the terminal emulator for a process
    #[cfg(target_os = "linux")]
    fn find_terminal_for_process(pid: u32) -> Option<TerminalInfo> {
        // Read /proc/PID/status to get parent PID
        let status_path = format!("/proc/{}/status", pid);
        let status = fs::read_to_string(&status_path).ok()?;

        let ppid = status
            .lines()
            .find(|line| line.starts_with("PPid:"))?
            .split_whitespace()
            .nth(1)?
            .parse::<u32>()
            .ok()?;

        // Read parent process command
        let parent_cmd_path = format!("/proc/{}/cmdline", ppid);
        let parent_cmd_raw = fs::read(&parent_cmd_path).ok()?;
        let parent_cmd = String::from_utf8_lossy(&parent_cmd_raw)
            .replace('\0', " ")
            .trim()
            .to_string();

        // Check if parent is a known terminal emulator
        let terminal_names = vec![
            "gnome-terminal",
            "konsole",
            "xterm",
            "alacritty",
            "kitty",
            "wezterm",
            "tmux",
            "screen",
            "code", // VSCode integrated terminal
        ];

        for term_name in terminal_names {
            if parent_cmd.contains(term_name) {
                return Some(TerminalInfo {
                    terminal_pid: ppid,
                    terminal_name: term_name.to_string(),
                    terminal_cmd: parent_cmd,
                });
            }
        }

        None
    }

    #[cfg(not(target_os = "linux"))]
    fn find_terminal_for_process(_pid: u32) -> Option<TerminalInfo> {
        None
    }
}

#[derive(Debug)]
struct SessionInfo {
    session_id: String,
    project_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_sessions() {
        let sessions = SessionMapper::map_sessions_to_processes().unwrap();
        for session in sessions {
            println!("Session: {} (PID: {})", session.session_id, session.pid);
            if let Some(ref term) = session.terminal_info {
                println!("  Terminal: {} (PID: {})", term.terminal_name, term.terminal_pid);
            }
        }
    }
}

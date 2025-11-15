use anyhow::{Context, Result};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

use crate::payload::InjectionPayload;
use crate::session::ClaudeSession;

/// Manages active Claude processes with stdin pipes for injection
pub struct ClaudeProcessManager {
    /// Active processes: session_id -> ProcessHandle
    processes: Arc<Mutex<HashMap<String, ProcessHandle>>>,
}

/// Handle to a running Claude process
pub struct ProcessHandle {
    pub session: ClaudeSession,
    pub child: Child,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl ClaudeProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Start a new Claude session with stdin/stdout/stderr pipes
    ///
    /// This spawns `claude` CLI and keeps stdin open for injection
    pub async fn start_session(
        &self,
        session: ClaudeSession,
        initial_prompt: Option<String>,
    ) -> Result<String> {
        let session_id = session.session_id.clone();

        log::info!(
            "Starting Claude session: {} in {}",
            session_id,
            session.project_path
        );

        // Build command
        let mut cmd = Command::new("claude");
        cmd.current_dir(&session.project_path)
            .stdin(Stdio::piped()) // CRITICAL: Keep stdin open for injection!
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Add initial prompt if provided
        if let Some(prompt) = initial_prompt {
            cmd.arg(prompt);
        }

        // Spawn process
        let child = cmd
            .spawn()
            .context("Failed to spawn claude process")?;

        log::info!("Spawned Claude process with PID: {:?}", child.id());

        // Store process handle
        let handle = ProcessHandle {
            session: session.clone(),
            child,
            started_at: chrono::Utc::now(),
        };

        {
            let mut processes = self.processes.lock().await;
            processes.insert(session_id.clone(), handle);
        }

        Ok(session_id)
    }

    /// Inject payload into a running session via stdin
    ///
    /// This is the KEY function that enables automatic injection!
    pub async fn inject(&self, session_id: &str, payload: InjectionPayload) -> Result<()> {
        log::info!(
            "Injecting payload into session {}: {:?}",
            session_id,
            payload.payload_type
        );

        let mut processes = self.processes.lock().await;

        let handle = processes
            .get_mut(session_id)
            .context(format!("Session {} not found in active processes", session_id))?;

        // Get stdin handle
        let stdin = handle
            .child
            .stdin
            .as_mut()
            .context("Session stdin not available")?;

        // Convert payload to string
        let message = payload.to_injection_string();

        log::debug!("Injecting message:\n{}", message);

        // Write to stdin
        stdin
            .write_all(message.as_bytes())
            .await
            .context("Failed to write to session stdin")?;

        stdin
            .write_all(b"\n")
            .await
            .context("Failed to write newline")?;

        // Flush to ensure immediate delivery
        stdin.flush().await.context("Failed to flush stdin")?;

        log::info!("Successfully injected payload into session {}", session_id);

        Ok(())
    }

    /// Inject into ALL active sessions
    pub async fn broadcast(&self, payload: InjectionPayload) -> Result<Vec<String>> {
        let session_ids: Vec<String> = {
            let processes = self.processes.lock().await;
            processes.keys().cloned().collect()
        };

        let mut injected = Vec::new();

        for session_id in session_ids {
            match self.inject(&session_id, payload.clone()).await {
                Ok(_) => {
                    injected.push(session_id.clone());
                }
                Err(e) => {
                    log::warn!("Failed to inject into session {}: {}", session_id, e);
                }
            }
        }

        Ok(injected)
    }

    /// Get list of active session IDs
    pub async fn list_active_sessions(&self) -> Vec<String> {
        let processes = self.processes.lock().await;
        processes.keys().cloned().collect()
    }

    /// Check if a session is still running
    pub async fn is_session_active(&self, session_id: &str) -> bool {
        let mut processes = self.processes.lock().await;

        if let Some(handle) = processes.get_mut(session_id) {
            // Try to check if process is still alive
            match handle.child.try_wait() {
                Ok(Some(_)) => {
                    // Process has exited
                    log::info!("Session {} has exited", session_id);
                    processes.remove(session_id);
                    false
                }
                Ok(None) => {
                    // Still running
                    true
                }
                Err(_) => {
                    // Error checking - assume dead
                    processes.remove(session_id);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Stop a session
    pub async fn stop_session(&self, session_id: &str) -> Result<()> {
        let mut processes = self.processes.lock().await;

        if let Some(mut handle) = processes.remove(session_id) {
            log::info!("Stopping session {}", session_id);
            handle.child.start_kill().context("Failed to kill process")?;
            handle.child.wait().await.context("Failed to wait for process")?;
        }

        Ok(())
    }

    /// Stop all active sessions
    pub async fn stop_all(&self) -> Result<()> {
        let session_ids: Vec<String> = {
            let processes = self.processes.lock().await;
            processes.keys().cloned().collect()
        };

        for session_id in session_ids {
            if let Err(e) = self.stop_session(&session_id).await {
                log::warn!("Failed to stop session {}: {}", session_id, e);
            }
        }

        Ok(())
    }

    /// Cleanup finished processes
    pub async fn cleanup_finished(&self) -> Vec<String> {
        let session_ids: Vec<String> = {
            let processes = self.processes.lock().await;
            processes.keys().cloned().collect()
        };

        let mut removed = Vec::new();

        for session_id in session_ids {
            if !self.is_session_active(&session_id).await {
                removed.push(session_id);
            }
        }

        removed
    }
}

impl Default for ClaudeProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payload::PayloadType;
    use crate::session::SessionDetector;

    #[tokio::test]
    async fn test_start_and_inject() {
        env_logger::init();

        // Get a real session
        let detector = SessionDetector::new().unwrap();
        let all_sessions = detector.get_all_sessions().unwrap();

        if all_sessions.is_empty() {
            println!("No sessions found - skipping test");
            return;
        }

        let session = all_sessions.values().next().unwrap()[0].clone();

        // Create manager
        let manager = ClaudeProcessManager::new();

        // Start session
        let session_id = manager
            .start_session(session, Some("Hello Claude!".to_string()))
            .await
            .unwrap();

        println!("Started session: {}", session_id);

        // Wait a bit for session to initialize
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Inject a message
        let payload = InjectionPayload {
            payload_type: PayloadType::Context,
            content: "This is injected context from Rust!".to_string(),
            metadata: None,
        };

        manager.inject(&session_id, payload).await.unwrap();

        println!("Injected message!");

        // Wait a bit
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        // Stop session
        manager.stop_session(&session_id).await.unwrap();

        println!("Test complete!");
    }
}

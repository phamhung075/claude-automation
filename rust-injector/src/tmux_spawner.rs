use anyhow::{Context, Result};
use std::process::Command;

/// Tmux-based Claude spawner - Creates visible, injectable sessions
pub struct TmuxSpawner;

impl TmuxSpawner {
    /// Check if tmux is installed
    pub fn is_available() -> bool {
        Command::new("tmux")
            .arg("-V")
            .output()
            .is_ok()
    }

    /// Spawn Claude in a new tmux session with automation settings
    pub fn spawn_session(session_name: &str, working_dir: &str) -> Result<String> {
        if !Self::is_available() {
            anyhow::bail!("tmux is not installed. Install with: sudo apt install tmux");
        }

        // Create a new tmux session running Claude with automation flags
        let output = Command::new("tmux")
            .args(&[
                "new-session",
                "-d",              // Detached (background)
                "-s", session_name, // Session name
                "-c", working_dir,  // Working directory
                "claude",          // Claude command
                "--dangerously-skip-permissions"  // Skip permission prompts for automation
            ])
            .output()
            .context("Failed to create tmux session")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to create tmux session: {}", stderr);
        }

        Ok(format!("Tmux session '{}' created with automation enabled", session_name))
    }

    /// Spawn Claude worker with agent type and automatic registration
    pub fn spawn_worker(
        name: &str,
        agent_type: &str,
        working_dir: &str,
        task_id: Option<String>,
    ) -> Result<crate::WorkerInfo> {
        // Spawn the tmux session
        Self::spawn_session(name, working_dir)?;

        // Create worker info
        let worker = crate::WorkerInfo {
            name: name.to_string(),
            agent_type: agent_type.to_string(),
            task_id,
            tmux_session: name.to_string(),
            working_dir: working_dir.to_string(),
            spawned_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            status: crate::WorkerStatus::Starting,
            messages_sent: 0,
        };

        // Register in registry
        let mut registry = crate::WorkerRegistry::load()?;
        registry.register(worker.clone())?;

        Ok(worker)
    }

    /// Inject message into a tmux session
    pub fn inject_message(session_name: &str, message: &str) -> Result<()> {
        // Send the message text with -l flag (literal, no key parsing)
        let output = Command::new("tmux")
            .args(&[
                "send-keys",
                "-l",           // Literal flag - treats input as plain text
                "-t", session_name,
                message,
            ])
            .output()
            .context("Failed to send message text")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to inject message text: {}", stderr);
        }

        // Send Enter key separately (without -l flag so it's interpreted as a key)
        let output = Command::new("tmux")
            .args(&[
                "send-keys",
                "-t", session_name,
                "Enter"
            ])
            .output()
            .context("Failed to send Enter key")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to send Enter key: {}", stderr);
        }

        Ok(())
    }

    /// Check if a tmux session exists
    pub fn session_exists(session_name: &str) -> bool {
        Command::new("tmux")
            .args(&["has-session", "-t", session_name])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// List all tmux sessions
    pub fn list_sessions() -> Result<Vec<String>> {
        let output = Command::new("tmux")
            .args(&["list-sessions", "-F", "#{session_name}"])
            .output()
            .context("Failed to list tmux sessions")?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let sessions = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();

        Ok(sessions)
    }

    /// Attach to a tmux session (returns command for user to run)
    pub fn attach_command(session_name: &str) -> String {
        format!("tmux attach-session -t {}", session_name)
    }

    /// Kill a tmux session
    pub fn kill_session(session_name: &str) -> Result<()> {
        Command::new("tmux")
            .args(&["kill-session", "-t", session_name])
            .output()
            .context("Failed to kill tmux session")?;

        Ok(())
    }

    /// Send Ctrl+C to a session
    pub fn send_interrupt(session_name: &str) -> Result<()> {
        Command::new("tmux")
            .args(&["send-keys", "-t", session_name, "C-c"])
            .output()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tmux_available() {
        println!("Tmux available: {}", TmuxSpawner::is_available());
    }

    #[test]
    fn test_list_sessions() {
        if let Ok(sessions) = TmuxSpawner::list_sessions() {
            println!("Tmux sessions: {:?}", sessions);
        }
    }
}

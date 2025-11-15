use anyhow::{Context, Result};
use std::process::Command;

/// Information about a running Claude process
#[derive(Debug, Clone)]
pub struct RunningProcess {
    pub pid: u32,
    pub command: String,
    pub working_dir: Option<String>,
}

/// Detector for finding running Claude processes on the system
pub struct ProcessDetector;

impl ProcessDetector {
    /// Find all running Claude processes
    pub fn find_running_claude_processes() -> Result<Vec<RunningProcess>> {
        #[cfg(target_os = "linux")]
        {
            Self::find_linux()
        }

        #[cfg(target_os = "macos")]
        {
            Self::find_macos()
        }

        #[cfg(target_os = "windows")]
        {
            Self::find_windows()
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            anyhow::bail!("Unsupported operating system")
        }
    }

    #[cfg(target_os = "linux")]
    fn find_linux() -> Result<Vec<RunningProcess>> {
        let output = Command::new("ps")
            .args(["aux"])
            .output()
            .context("Failed to execute ps command")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut processes = Vec::new();

        for line in stdout.lines() {
            if line.contains("claude") && !line.contains("grep") {
                if let Some(process) = Self::parse_ps_line(line) {
                    processes.push(process);
                }
            }
        }

        Ok(processes)
    }

    #[cfg(target_os = "macos")]
    fn find_macos() -> Result<Vec<RunningProcess>> {
        // Similar to Linux
        Self::find_linux()
    }

    #[cfg(target_os = "windows")]
    fn find_windows() -> Result<Vec<RunningProcess>> {
        let output = Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq claude.exe", "/FO", "CSV"])
            .output()
            .context("Failed to execute tasklist command")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut processes = Vec::new();

        for line in stdout.lines().skip(1) {
            // Skip header
            if let Some(process) = Self::parse_tasklist_line(line) {
                processes.push(process);
            }
        }

        Ok(processes)
    }

    fn parse_ps_line(line: &str) -> Option<RunningProcess> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 11 {
            return None;
        }

        let pid = parts[1].parse::<u32>().ok()?;
        let command = parts[10..].join(" ");

        Some(RunningProcess {
            pid,
            command,
            working_dir: None,
        })
    }

    fn parse_tasklist_line(line: &str) -> Option<RunningProcess> {
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 2 {
            return None;
        }

        let pid = parts[1].trim_matches('"').parse::<u32>().ok()?;
        let command = parts[0].trim_matches('"').to_string();

        Some(RunningProcess {
            pid,
            command,
            working_dir: None,
        })
    }

    /// Get working directory for a process (Linux only for now)
    #[cfg(target_os = "linux")]
    pub fn get_process_cwd(pid: u32) -> Option<String> {
        std::fs::read_link(format!("/proc/{}/cwd", pid))
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_process_cwd(_pid: u32) -> Option<String> {
        None
    }

    /// Kill a process by PID
    pub fn kill_process(pid: u32) -> Result<()> {
        #[cfg(unix)]
        {
            Command::new("kill")
                .args(["-TERM", &pid.to_string()])
                .output()
                .context("Failed to kill process")?;
        }

        #[cfg(windows)]
        {
            Command::new("taskkill")
                .args(["/F", "/PID", &pid.to_string()])
                .output()
                .context("Failed to kill process")?;
        }

        Ok(())
    }

    /// Check if a process is still running
    pub fn is_process_running(pid: u32) -> bool {
        #[cfg(unix)]
        {
            Command::new("kill")
                .args(["-0", &pid.to_string()])
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }

        #[cfg(windows)]
        {
            Command::new("tasklist")
                .args(["/FI", &format!("PID eq {}", pid), "/FO", "CSV"])
                .output()
                .map(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .count()
                        > 1
                })
                .unwrap_or(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_running_processes() {
        match ProcessDetector::find_running_claude_processes() {
            Ok(processes) => {
                println!("Found {} running Claude processes:", processes.len());
                for process in processes {
                    println!("  PID: {}, Command: {}", process.pid, process.command);

                    #[cfg(target_os = "linux")]
                    if let Some(cwd) = ProcessDetector::get_process_cwd(process.pid) {
                        println!("    Working dir: {}", cwd);
                    }
                }
            }
            Err(e) => {
                println!("Failed to find processes: {}", e);
            }
        }
    }

    #[test]
    fn test_process_detection() {
        // This test requires a Claude process to be running
        let processes = ProcessDetector::find_running_claude_processes().unwrap();

        if processes.is_empty() {
            println!("No Claude processes running - skipping test");
            return;
        }

        let pid = processes[0].pid;
        println!("Testing with PID: {}", pid);

        assert!(ProcessDetector::is_process_running(pid));
    }
}

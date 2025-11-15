use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// PTY Injector - Injects into existing Claude sessions via terminal device
pub struct PtyInjector;

impl PtyInjector {
    /// Inject message into a Claude session by writing to its controlling terminal
    pub fn inject_to_session(session_id: &str, message: &str) -> Result<()> {
        // Find the session
        let session = crate::SessionMapper::find_session_by_id(session_id)?
            .context(format!("Session '{}' not found or not running", session_id))?;

        println!("📌 Found session: {}", session.session_id);
        println!("📌 Process PID: {}", session.pid);

        // Get the controlling terminal
        let pty_path = Self::get_controlling_terminal(session.pid)?;
        println!("📌 Terminal device: {}", pty_path.display());

        // Write to the pty
        Self::write_to_pty(&pty_path, message)?;

        println!("✅ Message injected to terminal!");

        Ok(())
    }

    /// Get the controlling terminal device for a process
    #[cfg(target_os = "linux")]
    fn get_controlling_terminal(pid: u32) -> Result<PathBuf> {
        // Read /proc/PID/fd/0 (stdin) to find the terminal device
        let fd0_path = format!("/proc/{}/fd/0", pid);
        let target = std::fs::read_link(&fd0_path)
            .context("Failed to read stdin link")?;

        // Check if it's a terminal device
        let target_str = target.to_string_lossy();
        if target_str.starts_with("/dev/pts/") || target_str.starts_with("/dev/tty") {
            Ok(target)
        } else {
            anyhow::bail!("Process stdin is not a terminal device: {}", target_str);
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn get_controlling_terminal(_pid: u32) -> Result<PathBuf> {
        anyhow::bail!("PTY injection only supported on Linux");
    }

    /// Write message to a pty device using TIOCSTI to inject as keyboard input
    #[cfg(target_os = "linux")]
    fn write_to_pty(pty_path: &PathBuf, message: &str) -> Result<()> {
        use std::os::unix::io::AsRawFd;

        // Open the pty device for writing
        let pty = OpenOptions::new()
            .write(true)
            .open(pty_path)
            .context(format!(
                "Failed to open pty device: {}. You may need permissions.",
                pty_path.display()
            ))?;

        let fd = pty.as_raw_fd();

        // TIOCSTI constant (0x5412 on Linux)
        const TIOCSTI: libc::c_ulong = 0x5412;

        // Inject each character using TIOCSTI ioctl
        for byte in message.as_bytes() {
            unsafe {
                let result = libc::ioctl(fd, TIOCSTI, byte as *const u8);
                if result < 0 {
                    // TIOCSTI might be disabled in kernel 6.2+
                    return Err(anyhow::anyhow!(
                        "TIOCSTI ioctl failed. Your kernel may have disabled TIOCSTI (Linux 6.2+). \
                         Consider using tmux/screen or terminal automation tools instead."
                    ));
                }
            }
        }

        // Send Enter key
        unsafe {
            let newline: u8 = b'\n';
            libc::ioctl(fd, TIOCSTI, &newline as *const u8);
        }

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn write_to_pty(_pty_path: &PathBuf, _message: &str) -> Result<()> {
        anyhow::bail!("PTY injection with TIOCSTI only supported on Linux");
    }

    /// Inject message with proper escaping for terminal
    pub fn inject_safe(session_id: &str, message: &str) -> Result<()> {
        // Escape special characters that might cause issues
        let escaped = message.replace('\\', "\\\\").replace('\n', "\\n");

        Self::inject_to_session(session_id, &escaped)
    }

    /// Check if we have permission to write to a session's terminal
    pub fn can_inject(session_id: &str) -> Result<bool> {
        let session = crate::SessionMapper::find_session_by_id(session_id)?
            .context("Session not found")?;

        let pty_path = Self::get_controlling_terminal(session.pid)?;

        // Try to open for writing (without actually writing)
        match OpenOptions::new().write(true).open(&pty_path) {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    Ok(false)
                } else {
                    Err(e.into())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_controlling_terminal() {
        // Test with current process (should have a terminal if run from terminal)
        let pid = std::process::id();
        if let Ok(pty) = PtyInjector::get_controlling_terminal(pid) {
            println!("Current process terminal: {:?}", pty);
        }
    }
}

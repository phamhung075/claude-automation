use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<()> {
    println!("🧪 PTY Injection Demo\n");

    // Your current Claude process
    let pid = 16924;
    let pty_device = "/dev/pts/10";

    println!("📌 Target Claude PID: {}", pid);
    println!("📌 Terminal device: {}", pty_device);
    println!();

    // Test message
    let message = "🎯 PTY INJECTION TEST - This message was injected via /dev/pts/10!";

    println!("📤 Sending message: {}", message);
    println!();

    // Open the pty device
    let mut pty = OpenOptions::new()
        .write(true)
        .open(pty_device)?;

    // Write the message
    pty.write_all(message.as_bytes())?;
    pty.write_all(b"\n")?;  // Send Enter key
    pty.flush()?;

    println!("✅ Message injected!");
    println!();
    println!("💡 Check your other terminal - you should see the message!");

    Ok(())
}

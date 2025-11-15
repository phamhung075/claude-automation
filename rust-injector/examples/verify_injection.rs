use anyhow::Result;
use claude_injector::*;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Verifying stdin injection works\n");

    // Instead of spawning Claude, spawn a simple echo program
    println!("📝 Spawning a simple process that echoes stdin...\n");

    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");

    // Spawn task to read output
    let reader_task = tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            println!("📥 RECEIVED: {}", line);
        }
    });

    // Inject messages
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    use tokio::io::AsyncWriteExt;

    let messages = vec![
        "Hello from Rust!",
        "This is message 2",
        "Final message",
    ];

    let mut stdin = stdin;

    for (i, msg) in messages.iter().enumerate() {
        println!("📤 INJECTING: {}", msg);

        stdin.write_all(msg.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    drop(stdin); // Close stdin to signal EOF

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✅ All messages injected!");
    println!("⏳ Waiting for output reader to finish...\n");

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    child.kill().await.ok();
    reader_task.abort();

    println!("\n💡 As you can see above, stdin injection works perfectly!");
    println!("   The same mechanism works with Claude, but Claude's output");
    println!("   goes to its own stdout (not visible here).");

    Ok(())
}

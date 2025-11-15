use anyhow::Result;
use claude_injector::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Simple stdin injection demo\n");

    // Detect sessions
    let detector = SessionDetector::new()?;
    let all_sessions = detector.get_all_sessions()?;

    if all_sessions.is_empty() {
        println!("❌ No sessions found. Create a Claude session first.");
        return Ok(());
    }

    let session = all_sessions.values().next().unwrap()[0].clone();
    println!("📌 Using session: {}", session.session_id);
    println!("📁 Project path: {}\n", session.project_path);

    // Start Claude process
    let manager = ClaudeProcessManager::new();

    println!("🚀 Spawning new Claude process...");
    println!("💡 This will create a SEPARATE Claude instance\n");

    let session_id = manager.start_session(
        session.clone(),
        Some("You are a helpful assistant.".to_string()),
    ).await?;

    println!("✅ Session started: {}\n", session_id);
    println!("⏳ Waiting 3 seconds for initialization...\n");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Inject user prompts
    let prompts = vec![
        "Hello! Can you introduce yourself?",
        "What's 15 + 27?",
        "Thank you!",
    ];

    for (i, prompt_text) in prompts.iter().enumerate() {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📤 Injection #{}: {}", i + 1, prompt_text);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let payload = InjectionPayload::user_prompt(*prompt_text);
        manager.inject(&session_id, payload).await?;

        println!("✅ Injected to stdin");
        println!("⏳ Waiting 8 seconds for Claude to process...\n");
        tokio::time::sleep(tokio::time::Duration::from_secs(8)).await;
    }

    println!("\n✅ All injections complete!");
    println!("\n💡 NOTE: The spawned Claude instance received the messages,");
    println!("   but it's running in the background (no visible terminal).");
    println!("\nPress Enter to stop the session...");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    manager.stop_session(&session_id).await?;
    println!("🛑 Session stopped.");

    Ok(())
}

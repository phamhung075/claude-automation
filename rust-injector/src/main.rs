use anyhow::Result;
use claude_injector::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("
╔════════════════════════════════════════════════════════════╗
║   Claude Session Injector - Automatic Context Injection   ║
║   Real-time interaction with running Claude Code sessions ║
╚════════════════════════════════════════════════════════════╝
");

    // Example 1: Detect existing sessions
    println!("\n📋 Example 1: Detecting Claude Sessions");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let detector = SessionDetector::new()?;
    let all_sessions = detector.get_all_sessions()?;

    println!("Found {} projects with sessions", all_sessions.len());
    for (project_id, sessions) in all_sessions.iter().take(3) {
        println!("\n  Project: {}", project_id);
        println!("  Sessions: {}", sessions.len());
        for session in sessions.iter().take(2) {
            println!("    • {} ({})", session.session_id, session.project_path);
            if let Some(ref msg) = session.first_message {
                let preview = msg.chars().take(60).collect::<String>();
                println!("      First message: {}...", preview);
            }
        }
    }

    // Example 2: Find running Claude processes
    println!("\n\n🔍 Example 2: Finding Running Claude Processes");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    match ProcessDetector::find_running_claude_processes() {
        Ok(processes) => {
            println!("Found {} running Claude processes", processes.len());
            for process in processes {
                println!("  • PID: {} - {}", process.pid, process.command);
            }
        }
        Err(e) => {
            println!("  Error finding processes: {}", e);
        }
    }

    // Example 3: Payload creation
    println!("\n\n📦 Example 3: Creating Injection Payloads");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let payloads = vec![
        InjectionPayload::context("This is a context update from Rust!"),
        InjectionPayload::warning("Memory usage is high"),
        InjectionPayload::block("Tests are failing - please fix before continuing"),
        InjectionPayload::progress(75, "Almost done with implementation"),
        payload::presets::dependency_completed(
            "Design database schema",
            "Created 5 tables with proper indexes",
            vec![
                "Use UUID for all IDs".to_string(),
                "Add created_at/updated_at to all tables".to_string(),
            ],
        ),
    ];

    for payload in payloads {
        println!("\nPayload type: {:?}", payload.payload_type);
        println!("Injection string:\n{}", payload.to_injection_string());
    }

    // Example 4: Start session and inject (interactive example)
    println!("\n\n🚀 Example 4: Session Injection (Interactive)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if all_sessions.is_empty() {
        println!("No sessions found - skipping interactive example");
        println!("\n✅ All examples completed!");
        return Ok(());
    }

    // Get first available session
    let session = all_sessions.values().next().unwrap()[0].clone();

    println!("Would you like to test live injection? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        println!("\nStarting Claude session...");

        let manager = ClaudeProcessManager::new();

        // Start session
        let session_id = manager
            .start_session(
                session.clone(),
                Some("I am ready to receive real-time context from Rust!".to_string()),
            )
            .await?;

        println!("✓ Session started: {}", session_id);

        // Wait for session to initialize
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        // Inject a series of payloads as USER INPUT (simulating user typing)
        let injection_sequence = vec![
            (
                3,
                InjectionPayload::user_prompt(
                    "Can you help me build an authentication system with JWT tokens?",
                ),
            ),
            (
                5,
                InjectionPayload::user_prompt(
                    "Great! Can you start by creating the user model with fields for email, password_hash, and timestamps?",
                ),
            ),
            (
                5,
                InjectionPayload::user_prompt(
                    "Now let's add the JWT token generation function. It should accept user_id and return a signed token.",
                ),
            ),
            (
                5,
                InjectionPayload::user_prompt(
                    "Perfect! Can you also add token validation middleware?",
                ),
            ),
            (
                5,
                InjectionPayload::user_prompt(
                    "Excellent work! Please write unit tests for the JWT functions.",
                ),
            ),
        ];

        for (delay, payload) in injection_sequence {
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
            println!("\n⚡ Injecting: {:?}", payload.payload_type);
            manager.inject(&session_id, payload).await?;
        }

        println!("\n\n✅ Injection sequence complete!");
        println!("The Claude session should now have received all context updates.");
        println!("\nPress Enter to stop the session...");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        manager.stop_session(&session_id).await?;
        println!("Session stopped.");
    }

    println!("\n✅ All examples completed!");

    Ok(())
}

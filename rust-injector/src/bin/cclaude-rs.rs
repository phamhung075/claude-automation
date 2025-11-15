use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;
use std::process::Command;

/// Custom Claude launcher with automatic agent role setting
#[derive(Parser)]
#[command(name = "cclaude-rs")]
#[command(about = "Launch Claude with automatic agent configuration in new terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Agent name (e.g., coding-agent, test-orchestrator-agent)
    #[arg(short, long)]
    agent: Option<String>,

    /// Working directory
    #[arg(short, long)]
    dir: Option<String>,

    /// Direct command to pass to Claude
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch Claude with specific agent and optional prompt
    Launch {
        /// Agent name (e.g., coding-agent)
        agent: String,

        /// Optional initial prompt
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        prompt: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Launch { agent, prompt }) => {
            launch_claude_with_agent(&agent, cli.dir, prompt.join(" ").as_str())?;
        }
        None => {
            // Default mode: use --agent flag or default to master-orchestrator-agent
            let agent = cli.agent.unwrap_or_else(|| "master-orchestrator-agent".to_string());
            let prompt = cli.args.join(" ");
            launch_claude_with_agent(&agent, cli.dir, &prompt)?;
        }
    }

    Ok(())
}

fn launch_claude_with_agent(agent: &str, working_dir: Option<String>, prompt: &str) -> Result<()> {
    // Determine working directory
    let working_dir = working_dir.unwrap_or_else(|| {
        env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string()
    });

    println!("🤖 Agent: {}", agent);
    println!("📁 Directory: {}", working_dir);
    if !prompt.is_empty() {
        println!("📝 Prompt: {}", prompt);
    }
    println!();

    // Generate unique session name
    let session_name = format!("cclaude-{}", agent);

    // Check if tmux is available
    let tmux_check = Command::new("tmux").arg("-V").output();
    if tmux_check.is_err() {
        anyhow::bail!("tmux is not installed. Install with: sudo apt install tmux");
    }

    println!("🚀 Creating tmux session: {}", session_name);

    // Create tmux session with Claude running
    // IMPORTANT: Use -e flag to pass CCLAUDE_AGENT environment variable INTO the tmux session
    let env_var = format!("CCLAUDE_AGENT={}", agent);

    let tmux_create = Command::new("tmux")
        .args(&[
            "new-session",
            "-d",              // Detached
            "-e", &env_var,    // Pass environment variable into session
            "-s", &session_name,
            "-c", &working_dir,
            "claude",
            "--dangerously-skip-permissions",
        ])
        .output()?;

    if !tmux_create.status.success() {
        // Session might already exist, kill it and retry
        let _ = Command::new("tmux")
            .args(&["kill-session", "-t", &session_name])
            .output();

        // Retry creation with environment variable
        let retry = Command::new("tmux")
            .args(&[
                "new-session",
                "-d",
                "-e", &env_var,    // Pass environment variable into session
                "-s", &session_name,
                "-c", &working_dir,
                "claude",
                "--dangerously-skip-permissions",
            ])
            .output()?;

        if !retry.status.success() {
            anyhow::bail!("Failed to create tmux session: {}", String::from_utf8_lossy(&retry.stderr));
        }
    }

    println!("✅ Tmux session created: {}", session_name);
    println!("🔧 Agent: {} (auto-detected via CCLAUDE_AGENT)", agent);
    println!();

    // Open new terminal and attach to session
    open_terminal_with_tmux(&session_name, agent, &working_dir)?;

    // Send initial prompt if provided (AFTER terminal opens)
    if !prompt.is_empty() {
        println!("⏳ Waiting for Claude to initialize...");
        std::thread::sleep(std::time::Duration::from_secs(8));

        println!("📝 Injecting initial prompt...");

        // Format prompt
        let formatted_prompt = if prompt.starts_with("task_id:") || prompt.starts_with("subtask_id:") {
            format!("Call {} to do {}", agent, prompt)
        } else {
            prompt.to_string()
        };

        // Send message with -l flag (literal)
        let send_result = Command::new("tmux")
            .args(&["send-keys", "-l", "-t", &session_name, &formatted_prompt])
            .output()?;

        if !send_result.status.success() {
            eprintln!("⚠️  Warning: Failed to inject message text");
        }

        // Send Enter key
        let enter_result = Command::new("tmux")
            .args(&["send-keys", "-t", &session_name, "Enter"])
            .output()?;

        if !enter_result.status.success() {
            eprintln!("⚠️  Warning: Failed to inject Enter key");
        } else {
            println!("✅ Prompt injected successfully");
        }
    }

    Ok(())
}

fn open_terminal_with_tmux(session_name: &str, agent: &str, working_dir: &str) -> Result<()> {
    // Detect platform and open appropriate terminal

    // WSL2 with Windows Terminal
    if Command::new("wt.exe").arg("--version").output().is_ok() {
        println!("🪟 Opening Windows Terminal...");

        let attach_cmd = format!("cd '{}' && tmux attach -t {}", working_dir, session_name);

        Command::new("wt.exe")
            .args(&[
                "new-tab",
                "--title",
                &format!("Claude [{}]", agent),
                "bash",
                "-c",
                &attach_cmd,
            ])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;

        println!("✅ Windows Terminal opened");
        return Ok(());
    }

    // Linux with gnome-terminal
    if Command::new("gnome-terminal").arg("--version").output().is_ok() {
        println!("🐧 Opening GNOME Terminal...");

        Command::new("gnome-terminal")
            .args(&[
                "--working-directory", working_dir,
                "--title", &format!("Claude [{}]", agent),
                "--",
                "bash", "-c",
                &format!("tmux attach -t {}; exec bash", session_name),
            ])
            .spawn()?;

        println!("✅ GNOME Terminal opened");
        return Ok(());
    }

    // macOS with Terminal.app
    if cfg!(target_os = "macos") {
        println!("🍎 Opening Terminal.app...");

        let script = format!(
            r#"tell application "Terminal"
    activate
    do script "cd '{}' && tmux attach -t {}"
end tell"#,
            working_dir, session_name
        );

        Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .spawn()?;

        println!("✅ Terminal.app opened");
        return Ok(());
    }

    // Fallback: Print attach command
    println!("⚠️  No supported terminal found");
    println!("📝 Manually attach with: tmux attach -t {}", session_name);
    println!("💡 Or install: wt.exe (WSL2) | gnome-terminal (Linux) | Terminal.app (macOS)");

    Ok(())
}

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use claude_injector::*;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "claude-inject")]
#[command(about = "CLI tool for injecting messages into Claude sessions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Spawn a new Claude session with a custom ID
    Spawn {
        /// Custom session identifier
        #[arg(short, long)]
        id: String,

        /// Initial prompt (optional)
        #[arg(short, long)]
        prompt: Option<String>,
    },

    /// Inject a message into a managed session (spawned by this tool)
    Inject {
        /// Session ID to inject into
        #[arg(short, long)]
        id: String,

        /// Message to inject (will be sent as user input)
        #[arg(short, long)]
        message: String,
    },

    /// Inject into ANY existing Claude session via terminal device (PTY)
    Pty {
        /// Session ID to inject into
        #[arg(short, long)]
        id: String,

        /// Message to inject (will be sent as user input)
        #[arg(short, long)]
        message: String,
    },

    /// List active managed sessions
    List,

    /// Stop a running session
    Stop {
        /// Session ID to stop
        #[arg(short, long)]
        id: String,
    },

    /// Find existing Claude sessions by ID
    Find {
        /// Session ID to find (optional - lists all if not provided)
        #[arg(short, long)]
        id: Option<String>,
    },

    /// Spawn Claude in a tmux session (visible + injectable)
    Tmux {
        /// Tmux session name
        #[arg(short = 'n', long)]
        name: String,

        /// Working directory for Claude
        #[arg(short = 'd', long)]
        dir: Option<String>,
    },

    /// Inject message into a tmux Claude session
    TmuxInject {
        /// Tmux session name
        #[arg(short = 'n', long)]
        name: String,

        /// Message to inject
        #[arg(short, long)]
        message: String,
    },

    /// Spawn a worker with agent type (auto-registered)
    SpawnWorker {
        /// Worker name
        #[arg(short, long)]
        name: String,

        /// Agent type (e.g., coding-agent, test-orchestrator-agent)
        #[arg(short, long)]
        agent: String,

        /// Working directory
        #[arg(short, long)]
        dir: Option<String>,

        /// Task ID (optional)
        #[arg(short, long)]
        task_id: Option<String>,

        /// Initial prompt to send after spawn
        #[arg(short = 'p', long)]
        prompt: Option<String>,
    },

    /// List all registered workers
    ListWorkers {
        /// Output format
        #[arg(short, long, default_value = "table")]
        format: String,

        /// Filter by agent type
        #[arg(long)]
        agent: Option<String>,

        /// Filter by status
        #[arg(long)]
        status: Option<String>,
    },

    /// Get worker status
    WorkerStatus {
        /// Worker name
        #[arg(short, long)]
        name: String,
    },

    /// Stop a worker
    StopWorker {
        /// Worker name
        #[arg(short, long)]
        name: String,

        /// Force kill
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Serialize, Deserialize)]
struct SessionRegistry {
    sessions: std::collections::HashMap<String, SessionInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SessionInfo {
    custom_id: String,
    claude_session_id: String,
    project_path: String,
    started_at: u64,
}

fn get_registry_path() -> PathBuf {
    let home = dirs::home_dir().expect("Cannot find home directory");
    home.join(".claude-injector-registry.json")
}

fn load_registry() -> Result<SessionRegistry> {
    let path = get_registry_path();
    if !path.exists() {
        return Ok(SessionRegistry {
            sessions: std::collections::HashMap::new(),
        });
    }

    let content = fs::read_to_string(&path)?;
    let registry: SessionRegistry = serde_json::from_str(&content)?;
    Ok(registry)
}

fn save_registry(registry: &SessionRegistry) -> Result<()> {
    let path = get_registry_path();
    let content = serde_json::to_string_pretty(registry)?;
    fs::write(&path, content)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Spawn { id, prompt } => {
            println!("🚀 Spawning Claude session with ID: {}", id);

            // Detect available sessions
            let detector = SessionDetector::new()?;
            let all_sessions = detector.get_all_sessions()?;

            if all_sessions.is_empty() {
                anyhow::bail!("No Claude sessions found. Create one first with: cd /some/project && claude");
            }

            let session = all_sessions.values().next().unwrap()[0].clone();
            println!("📁 Using base session: {}", session.project_path);

            // Start Claude process
            let manager = ClaudeProcessManager::new();

            let initial_prompt = prompt.unwrap_or_else(|| {
                "I am ready to receive injected messages.".to_string()
            });

            let claude_session_id = manager
                .start_session(session.clone(), Some(initial_prompt))
                .await
                .context("Failed to start Claude session")?;

            println!("✅ Claude process started: {}", claude_session_id);

            // Save to registry
            let mut registry = load_registry()?;
            registry.sessions.insert(
                id.clone(),
                SessionInfo {
                    custom_id: id.clone(),
                    claude_session_id: claude_session_id.clone(),
                    project_path: session.project_path,
                    started_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                },
            );
            save_registry(&registry)?;

            println!("\n💡 Session registered! Now you can inject messages:");
            println!("   claude-inject inject --id {} --message \"Your message here\"", id);
            println!("\n⏳ Session will run in background. Stop with:");
            println!("   claude-inject stop --id {}", id);

            // Keep process alive
            println!("\n🔄 Session running... Press Ctrl+C to stop");
            tokio::signal::ctrl_c().await?;

            // Cleanup
            manager.stop_session(&claude_session_id).await?;
            let mut registry = load_registry()?;
            registry.sessions.remove(&id);
            save_registry(&registry)?;

            println!("🛑 Session stopped");
        }

        Commands::Inject { id, message } => {
            println!("📤 Injecting message into MANAGED session: {}", id);

            let registry = load_registry()?;
            let session_info = registry
                .sessions
                .get(&id)
                .context(format!("Session '{}' not found. Is it running?", id))?;

            println!("📝 Message: {}", message);

            let manager = ClaudeProcessManager::new();

            let payload = InjectionPayload::user_prompt(message);

            manager
                .inject(&session_info.claude_session_id, payload)
                .await
                .context("Failed to inject message")?;

            println!("✅ Message injected successfully!");
        }

        Commands::Pty { id, message } => {
            println!("📤 Injecting into EXISTING Claude session via PTY: {}", id);
            println!("📝 Message: {}", message);
            println!();

            PtyInjector::inject_to_session(&id, &message)?;
        }

        Commands::List => {
            let registry = load_registry()?;

            if registry.sessions.is_empty() {
                println!("No active sessions");
                return Ok(());
            }

            println!("Active sessions:");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            for (id, info) in &registry.sessions {
                let age = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    - info.started_at;

                println!("\n  ID: {}", id);
                println!("  Claude Session: {}", info.claude_session_id);
                println!("  Project: {}", info.project_path);
                println!("  Running for: {}s", age);
            }

            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }

        Commands::Stop { id } => {
            println!("🛑 Stopping session: {}", id);

            let mut registry = load_registry()?;
            let session_info = registry
                .sessions
                .get(&id)
                .context(format!("Session '{}' not found", id))?
                .clone();

            let manager = ClaudeProcessManager::new();
            manager
                .stop_session(&session_info.claude_session_id)
                .await
                .context("Failed to stop session")?;

            registry.sessions.remove(&id);
            save_registry(&registry)?;

            println!("✅ Session stopped");
        }

        Commands::Find { id } => {
            println!("🔍 Finding existing Claude sessions...\n");

            let sessions = SessionMapper::map_sessions_to_processes()?;

            if sessions.is_empty() {
                println!("No running Claude sessions found");
                return Ok(());
            }

            if let Some(target_id) = id {
                // Find specific session
                if let Some(session) = sessions.iter().find(|s| s.session_id.starts_with(&target_id)) {
                    println!("✅ Found session!");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("  Session ID: {}", session.session_id);
                    println!("  Process PID: {}", session.pid);
                    println!("  Project: {}", session.project_path);

                    if let Some(ref term) = session.terminal_info {
                        println!("\n  Terminal Info:");
                        println!("    Type: {}", term.terminal_name);
                        println!("    PID: {}", term.terminal_pid);
                        println!("    Command: {}", term.terminal_cmd);

                        println!("\n💡 Injection Options:");
                        println!("  ⚠️  Direct stdin injection: NOT POSSIBLE (process not spawned by us)");
                        println!("  ✅ Terminal automation: Use tools like:");
                        println!("     - xdotool (X11): xdotool type --window <WID> \"message\"");
                        println!("     - tmux (if in tmux): tmux send-keys -t <session> \"message\" Enter");
                        println!("     - expect scripts: Automate terminal input");
                    } else {
                        println!("\n  Terminal: Unknown");
                        println!("\n⚠️  Cannot inject: Terminal information not available");
                    }
                } else {
                    println!("❌ Session '{}' not found", target_id);
                }
            } else {
                // List all sessions
                println!("Found {} running Claude session(s):", sessions.len());
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

                for session in &sessions {
                    println!("  Session ID: {}", session.session_id);
                    println!("  Process PID: {}", session.pid);
                    println!("  Project: {}", session.project_path);

                    if let Some(ref term) = session.terminal_info {
                        println!("  Terminal: {} (PID: {})", term.terminal_name, term.terminal_pid);
                    } else {
                        println!("  Terminal: Unknown");
                    }
                    println!();
                }

                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("\n💡 To find a specific session:");
                println!("   claude-inject find --id <session-id>");
            }
        }

        Commands::Tmux { name, dir } => {
            println!("🚀 Spawning Claude in tmux session: {}", name);

            if !TmuxSpawner::is_available() {
                anyhow::bail!("tmux is not installed. Install with: sudo apt install tmux");
            }

            if TmuxSpawner::session_exists(&name) {
                anyhow::bail!("Tmux session '{}' already exists", name);
            }

            let working_dir = dir.unwrap_or_else(|| {
                std::env::current_dir()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            });

            TmuxSpawner::spawn_session(&name, &working_dir)?;

            println!("✅ Claude started in tmux session!");
            println!("\n📺 To view the session, run:");
            println!("   {}", TmuxSpawner::attach_command(&name));
            println!("\n💡 To inject messages:");
            println!("   claude-inject tmux-inject --name {} --message \"Your message\"", name);
            println!("\n🛑 To stop:");
            println!("   tmux kill-session -t {}", name);
        }

        Commands::TmuxInject { name, message } => {
            println!("📤 Injecting into tmux session: {}", name);
            println!("📝 Message: {}", message);

            if !TmuxSpawner::session_exists(&name) {
                anyhow::bail!("Tmux session '{}' not found", name);
            }

            TmuxSpawner::inject_message(&name, &message)?;

            // Update message counter
            let mut registry = WorkerRegistry::load()?;
            registry.increment_messages(&name).ok();

            println!("✅ Message injected!");
            println!("\n💡 View the session with:");
            println!("   {}", TmuxSpawner::attach_command(&name));
        }

        Commands::SpawnWorker { name, agent, dir, task_id, prompt } => {
            println!("🚀 Spawning worker: {}", name);
            println!("🤖 Agent: {}", agent);

            let working_dir = dir.unwrap_or_else(|| {
                std::env::current_dir()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            });

            println!("📁 Directory: {}", working_dir);
            if let Some(ref tid) = task_id {
                println!("📋 Task ID: {}", tid);
            }

            // Spawn and register worker
            let worker = TmuxSpawner::spawn_worker(&name, &agent, &working_dir, task_id)?;

            println!("✅ Worker spawned and registered!");
            println!("\n📺 View session: tmux attach -t {}", worker.name);
            println!("📤 Inject message: claude-inject tmux-inject --name {} --message \"...\"", worker.name);

            // Wait for session to initialize
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            // Always load the specified agent first
            println!("\n🔧 Loading agent: {}...", agent);
            let load_agent_cmd = format!(
                "mcp__agenthub_http__call_agent(\"{}\")",
                agent
            );
            TmuxSpawner::inject_message(&name, &load_agent_cmd)?;

            // Wait for agent to load
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

            // Send initial prompt if provided
            if let Some(initial_prompt) = prompt {
                println!("📝 Sending initial prompt...");
                TmuxSpawner::inject_message(&name, &initial_prompt)?;

                let mut registry = WorkerRegistry::load()?;
                registry.update_status(&name, WorkerStatus::Working)?;
                println!("✅ Initial prompt sent!");
            } else {
                let mut registry = WorkerRegistry::load()?;
                registry.update_status(&name, WorkerStatus::Ready)?;
            }
        }

        Commands::ListWorkers { format, agent, status } => {
            let registry = WorkerRegistry::load()?;

            let mut workers: Vec<&WorkerInfo> = if let Some(ref agent_filter) = agent {
                registry.list_by_agent(agent_filter)
            } else {
                registry.list_all()
            };

            if let Some(ref status_filter) = status {
                let status_enum = match status_filter.as_str() {
                    "starting" => WorkerStatus::Starting,
                    "ready" => WorkerStatus::Ready,
                    "working" => WorkerStatus::Working,
                    "idle" => WorkerStatus::Idle,
                    "error" => WorkerStatus::Error,
                    "stopped" => WorkerStatus::Stopped,
                    _ => anyhow::bail!("Invalid status: {}", status_filter),
                };
                workers.retain(|w| w.status == status_enum);
            }

            if workers.is_empty() {
                println!("No workers found");
                return Ok(());
            }

            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&workers)?);
            } else {
                // Table format
                println!("\n{:<20} {:<20} {:<15} {:<10} {:<8}", "NAME", "AGENT", "TASK_ID", "STATUS", "MESSAGES");
                println!("{}", "=".repeat(80));

                for worker in &workers {
                    let task = worker.task_id.as_deref().unwrap_or("-");
                    let task_short = if task.len() > 12 {
                        format!("{}...", &task[..9])
                    } else {
                        task.to_string()
                    };

                    println!(
                        "{:<20} {:<20} {:<15} {:<10} {:<8}",
                        worker.name,
                        worker.agent_type,
                        task_short,
                        worker.status,
                        worker.messages_sent
                    );
                }
                println!("\nTotal: {} worker(s)\n", workers.len());
            }
        }

        Commands::WorkerStatus { name } => {
            let registry = WorkerRegistry::load()?;

            match registry.get(&name) {
                Some(worker) => {
                    println!("\n🔍 Worker Status: {}", name);
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("Agent:        {}", worker.agent_type);
                    println!("Status:       {}", worker.status);
                    println!("Task ID:      {}", worker.task_id.as_deref().unwrap_or("-"));
                    println!("Directory:    {}", worker.working_dir);
                    println!("Messages:     {}", worker.messages_sent);
                    println!("Tmux Session: {}", worker.tmux_session);

                    let uptime = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        - worker.spawned_at;
                    println!("Uptime:       {}s", uptime);

                    let session_exists = TmuxSpawner::session_exists(&worker.tmux_session);
                    println!("Running:      {}", if session_exists { "yes" } else { "no" });

                    println!("\n💡 Attach: tmux attach -t {}", worker.tmux_session);
                }
                None => {
                    println!("❌ Worker '{}' not found in registry", name);
                }
            }
        }

        Commands::StopWorker { name, force } => {
            println!("🛑 Stopping worker: {}", name);

            let mut registry = WorkerRegistry::load()?;

            if !registry.exists(&name) {
                println!("⚠️  Worker not found in registry");
            }

            if TmuxSpawner::session_exists(&name) {
                if force {
                    TmuxSpawner::kill_session(&name)?;
                    println!("✅ Worker killed");
                } else {
                    TmuxSpawner::send_interrupt(&name)?;
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    TmuxSpawner::kill_session(&name)?;
                    println!("✅ Worker stopped");
                }
            }

            registry.update_status(&name, WorkerStatus::Stopped)?;
            registry.unregister(&name)?;

            println!("✅ Worker unregistered");
        }
    }

    Ok(())
}

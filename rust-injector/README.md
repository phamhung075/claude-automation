# Claude Session Injector - Rust Implementation

**Automatic real-time context injection into running Claude Code sessions**

---

## 🎯 Purpose

This Rust library enables **programmatic interaction** with running Claude Code sessions by:

1. **Detecting Claude sessions** from `~/.claude/projects/`
2. **Finding running Claude processes** on your system
3. **Injecting context/warnings/blocks** into active sessions via stdin pipes
4. **Preparing automatic coordination** between MCP backend and Claude sessions

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│ Your Application (MCP Backend, Automation System)      │
└────────────────┬────────────────────────────────────────┘
                 │ Uses library
┌────────────────▼────────────────────────────────────────┐
│ Claude Session Injector (Rust Library)                  │
│                                                          │
│ ┌────────────────┐  ┌──────────────┐  ┌─────────────┐  │
│ │ SessionDetector│  │ProcessManager│  │PayloadSystem│  │
│ │                │  │              │  │             │  │
│ │ • List sessions│  │• Start claude│  │• Context    │  │
│ │ • Read metadata│  │• Inject stdin│  │• Warnings   │  │
│ │ • Get projects │  │• Monitor PID │  │• Blocks     │  │
│ └────────────────┘  └──────────────┘  └─────────────┘  │
└────────────────┬────────────────────────────────────────┘
                 │ Spawns & injects
┌────────────────▼────────────────────────────────────────┐
│ Running Claude Code Sessions                            │
│ • Receives injected context in real-time               │
│ • Continues work with new information                   │
│ • No polling needed - direct stdin injection!          │
└─────────────────────────────────────────────────────────┘
```

---

## 🚀 Key Features

### 1. Session Detection
```rust
use claude_injector::SessionDetector;

let detector = SessionDetector::new()?;

// List all projects
let projects = detector.list_projects()?;

// Get sessions for a project
let sessions = detector.get_project_sessions("my-project-id")?;

// Get ALL sessions across all projects
let all_sessions = detector.get_all_sessions()?;
```

**What it does:**
- Reads `~/.claude/projects/<project-id>/<session-id>.jsonl`
- Extracts session metadata (project path, first message, model, timestamps)
- Provides structured access to Claude's session history

---

### 2. Process Management
```rust
use claude_injector::ClaudeProcessManager;

let manager = ClaudeProcessManager::new();

// Start a new Claude session with stdin pipe
let session_id = manager.start_session(
    session,
    Some("Initial prompt".to_string())
).await?;

// Inject payload into running session
let payload = InjectionPayload::context("New information!");
manager.inject(&session_id, payload).await?;

// Broadcast to ALL active sessions
manager.broadcast(payload).await?;
```

**What it does:**
- Spawns `claude` CLI with `Stdio::piped()` for stdin/stdout/stderr
- Keeps stdin pipe open for injection
- Tracks process PIDs
- Manages process lifecycle

---

### 3. Structured Payloads
```rust
use claude_injector::payload::*;

// Context update
let payload = InjectionPayload::context("Task dependency completed");

// Warning
let payload = InjectionPayload::warning("Memory usage high");

// Blocker (requires attention)
let payload = InjectionPayload::block("Tests failing - fix before continuing");

// Progress update
let payload = InjectionPayload::progress(75, "Almost done");

// Completion notification
let payload = InjectionPayload::completion(
    "Feature complete".to_string(),
    metadata_map
);

// Preset payloads for common scenarios
let payload = presets::dependency_completed(
    "Design schema",
    "Created 5 tables",
    vec!["Use UUIDs", "Add timestamps"]
);
```

**What it does:**
- Formats messages for Claude to understand
- Adds visual indicators (📋, ⚠️, 🚨, ✅, etc.)
- Includes metadata for structured communication
- Provides presets for common coordination patterns

---

### 4. Process Detection
```rust
use claude_injector::ProcessDetector;

// Find running Claude processes
let processes = ProcessDetector::find_running_claude_processes()?;

for process in processes {
    println!("PID: {}, Command: {}", process.pid, process.command);

    // Get working directory (Linux only)
    if let Some(cwd) = ProcessDetector::get_process_cwd(process.pid) {
        println!("Working dir: {}", cwd);
    }

    // Check if still running
    if ProcessDetector::is_process_running(process.pid) {
        println!("Process is active");
    }
}
```

**What it does:**
- Finds Claude processes using system commands (`ps`, `tasklist`)
- Cross-platform (Linux, macOS, Windows)
- Provides process metadata

---

## 📦 Installation & Building

### Prerequisites
```bash
# Rust toolchain (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Claude Code CLI
# https://code.claude.com/docs/en/getting-started/installation
```

### Build the Project
```bash
cd claude-automation/rust-injector

# Build library and binary
cargo build --release

# Run tests
cargo test

# Run the examples
cargo run --release
```

### Build Output
```
target/release/
├── claude-injector           # CLI executable
└── libclaude_injector.rlib   # Library
```

---

## 🧪 Usage Examples

### Example 1: List All Sessions
```rust
use claude_injector::SessionDetector;

#[tokio::main]
async fn main() -> Result<()> {
    let detector = SessionDetector::new()?;
    let all_sessions = detector.get_all_sessions()?;

    for (project_id, sessions) in all_sessions {
        println!("Project: {}", project_id);
        for session in sessions {
            println!("  Session: {}", session.session_id);
            println!("  Path: {}", session.project_path);
        }
    }

    Ok(())
}
```

### Example 2: Start Session and Inject Context
```rust
use claude_injector::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Get a session
    let detector = SessionDetector::new()?;
    let sessions = detector.get_project_sessions("my-project")?;
    let session = sessions[0].clone();

    // Start Claude with stdin pipe
    let manager = ClaudeProcessManager::new();
    let session_id = manager.start_session(
        session,
        Some("Ready to receive real-time context!".to_string())
    ).await?;

    // Wait for initialization
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Inject context
    let payload = InjectionPayload::context(
        "Task 'Design Schema' completed. You can now start implementation."
    );
    manager.inject(&session_id, payload).await?;

    // Inject completion notification
    let payload = presets::dependency_completed(
        "Database schema",
        "Created 5 tables with indexes",
        vec!["Use UUID for IDs", "Add created_at/updated_at"]
    );
    manager.inject(&session_id, payload).await?;

    // Wait and stop
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    manager.stop_session(&session_id).await?;

    Ok(())
}
```

### Example 3: MCP Event → Claude Injection
```rust
// Simulate MCP WebSocket event
async fn handle_subtask_completed(event: SubtaskCompletedEvent) -> Result<()> {
    let manager = ClaudeProcessManager::new();

    // Create payload from MCP event
    let payload = presets::dependency_completed(
        &event.title,
        &event.completion_summary,
        event.insights_found
    );

    // Inject into ALL active sessions assigned to this task
    for session_id in get_assigned_sessions(&event.subtask_id) {
        manager.inject(&session_id, payload.clone()).await?;
    }

    Ok(())
}
```

### Example 4: Automatic Progress Updates
```rust
async fn update_progress(session_id: &str, percentage: u8, message: &str) -> Result<()> {
    let manager = ClaudeProcessManager::new();

    let payload = InjectionPayload::progress(percentage, message);
    manager.inject(session_id, payload).await?;

    Ok(())
}

// Usage:
update_progress(&session_id, 25, "Started implementation").await?;
update_progress(&session_id, 50, "Core logic complete").await?;
update_progress(&session_id, 75, "Tests passing").await?;
update_progress(&session_id, 100, "Feature complete").await?;
```

---

## 🔗 Integration with Autonomous System

### Scenario: Event-Driven Coordination

```rust
// WebSocket event handler in agenthub backend
async fn on_subtask_completed(event: SubtaskCompletedEvent) {
    // 1. Find dependent tasks
    let dependents = find_dependent_subtasks(&event.subtask_id);

    // 2. For each dependent with active Claude session
    for dependent in dependents {
        if let Some(session_id) = get_active_claude_session(&dependent.assignees) {
            // 3. Inject completion context
            let payload = presets::dependency_completed(
                &event.title,
                &event.completion_summary,
                event.insights_found.clone()
            );

            claude_manager.inject(&session_id, payload).await?;
        }
    }
}
```

**Benefits:**
- **Zero polling** - Real-time via stdin injection
- **Instant coordination** - Claude gets context as soon as dependencies complete
- **Automatic workflow** - No human intervention needed
- **Scalable** - Works with N concurrent sessions

---

## 🎓 Key Insights

`★ Insight ─────────────────────────────────────`
**Why Rust for this?**
- **Process management**: Tokio's async process handling is perfect for managing multiple Claude sessions
- **stdin/stdout pipes**: Low-level control over process I/O
- **Performance**: Zero-overhead abstraction for high-frequency injections
- **Safety**: Type system ensures correct payload construction

**How injection works:**
1. Spawn `claude` with `Stdio::piped()`
2. Keep `stdin` handle in `ProcessHandle`
3. Write formatted payload to `stdin` via `AsyncWriteExt`
4. Claude reads from stdin as if user typed the message
5. **Result**: Instant context delivery with zero API calls!

**Comparison with WebSocket approach:**
- WebSocket: Backend → Frontend → User sees update
- stdin injection: Backend → **Directly into Claude session** (no UI needed!)
- Use case: **Autonomous agents** that don't need human monitoring
`─────────────────────────────────────────────────`

---

## 📊 Architecture Comparison

| Approach | How it Works | Latency | Use Case |
|----------|-------------|---------|----------|
| **WebSocket (Node.js)** | Backend → WS → Frontend → User → Claude | ~200ms | Human-in-the-loop monitoring |
| **stdin Injection (Rust)** | Backend → Directly to Claude stdin | <10ms | Fully autonomous coordination |
| **File Polling (Bash)** | Write file → Claude polls | 2-5s | Simple, but slow |

**When to use Rust injector:**
- Fully autonomous multi-agent workflows
- Real-time coordination without UI
- High-frequency context updates
- Production automation pipelines

---

## 🐛 Troubleshooting

### "Failed to spawn claude process"
**Solution**: Ensure `claude` CLI is in PATH
```bash
which claude
# Should output: /usr/local/bin/claude or similar
```

### "Session stdin not available"
**Solution**: Ensure `Stdio::piped()` is set when spawning
```rust
cmd.stdin(Stdio::piped())  // Required!
```

### "Process not found in registry"
**Solution**: Session may have exited. Check with:
```rust
manager.is_session_active(&session_id).await
```

---

## 🚀 Next Steps

1. **Test the CLI**: `cargo run --release`
2. **Integration Example**: See `examples/mcp_integration.rs` (TODO)
3. **Production Deployment**: Build release binary and integrate with your backend
4. **Extend Payloads**: Add custom payload types for your use case

---

## 📝 License

[Your License Here]

---

**This Rust library is the missing piece for fully autonomous Claude Code coordination!** 🎉

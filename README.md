# Claude Automation System

**High-Performance Multi-Agent Workflow Orchestration with Rust-Based Launchers**

---

## 🎯 What is This?

A **Rust-based launcher system** for orchestrating Claude AI agents with tmux sessions, message injection, and worker management. Built for performance, reliability, and seamless integration with the agenthub MCP backend.

### Key Features

✅ **cclaude-rs** - Interactive launcher that opens new terminals with tmux
✅ **claude-inject** - Message injection and background worker spawning
✅ **Automatic agent detection** - Hooks auto-load agents from status line
✅ **Worker registry** - Track and manage multiple background workers
✅ **Tmux integration** - Visible sessions with message injection capability
✅ **Cross-platform** - WSL2 (Windows Terminal), Linux (gnome-terminal), macOS (Terminal.app)

---

## 🚀 Quick Start

### Prerequisites

```bash
# 1. Install Rust (for building from source)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Install Claude Code CLI
# https://docs.claude.com/en/docs/claude-code/installation

# 3. Install tmux
sudo apt-get install tmux  # Ubuntu/Debian
brew install tmux          # macOS
```

### Build the Tools

```bash
cd rust-injector

# Build both cclaude-rs and claude-inject
cargo build --release

# Binaries will be in:
# ./target/release/cclaude-rs
# ./target/release/claude-inject
```

### Test the System

```bash
# Launch an interactive coding session
./target/release/cclaude-rs --agent coding-agent "Implement authentication"

# Spawn a background worker
./target/release/claude-inject spawn-worker \
    --name worker-1 \
    --agent coding-agent \
    --dir /path/to/project \
    --prompt "Fix bug in auth.js"

# Check worker status
./target/release/claude-inject list-workers
./target/release/claude-inject worker-status --name worker-1

# Inject a message into running worker
./target/release/claude-inject tmux-inject \
    --name worker-1 \
    --message "Add error handling"
```

---

## 📖 Core Tools

### 1. cclaude-rs (Interactive Launcher)

**Purpose**: Launch Claude in a new terminal window with tmux session

**Features**:
- Opens new terminal (Windows Terminal on WSL2, gnome-terminal on Linux, Terminal.app on macOS)
- Creates named tmux session: `cclaude-{agent-name}`
- Hooks automatically load specified agent
- Visible session for real-time feedback
- Supports custom working directory

**Usage**:
```bash
cclaude-rs --agent <agent-name> [--dir <path>] "<prompt>"

# Examples:
cclaude-rs --agent coding-agent "Implement JWT auth"
cclaude-rs --agent test-orchestrator-agent "Run unit tests"
cclaude-rs --agent coding-agent --dir /home/user/project "Fix bug"
```

**When to Use**:
- Interactive development work
- Debugging and testing
- Real-time feedback needed
- Manual intervention expected

---

### 2. claude-inject (Worker & Message Management)

**Purpose**: Spawn background workers and inject messages into tmux sessions

#### 2a. Spawn Worker

**Features**:
- Creates detached tmux session
- Registers worker in worker registry
- Auto-loads specified agent via hooks
- Supports task ID tracking
- Background execution (fire-and-forget)

**Usage**:
```bash
claude-inject spawn-worker \
    --name <worker-name> \
    --agent <agent-name> \
    --dir <working-directory> \
    [--task-id <mcp-task-id>] \
    [--prompt "<initial-prompt>"]

# Example:
claude-inject spawn-worker \
    --name worker-auth \
    --agent coding-agent \
    --dir /home/user/project \
    --task-id task-abc-123 \
    --prompt "Implement OAuth flow"
```

**When to Use**:
- Long-running tasks
- Background automation
- Parallel work (multiple workers)
- Fire-and-forget workflows

#### 2b. Worker Management

```bash
# List all workers
claude-inject list-workers
claude-inject list-workers --format table
claude-inject list-workers --agent coding-agent

# Get worker status
claude-inject worker-status --name worker-auth

# Stop a worker
claude-inject stop-worker --name worker-auth
claude-inject stop-worker --name worker-auth --force
```

#### 2c. Message Injection

```bash
# Inject message into running worker
claude-inject tmux-inject \
    --name worker-auth \
    --message "Add rate limiting to API"

# Works with ANY tmux session (not just workers)
claude-inject tmux-inject \
    --name cclaude-coding-agent \
    --message "Add tests for this function"
```

---

## 🏗️ Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────────┐
│                  User / Orchestrator                    │
└─────────────────────────────────────────────────────────┘
                         │
         ┌───────────────┴───────────────┐
         │                               │
    ┌────▼──────┐                  ┌────▼──────────┐
    │ cclaude-rs│                  │ claude-inject │
    │(Interactive)                 │(Background)   │
    └────┬──────┘                  └────┬──────────┘
         │                               │
         │ Sets agent env var            │ Sets agent env var
         │ Opens new terminal            │ Creates detached tmux
         │                               │ Registers in worker registry
         │                               │
         └───────────────┬───────────────┘
                         │
                    ┌────▼─────────┐
                    │ Tmux Session │
                    │ Named:       │
                    │ cclaude-{agent} or worker-{name}
                    └────┬─────────┘
                         │
                    ┌────▼─────────┐
                    │ Claude Code  │
                    │ CLI Running  │
                    └────┬─────────┘
                         │
                    ┌────▼─────────┐
                    │ Session Hook │
                    │ Detects agent│
                    │ from env/status
                    └────┬─────────┘
                         │
                    ┌────▼─────────┐
                    │ call_agent() │
                    │ Loads agent  │
                    │ instructions │
                    └────┬─────────┘
                         │
                    ┌────▼─────────┐
                    │ Agent Works  │
                    │ as specialist│
                    └──────────────┘
```

### How Agent Loading Works

1. **Launch**: `cclaude-rs --agent coding-agent` or `claude-inject spawn-worker --agent coding-agent`
2. **Environment**: Launcher sets agent type (infrastructure detail)
3. **Tmux Session**: Created with name `cclaude-coding-agent` or custom worker name
4. **Claude Starts**: CLI runs in tmux session
5. **Hooks Execute**: `.claude/hooks/session_start.py` runs automatically
6. **Status Line**: Shows `🤖 Agent: coding-agent`
7. **Agent Loads**: Claude calls `mcp__agenthub_http__call_agent("coding-agent")`
8. **Work Begins**: Agent receives instructions and starts working

---

## 🔄 Workflows

### Interactive Development (cclaude-rs)

```bash
# 1. Launch agent in new terminal
cclaude-rs --agent coding-agent "Implement feature X"

# 2. New terminal opens, Claude starts as coding-agent
# 3. Work interactively with visible output
# 4. Close terminal when done
```

### Background Automation (spawn-worker)

```bash
# 1. Create MCP task first (using parent project tools)
# task_id = create_mcp_task(...)

# 2. Spawn worker with task ID
claude-inject spawn-worker \
    --name worker-feature-x \
    --agent coding-agent \
    --task-id $task_id \
    --prompt "Implement feature X from task"

# 3. Monitor worker status
claude-inject worker-status --name worker-feature-x

# 4. Inject additional instructions if needed
claude-inject tmux-inject \
    --name worker-feature-x \
    --message "Also add error handling"

# 5. Check completion or stop worker
claude-inject stop-worker --name worker-feature-x
```

### Parallel Workers

```bash
# Spawn multiple workers for parallel execution
claude-inject spawn-worker --name worker-frontend --agent coding-agent --prompt "Build UI"
claude-inject spawn-worker --name worker-backend --agent coding-agent --prompt "Build API"
claude-inject spawn-worker --name worker-tests --agent test-orchestrator-agent --prompt "Write tests"

# Monitor all
claude-inject list-workers

# Each works independently in parallel
```

---

## 🔧 Configuration

### Environment Variables

```bash
# Set default working directory
export CLAUDE_WORK_DIR=/path/to/project

# Set MCP backend URL (if using task management)
export MCP_API_URL=http://localhost:8000

# Set git branch ID (for MCP task creation)
export GIT_BRANCH_ID=branch-uuid-here
```

### Tmux Session Naming

- **cclaude-rs**: `cclaude-{agent-name}`
  - Example: `cclaude-coding-agent`
- **spawn-worker**: `{worker-name}`
  - Example: `worker-feature-x`

### Terminal Detection

cclaude-rs automatically detects platform and uses:
- **WSL2**: Windows Terminal (`wt.exe`)
- **Linux**: GNOME Terminal (`gnome-terminal`)
- **macOS**: Terminal.app (`open -a Terminal`)

---

## 📝 Best Practices

### When to Use cclaude-rs

✅ Interactive development
✅ Debugging and testing
✅ Quick prototyping
✅ Tasks requiring manual intervention
✅ Real-time feedback needed

### When to Use spawn-worker

✅ Long-running tasks (hours)
✅ Background automation
✅ Parallel execution (multiple tasks)
✅ Fire-and-forget workflows
✅ No user interaction needed

### Worker Management

- **Name workers descriptively**: `worker-auth-implementation` not `worker-1`
- **Track task IDs**: Link workers to MCP tasks for context
- **Monitor regularly**: Use `list-workers` and `worker-status`
- **Clean up**: Stop completed workers to free resources

---

## 🧪 Testing

```bash
# Test cclaude-rs
./target/release/cclaude-rs --agent coding-agent "echo 'test'"

# Test spawn-worker
./target/release/claude-inject spawn-worker \
    --name test-worker \
    --agent coding-agent \
    --prompt "echo 'background test'"

# Verify worker created
./target/release/claude-inject list-workers

# Check tmux session exists
tmux ls | grep test-worker

# Clean up
./target/release/claude-inject stop-worker --name test-worker
```

---

## 📚 Additional Documentation

- **WebSocket Coordinator**: See `docs/websocket-comparison.md` and `docs/websocket-auto-input-architecture.md`

---

## 🐛 Troubleshooting

### Worker Not Starting

```bash
# Check tmux sessions
tmux ls

# Attach to worker session to see output
tmux attach -t worker-name

# Check worker registry
cat ~/.claude-workers/registry.json
```

### Agent Not Loading

```bash
# Verify hooks are present
ls -la .claude/hooks/session_start.py

# Check status line in running session
# Should show: 🤖 Agent: <agent-name>
```

### Terminal Not Opening (cclaude-rs)

```bash
# Check platform detection
uname -a

# WSL2: Verify wt.exe available
which wt.exe

# Linux: Verify gnome-terminal available
which gnome-terminal

# macOS: Terminal.app should be built-in
```

---

## 🤝 Contributing

This is part of the agenthub project. See parent project for contribution guidelines.

---

## 📄 License

See parent project LICENSE file.

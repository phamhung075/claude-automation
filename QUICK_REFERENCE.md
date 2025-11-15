# Claude Automation - Quick Reference

**Rust-Based Launcher System for Claude AI Agents**

---

## 🚀 Quick Commands

### Build Tools

```bash
cd rust-injector
cargo build --release

# Binaries location:
# ./target/release/cclaude-rs
# ./target/release/claude-inject
```

---

## 📖 cclaude-rs (Interactive Launcher)

### Basic Usage
```bash
cclaude-rs --agent <agent-name> "<prompt>"
```

### Common Agents
```bash
# Coding work
cclaude-rs --agent coding-agent "Implement authentication"

# Testing
cclaude-rs --agent test-orchestrator-agent "Run unit tests"

# Documentation
cclaude-rs --agent documentation-agent "Update API docs"

# Debugging
cclaude-rs --agent debugger-agent "Fix crash in login"

# Security audit
cclaude-rs --agent security-auditor-agent "Review auth code"
```

### With Custom Directory
```bash
cclaude-rs --agent coding-agent --dir /home/user/project "Fix bug"
```

### Features
- Opens new terminal window
- Creates tmux session: `cclaude-{agent}`
- Hooks auto-load agent
- Visible real-time output

---

## 🔧 claude-inject (Worker Management)

### Spawn Background Worker
```bash
claude-inject spawn-worker \
    --name <worker-name> \
    --agent <agent-name> \
    --dir <working-dir> \
    [--task-id <mcp-task-id>] \
    [--prompt "<initial-prompt>"]
```

### List Workers
```bash
# All workers
claude-inject list-workers

# Table format
claude-inject list-workers --format table

# Filter by agent
claude-inject list-workers --agent coding-agent

# Filter by status
claude-inject list-workers --status running
```

### Worker Status
```bash
claude-inject worker-status --name <worker-name>
```

### Stop Worker
```bash
# Graceful stop
claude-inject stop-worker --name <worker-name>

# Force kill
claude-inject stop-worker --name <worker-name> --force
```

### Inject Message
```bash
claude-inject tmux-inject \
    --name <worker-name> \
    --message "<message-to-send>"
```

---

## 💡 Common Workflows

### Interactive Development
```bash
# Launch and work interactively
cclaude-rs --agent coding-agent "Implement feature X"

# Terminal opens, work with real-time feedback
# Close terminal when done
```

### Background Task
```bash
# Spawn worker for long-running task
claude-inject spawn-worker \
    --name worker-feature-x \
    --agent coding-agent \
    --dir /home/user/project \
    --prompt "Implement feature X"

# Check status
claude-inject worker-status --name worker-feature-x

# Inject additional instructions
claude-inject tmux-inject \
    --name worker-feature-x \
    --message "Also add error handling"

# Stop when done
claude-inject stop-worker --name worker-feature-x
```

### Parallel Execution
```bash
# Spawn multiple workers
claude-inject spawn-worker --name w-frontend --agent coding-agent --prompt "Build UI"
claude-inject spawn-worker --name w-backend --agent coding-agent --prompt "Build API"
claude-inject spawn-worker --name w-tests --agent test-orchestrator-agent --prompt "Write tests"

# Monitor all
claude-inject list-workers

# Each works independently in parallel
```

---

## 🎯 Available Agents

```bash
# Development
coding-agent
test-orchestrator-agent
debugger-agent

# Architecture & Design
system-architect-agent
design-system-agent
ui-specialist-agent

# Quality & Security
security-auditor-agent
code-reviewer-agent
performance-load-tester-agent

# Operations
devops-agent
documentation-agent
task-planning-agent

# Specialized
ml-specialist-agent
deep-research-agent
analytics-setup-agent

# 42 total specialized agents available
```

---

## 🔍 Troubleshooting

### Check Running Sessions
```bash
# List all tmux sessions
tmux ls

# Attach to session to see output
tmux attach -t cclaude-coding-agent
tmux attach -t worker-name
```

### Verify Agent Loaded
```bash
# In running session, status line should show:
# 🤖 Agent: <agent-name>
```

### Worker Registry
```bash
# Check worker registry file
cat ~/.claude-workers/registry.json

# Or use list-workers
claude-inject list-workers
```

### Clean Up Orphaned Sessions
```bash
# List all tmux sessions
tmux ls

# Kill specific session
tmux kill-session -t session-name

# Kill all Claude sessions
tmux ls | grep cclaude | cut -d: -f1 | xargs -I {} tmux kill-session -t {}
```

---

## 📝 Best Practices

### Interactive (cclaude-rs)
✅ Use for debugging and testing
✅ Use when manual intervention needed
✅ Use for quick prototyping
❌ Don't use for long-running automation

### Background (spawn-worker)
✅ Use for long-running tasks (hours)
✅ Use for parallel execution
✅ Use for fire-and-forget workflows
✅ Name workers descriptively
✅ Link to MCP task IDs
✅ Clean up completed workers

### Worker Naming
```bash
# Good names
worker-auth-implementation
worker-test-suite-refactor
worker-api-documentation

# Bad names
worker-1
w1
temp
```

---

## 🌐 Environment Variables

```bash
# Default working directory
export CLAUDE_WORK_DIR=/path/to/project

# MCP backend URL
export MCP_API_URL=http://localhost:8000

# Git branch ID for MCP tasks
export GIT_BRANCH_ID=branch-uuid
```

---

## 📚 More Information

- **Full Documentation**: See `README.md`
- **WebSocket Coordinator**: See `docs/websocket-comparison.md` and `src/websocket-coordinator/`

---

## ⚡ One-Liners

```bash
# Quick test
./target/release/cclaude-rs --agent coding-agent "echo test"

# Spawn worker with task ID
./target/release/claude-inject spawn-worker --name test --agent coding-agent --task-id $TASK_ID --prompt "work"

# Monitor all workers
watch -n 5 './target/release/claude-inject list-workers'

# Stop all workers
./target/release/claude-inject list-workers | tail -n +2 | awk '{print $1}' | xargs -I {} ./target/release/claude-inject stop-worker --name {}
```

# Claude Automation - Quick Reference

**Essential commands and patterns for autonomous workflows**

---

## 🚀 Quick Start

```bash
# Test system
./tests/test_autonomous_system.sh

# Start workflow
./scripts/start_autonomous_workflow.sh "project-id" "branch" "goal"

# Demo communication
./scripts/demo_agent_communication.sh
```

---

## 📝 Common Commands

### Start Autonomous Workflow

```bash
./scripts/start_autonomous_workflow.sh \
    "project-uuid-123" \
    "feature/authentication" \
    "Build complete JWT authentication with tests and security audit"
```

### Monitor Progress

```bash
# Watch files
watch -n 2 'ls -lh /tmp/agenthub_autonomous/'

# View logs
tail -f /tmp/agenthub_autonomous/agent_raw_output.log

# Check shared knowledge
jq . /tmp/agenthub_autonomous/shared_knowledge.json

# Check MCP tasks
curl -s http://localhost:8000/api/manage_task \
     -d '{"action":"list","git_branch_id":"branch-id"}' | jq
```

### Shared Knowledge Operations

```bash
# Add discovery
./scripts/shared_knowledge_manager.sh add-discovery "agent" "discovery text"

# Add warning
./scripts/shared_knowledge_manager.sh add-warning "agent" "warning" "high"

# Send message
./scripts/shared_knowledge_manager.sh add-message "from-agent" "to-agent" "message"

# Get all knowledge
./scripts/shared_knowledge_manager.sh get

# Get messages for agent
./scripts/shared_knowledge_manager.sh get-messages "agent-name"
```

---

## 🎯 Workflow Patterns

### Pattern 1: Simple Feature

```bash
./scripts/start_autonomous_workflow.sh \
    "$PROJECT_ID" \
    "feature/simple-calc" \
    "Create calculator with add, subtract, multiply, divide. Include unit tests."
```

### Pattern 2: Bug Fix

```bash
./scripts/start_autonomous_workflow.sh \
    "$PROJECT_ID" \
    "fix/token-expiry" \
    "Fix JWT token expiring 5 seconds too early. Root cause in auth/jwt.py:23"
```

### Pattern 3: Refactoring

```bash
./scripts/start_autonomous_workflow.sh \
    "$PROJECT_ID" \
    "refactor/auth-module" \
    "Refactor authentication module to follow SOLID principles. Maintain all existing functionality."
```

### Pattern 4: Testing

```bash
./scripts/start_autonomous_workflow.sh \
    "$PROJECT_ID" \
    "test/increase-coverage" \
    "Increase test coverage from 70% to 90% for authentication module"
```

---

## 🔧 Troubleshooting

### Orchestrator stops immediately

```bash
# Check tasks exist
curl -s http://localhost:8000/api/manage_task \
     -d '{"action":"list","git_branch_id":"branch-id"}' | jq '.tasks | length'
```

### Agent not writing result

```bash
# Check agent prompt
cat /tmp/agenthub_autonomous/prompts/coding-agent.txt

# Verify Bash tool instruction present
```

### Workflow won't complete

```bash
# Check missing flags
ls -la /tmp/agenthub_autonomous/*.flag

# Create missing manually
touch /tmp/agenthub_autonomous/tests_passed.flag
```

### View detailed logs

```bash
# Agent execution log
tail -f /tmp/agenthub_autonomous/agent_raw_output.log

# With verbose
# Edit autonomous_orchestrator.sh
# Add --verbose to claude -p calls
```

---

## 📊 File Locations

```
/tmp/agenthub_autonomous/
├── shared_knowledge.json          # Agent communication
├── current_task.json              # Task being executed
├── task_result_*.json             # Agent results
├── tests_passed.flag              # Completion condition
├── review_approved.flag           # Completion condition
├── security_passed.flag           # Completion condition
├── workflow_complete.flag         # Final stop signal
├── human_intervention_needed.flag # Need human help
└── prompts/                       # Agent prompt templates
```

---

## 🤖 Agent Types

| Agent | Purpose | When to Use |
|-------|---------|-------------|
| `coding-agent` | Write code | Feature implementation |
| `test-orchestrator-agent` | Write/run tests | Testing phase |
| `debugger-agent` | Fix bugs | Test failures |
| `code-reviewer-agent` | Review quality | Before completion |
| `security-auditor-agent` | Security audit | Security checks |
| `system-architect-agent` | Design architecture | Planning phase |
| `documentation-agent` | Write docs | Documentation needs |

---

## 🛑 Stop Conditions

Workflow stops when **ALL** are true:

✅ All MCP tasks `status = "done"`
✅ File exists: `tests_passed.flag`
✅ File exists: `review_approved.flag`
✅ File exists: `security_passed.flag`

---

## 💬 Agent Communication

### Agent writes discovery

```bash
/scripts/shared_knowledge_manager.sh \
    add-discovery "coding-agent" \
    "JWT works best with HS256 algorithm for symmetric signing"
```

### Agent sends message

```bash
/scripts/shared_knowledge_manager.sh \
    add-message "coding-agent" "test-agent" \
    "Please test the generate_token() function with expired tokens"
```

### Agent reads knowledge

```bash
cat /tmp/agenthub_autonomous/shared_knowledge.json | \
    jq '.discoveries[] | select(.agent == "coding-agent")'
```

---

## 🔄 Human Intervention

### When agent blocks

```bash
# System creates flag
/tmp/agenthub_autonomous/human_intervention_needed.flag

# View reason
cat /tmp/agenthub_autonomous/blocker_details.txt

# Fix issue, then remove flag
rm /tmp/agenthub_autonomous/human_intervention_needed.flag

# Workflow resumes!
```

---

## 📈 Performance Tips

1. **Run in background**:
   ```bash
   nohup ./scripts/start_autonomous_workflow.sh ... > workflow.log 2>&1 &
   ```

2. **Multiple workflows parallel**:
   ```bash
   ./scripts/start_autonomous_workflow.sh ... &
   ./scripts/start_autonomous_workflow.sh ... &
   ```

3. **Reduce sleep interval**:
   ```bash
   # Edit autonomous_orchestrator.sh
   SLEEP_INTERVAL=0.5  # Instead of 2
   ```

---

## 🔗 Integration

### With CI/CD

```yaml
- name: Run autonomous workflow
  run: |
    ./claude-automation/scripts/start_autonomous_workflow.sh \
      "$PROJECT_ID" \
      "auto/${{ github.run_number }}" \
      "$GOAL"
```

### With Git Hooks

```bash
# .git/hooks/post-commit
#!/bin/bash
./claude-automation/scripts/start_autonomous_workflow.sh \
    "$PROJECT_ID" \
    "$(git rev-parse --abbrev-ref HEAD)" \
    "Review and test recent changes"
```

---

## 📚 Documentation

- **[README.md](README.md)**: Overview and installation
- **[docs/architecture.md](docs/architecture.md)**: System architecture
- **[docs/usage-guide.md](docs/usage-guide.md)**: Complete usage guide
- **[docs/agent-communication.md](docs/agent-communication.md)**: Agent communication
- **[SETUP_AS_SUBMODULE.md](SETUP_AS_SUBMODULE.md)**: Git submodule setup

---

## 🆘 Getting Help

- Check logs: `/tmp/agenthub_autonomous/agent_raw_output.log`
- Test system: `./tests/test_autonomous_system.sh`
- View examples: `examples/`
- Read docs: `docs/`

---

**Quick reference for the autonomous agent system** 📖

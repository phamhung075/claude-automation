# Autonomous File-Based Agent System - Complete Guide

**Date**: 2025-11-14
**Status**: ✅ Implementation Ready
**Approach**: File-based coordination + `claude -p` CLI + MCP API

---

## 🎯 System Overview

This system enables **fully autonomous multi-agent workflows** using:

1. **`claude -p`** - Non-interactive Claude CLI (uses your subscription, NO API key needed!)
2. **File system** - Coordination mechanism between agents
3. **Bash orchestrator** - Loop control and condition checking
4. **MCP API** - Task state synchronization
5. **Condition flags** - File-based workflow completion detection

### Key Benefits

✅ **No API costs** - Uses Claude Code subscription
✅ **Simple architecture** - Just Bash + files + claude CLI
✅ **Autonomous loops** - Runs until all conditions met
✅ **MCP integration** - Syncs with existing task management
✅ **Graceful stopping** - Checks conditions before stopping

---

## 🏗️ Architecture

### Component Diagram

```
┌─────────────────────────────────────────────────────────┐
│  Human Input                                            │
│  • Submit goal via start_autonomous_workflow.sh         │
│  • System generates task breakdown using claude -p      │
│  • Creates MCP tasks via API                            │
└────────────────┬────────────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────────────┐
│  Autonomous Orchestrator (Bash Loop)                    │
│  • Fetches next task from MCP API                       │
│  • Writes task context to file                          │
│  • Calls agent via: cat task.json | claude -p "..."     │
│  • Reads agent result from file                         │
│  • Updates MCP task status via API                      │
│  • Checks completion conditions (files exist?)          │
│  • Loop continues until all conditions met              │
└────────────────┬────────────────────────────────────────┘
                 │
      ┌──────────┼──────────┬──────────┐
      │          │          │          │
┌─────▼────┐ ┌──▼────┐ ┌───▼────┐ ┌───▼────┐
│ claude -p│ │claude │ │ claude │ │ claude │
│ (coding) │ │ (test)│ │(review)│ │(debug) │
│          │ │       │ │        │ │        │
│ Reads:   │ │Reads: │ │Reads:  │ │Reads:  │
│ task.json│ │task   │ │task    │ │task    │
│          │ │       │ │        │ │        │
│ Writes:  │ │Writes:│ │Writes: │ │Writes: │
│ result   │ │result │ │result  │ │result  │
└─────┬────┘ └──┬────┘ └───┬────┘ └───┬────┘
      │         │          │          │
      └─────────┼──────────┴──────────┘
                │
┌───────────────▼─────────────────────────────────────────┐
│  File System (Coordination Layer)                       │
│  /tmp/agenthub_autonomous/                              │
│  ├── current_task.json      (input to agents)           │
│  ├── task_result_*.json     (output from agents)        │
│  ├── tests_passed.flag      (condition check)           │
│  ├── review_approved.flag   (condition check)           │
│  ├── security_passed.flag   (condition check)           │
│  └── workflow_complete.flag (final condition)           │
└─────────────────────────────────────────────────────────┘
```

### Data Flow

```
1. Human submits goal
   ↓
2. AI generates task breakdown (claude -p)
   ↓
3. Tasks created in MCP database
   ↓
4. Orchestrator loop starts
   ↓
5. For each task:
   ┌─────────────────────────────────────┐
   │ a. Fetch next task from MCP API     │
   │ b. Write task to file               │
   │ c. Call: cat task | claude -p "..." │
   │ d. Agent writes result to file      │
   │ e. Read result file                 │
   │ f. Update MCP task via API          │
   └─────────────────────────────────────┘
   ↓
6. Check completion conditions:
   • All tasks status = "done"?
   • tests_passed.flag exists?
   • review_approved.flag exists?
   • security_passed.flag exists?
   ↓
7. If all true → STOP
   If any false → CONTINUE LOOP
```

---

## 🚀 Quick Start

### Prerequisites

```bash
# 1. Install required tools
sudo apt-get install jq curl

# 2. Verify Claude Code installed
which claude
# Should output: /usr/local/bin/claude (or similar)

# 3. Start agenthub backend
cd /home/daihu/__projects__/4genthub
docker-compose up -d  # Or your startup method

# 4. Verify MCP API is running
curl http://localhost:8000/health
# Should return: {"status": "healthy"}
```

### Start Autonomous Workflow

```bash
# Make scripts executable
chmod +x scripts/autonomous_orchestrator.sh
chmod +x scripts/start_autonomous_workflow.sh

# Start workflow with goal
./scripts/start_autonomous_workflow.sh \
    "project-uuid-123" \
    "feature/authentication" \
    "Build complete JWT authentication system with refresh tokens, password hashing, unit tests, and security audit"
```

### What Happens Next

```
🚀 Starting Autonomous Workflow
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Project: project-uuid-123
Branch: feature/authentication
Goal: Build complete JWT authentication system...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📍 Step 1: Getting git branch...
✅ Git Branch ID: branch-uuid-456

📋 Step 2: Generating task breakdown using AI...
✅ Task breakdown generated

📝 Step 3: Creating parent task...
✅ Parent task created: task-uuid-789

📋 Step 4: Creating subtasks...
  Creating: Design authentication architecture (Agent: system-architect-agent)
  Creating: Implement JWT token generation (Agent: coding-agent)
  Creating: Implement password hashing (Agent: coding-agent)
  Creating: Write unit tests (Agent: test-orchestrator-agent)
  Creating: Run security audit (Agent: security-auditor-agent)
  Creating: Code review (Agent: code-reviewer-agent)
✅ Created 6 subtasks

🔄 Step 5: Starting autonomous orchestrator...
Press Ctrl+C to stop

╔═══════════════════════════════════════════════════════╗
║   Autonomous Agent Orchestrator (File-Based)         ║
║   Using: claude -p + MCP API + File Coordination     ║
╚═══════════════════════════════════════════════════════╝

🚀 Setting up autonomous workspace...
✅ Workspace ready: /tmp/agenthub_autonomous
📝 Creating agent prompt templates...
✅ Agent prompts created
🔄 Starting autonomous orchestration loop...
📍 Git Branch ID: branch-uuid-456
🛑 Stop conditions: All tasks complete + Tests passed + Review approved + Security passed

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📍 Iteration #1
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Checking workflow completion status...
⏳ Still have 6 pending tasks
🔍 Fetching next task from MCP...
📋 Next task: Design authentication architecture
🤖 Agent: system-architect-agent
🆔 Task ID: task-uuid-790

🤖 Calling system-architect-agent agent...
✅ Agent completed successfully
✅ Task completed successfully

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📍 Iteration #2
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

...continues autonomously until complete...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🎊 Autonomous workflow completed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## 📂 File Structure

```
/tmp/agenthub_autonomous/
├── current_task.json              # Input: Current task for agent
├── task_result_<task-id>.json     # Output: Agent results
├── test_task.json                 # Generated test task
├── test_results.json              # Test execution results
├── review_task.json               # Generated review task
├── review_results.json            # Review results
├── security_task.json             # Generated security task
├── security_results.json          # Security audit results
├── tests_passed.flag              # Condition: Tests passed
├── review_approved.flag           # Condition: Review approved
├── security_passed.flag           # Condition: Security passed
├── workflow_complete.flag         # Final condition: All done
├── human_intervention_needed.flag # Agent blocked, needs help
├── blocker_details.txt            # Why agent is blocked
├── agent_raw_output.log           # Debug log from claude -p
└── prompts/
    ├── coding-agent.txt           # System prompt for coding
    ├── test-orchestrator-agent.txt
    ├── code-reviewer-agent.txt
    ├── debugger-agent.txt
    └── security-auditor-agent.txt
```

---

## 🔧 How It Works

### 1. Task Execution Flow

```bash
# Orchestrator fetches next task from MCP
task=$(curl -X POST http://localhost:8000/api/manage_task \
       -d '{"action":"next","git_branch_id":"branch-123"}')

# Writes task to file
echo "$task" > /tmp/agenthub_autonomous/current_task.json

# Calls agent via claude -p (uses your subscription!)
cat /tmp/agenthub_autonomous/current_task.json | \
claude -p \
  --append-system-prompt "$(cat /tmp/agenthub_autonomous/prompts/coding-agent.txt)" \
  --output-format json \
  "Execute this task and write results to /tmp/agenthub_autonomous/task_result_TASK_ID.json"

# Agent (Claude) reads task, does work, writes result using Bash tool
# Example agent actions:
#   1. Read task JSON
#   2. Understand requirements
#   3. Write code using Write tool
#   4. Run tests using Bash tool
#   5. Write result JSON using Bash tool

# Orchestrator reads result
result=$(cat /tmp/agenthub_autonomous/task_result_TASK_ID.json)

# Updates MCP task status
curl -X POST http://localhost:8000/api/manage_task \
     -d "{\"action\":\"complete\",\"task_id\":\"...\",\"completion_summary\":\"...\"}"
```

### 2. Completion Checking

```bash
# Check 1: All MCP tasks complete?
tasks=$(curl -X POST http://localhost:8000/api/manage_task \
        -d '{"action":"list","git_branch_id":"branch-123"}')
pending=$(echo "$tasks" | jq '[.tasks[] | select(.status != "done")] | length')

if [ "$pending" -gt 0 ]; then
    echo "Still have $pending pending tasks - CONTINUE LOOP"
fi

# Check 2: Tests passed?
if [ ! -f "/tmp/agenthub_autonomous/tests_passed.flag" ]; then
    echo "Tests not passed - CONTINUE LOOP"
    # Run test validation
fi

# Check 3: Review approved?
if [ ! -f "/tmp/agenthub_autonomous/review_approved.flag" ]; then
    echo "Review not approved - CONTINUE LOOP"
    # Run code review
fi

# Check 4: Security audit passed?
if [ ! -f "/tmp/agenthub_autonomous/security_passed.flag" ]; then
    echo "Security not passed - CONTINUE LOOP"
    # Run security audit
fi

# All checks passed?
if [ all_checks_true ]; then
    touch "/tmp/agenthub_autonomous/workflow_complete.flag"
    echo "🎉 WORKFLOW COMPLETE!"
    exit 0
fi
```

### 3. Human Intervention

When agent gets blocked:

```bash
# Agent writes result with status="blocked"
{
  "status": "blocked",
  "blocker_reason": "Need database credentials to continue",
  "summary": "Cannot proceed without DB access"
}

# Orchestrator detects block
touch /tmp/agenthub_autonomous/human_intervention_needed.flag
echo "Need database credentials" > /tmp/agenthub_autonomous/blocker_details.txt

# Orchestrator waits
while [ -f "/tmp/agenthub_autonomous/human_intervention_needed.flag" ]; do
    echo "⏸️ Waiting for human intervention..."
    sleep 5
done

# Human resolves issue and removes flag:
# rm /tmp/agenthub_autonomous/human_intervention_needed.flag

# Orchestrator resumes automatically!
```

---

## 🎨 Customization

### Add New Agent Type

```bash
# 1. Create agent prompt template
cat > /tmp/agenthub_autonomous/prompts/my-custom-agent.txt <<'EOF'
You are a custom agent specialized in [YOUR SPECIALTY].

IMPORTANT: After completing work, write result JSON to file using Bash tool.
EOF

# 2. Use in task assignment
curl -X POST http://localhost:8000/api/manage_subtask \
     -d '{
         "action": "create",
         "task_id": "parent-task-id",
         "title": "Custom task",
         "assignees": "my-custom-agent"
     }'

# 3. Orchestrator will automatically call it!
```

### Modify Completion Conditions

Edit `autonomous_orchestrator.sh` function `check_workflow_complete()`:

```bash
check_workflow_complete() {
    # Add custom condition
    if [ ! -f "$WORK_DIR/my_custom_check.flag" ]; then
        echo "Custom check not complete"
        return 1
    fi

    # Rest of checks...
}
```

### Change Validation Strategy

Edit validation functions:

```bash
run_tests() {
    # Customize test command
    cat > "$WORK_DIR/test_task.json" <<EOF
{
    "test_command": "npm test",  # Change this
    "coverage_threshold": 90     # Or this
}
EOF

    # Rest of function...
}
```

---

## 🐛 Troubleshooting

### Issue: Orchestrator exits immediately

**Cause**: No tasks in MCP or git branch ID invalid

**Solution**:
```bash
# Verify git branch exists
curl -X POST http://localhost:8000/api/manage_git_branch \
     -d '{"action":"list","project_id":"your-project-id"}' | jq

# List all tasks
curl -X POST http://localhost:8000/api/manage_task \
     -d '{"action":"list","git_branch_id":"your-branch-id"}' | jq
```

### Issue: Agent never writes result file

**Cause**: Agent system prompt missing file write instruction

**Solution**: Check agent prompt in `/tmp/agenthub_autonomous/prompts/`. Ensure it includes:

```
IMPORTANT OUTPUT FORMAT:
After completing your work, you MUST write a JSON result file using the Bash tool:

bash -c 'cat > /tmp/agenthub_autonomous/task_result_TASK_ID.json <<EOJ
{
  "status": "success",
  "summary": "..."
}
EOJ'
```

### Issue: Workflow never completes

**Cause**: Completion condition flags not being created

**Solution**: Check which flag is missing:

```bash
ls -la /tmp/agenthub_autonomous/*.flag

# If tests_passed.flag missing:
# Manually run tests or create flag:
touch /tmp/agenthub_autonomous/tests_passed.flag

# If review_approved.flag missing:
# Run code review manually or create flag
```

### Issue: `claude` command not found

**Cause**: Claude Code not installed or not in PATH

**Solution**:
```bash
# Install Claude Code
# https://docs.claude.com/en/docs/claude-code/installation

# Or add to PATH
export PATH=$PATH:/path/to/claude
```

### Debug Mode

Enable verbose logging:

```bash
# Edit autonomous_orchestrator.sh
# Change call_agent function to add --verbose:

cat "$task_file" | claude -p \
    --verbose \  # Add this line
    --append-system-prompt "$system_prompt" \
    ...
```

View logs:

```bash
tail -f /tmp/agenthub_autonomous/agent_raw_output.log
```

---

## 📊 Monitoring

### Watch Orchestrator Progress

```bash
# In separate terminal
watch -n 2 'ls -lh /tmp/agenthub_autonomous/*.json'
```

### Monitor MCP Task Status

```bash
# Check all tasks
curl -s -X POST http://localhost:8000/api/manage_task \
     -d '{"action":"list","git_branch_id":"your-branch-id"}' | \
     jq '.tasks[] | {title, status, progress_percentage}'
```

### Check Completion Flags

```bash
# Monitor flags
watch -n 2 'ls -lh /tmp/agenthub_autonomous/*.flag'
```

---

## 🚦 Stopping the Orchestrator

### Graceful Stop

```bash
# Ctrl+C in orchestrator terminal
# OR create stop flag:
touch /tmp/agenthub_autonomous/workflow_complete.flag
```

### Force Stop

```bash
# Find process
ps aux | grep autonomous_orchestrator.sh

# Kill it
kill <PID>
```

---

## 💡 Advanced Usage

### Parallel Agent Execution

Current implementation is sequential. To run agents in parallel:

```bash
# Modify autonomous_orchestrator.sh main_loop()
# Launch multiple agents simultaneously:

call_agent "coding-agent" "$task1_file" "$result1_file" &
call_agent "test-agent" "$task2_file" "$result2_file" &
call_agent "review-agent" "$task3_file" "$result3_file" &

# Wait for all to complete
wait
```

### Resume After Crash

```bash
# Orchestrator uses same workspace
# Just restart:
GIT_BRANCH_ID=branch-123 ./scripts/autonomous_orchestrator.sh

# It will resume from where it left off!
```

### Multiple Workflows

```bash
# Run multiple orchestrators for different branches
GIT_BRANCH_ID=branch-auth ./scripts/autonomous_orchestrator.sh &
GIT_BRANCH_ID=branch-ui ./scripts/autonomous_orchestrator.sh &
GIT_BRANCH_ID=branch-api ./scripts/autonomous_orchestrator.sh &

# Each uses separate workspace by branch ID
```

---

## 📈 Performance Considerations

### Latency

- **`claude -p` call**: ~2-5 seconds per execution
- **MCP API call**: ~50-200ms
- **File I/O**: <1ms
- **Total per task**: ~3-10 seconds

### Optimization

```bash
# Reduce sleep interval
SLEEP_INTERVAL=0.5  # Instead of 2 seconds

# Cache agent prompts in memory
# Batch MCP API calls
# Run multiple agents in parallel (see Advanced Usage)
```

---

## 🔗 Integration with Existing agenthub

This system **complements** the existing agenthub:

| Feature | Interactive agenthub | Autonomous System |
|---------|---------------------|------------------|
| Use Case | Human-guided workflows | Fully autonomous execution |
| Interface | Web UI + Claude terminal | Bash script + files |
| Control | Human orchestrates each step | AI orchestrates everything |
| Stopping | Human ends session | Automatic on completion |
| Best For | Exploratory work, learning | Production automation, CI/CD |

**Combined Usage**:
- Use interactive for **initial development** and **exploration**
- Use autonomous for **testing**, **CI/CD**, and **repetitive tasks**

---

## ✅ Next Steps

1. **Test the system**:
   ```bash
   ./scripts/start_autonomous_workflow.sh \
       "proj-123" \
       "test/simple-feature" \
       "Create a simple calculator function with tests"
   ```

2. **Monitor execution**:
   ```bash
   tail -f /tmp/agenthub_autonomous/agent_raw_output.log
   ```

3. **Iterate and improve**:
   - Add custom agents
   - Modify completion conditions
   - Optimize performance

---

## 📚 Additional Resources

- [Claude Code CLI Reference](https://docs.claude.com/en/docs/claude-code/cli-reference)
- [MCP Protocol Documentation](https://docs.claude.com/en/mcp)
- [agenthub Task Management API](../api-integration/mcp-tools-reference.md)

---

**Questions? Issues?**

Check `/tmp/agenthub_autonomous/agent_raw_output.log` for debugging!

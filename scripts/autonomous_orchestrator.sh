#!/bin/bash

###############################################################################
# Autonomous Agent Orchestrator
#
# Runs in infinite loop, coordinating agents via claude -p and file system
# Uses MCP API to manage task state
# Stops when all conditions met (tests pass, review approved, no pending tasks)
###############################################################################

# Configuration
WORK_DIR="/tmp/agenthub_autonomous"
MCP_API_URL="http://localhost:8000/api"
GIT_BRANCH_ID="${GIT_BRANCH_ID:-}"  # Set via environment variable
MAX_RETRIES=3
SLEEP_INTERVAL=2

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

###############################################################################
# Setup
###############################################################################

setup_workspace() {
    echo -e "${BLUE}🚀 Setting up autonomous workspace...${NC}"

    # Create working directory
    mkdir -p "$WORK_DIR"

    # Clean previous run
    rm -f "$WORK_DIR"/*.json
    rm -f "$WORK_DIR"/*.flag
    rm -f "$WORK_DIR"/*.log

    # Create agent prompts directory
    mkdir -p "$WORK_DIR/prompts"

    echo -e "${GREEN}✅ Workspace ready: $WORK_DIR${NC}"
}

###############################################################################
# MCP API Functions
###############################################################################

get_next_task() {
    local branch_id="$1"

    # Call MCP API to get next recommended task
    curl -s -X POST "$MCP_API_URL/manage_task" \
         -H "Content-Type: application/json" \
         -d "{
             \"action\": \"next\",
             \"git_branch_id\": \"$branch_id\",
             \"include_context\": \"true\"
         }"
}

update_task_status() {
    local task_id="$1"
    local status="$2"
    local details="$3"

    curl -s -X POST "$MCP_API_URL/manage_task" \
         -H "Content-Type: application/json" \
         -d "{
             \"action\": \"update\",
             \"task_id\": \"$task_id\",
             \"status\": \"$status\",
             \"details\": \"$details\"
         }"
}

complete_task() {
    local task_id="$1"
    local summary="$2"
    local testing_notes="$3"

    curl -s -X POST "$MCP_API_URL/manage_task" \
         -H "Content-Type: application/json" \
         -d "{
             \"action\": \"complete\",
             \"task_id\": \"$task_id\",
             \"completion_summary\": \"$summary\",
             \"testing_notes\": \"$testing_notes\"
         }"
}

list_all_tasks() {
    local branch_id="$1"

    curl -s -X POST "$MCP_API_URL/manage_task" \
         -H "Content-Type: application/json" \
         -d "{
             \"action\": \"list\",
             \"git_branch_id\": \"$branch_id\",
             \"status\": \"all\"
         }"
}

###############################################################################
# Agent Execution Functions
###############################################################################

call_agent() {
    local agent_type="$1"
    local task_file="$2"
    local output_file="$3"

    echo -e "${YELLOW}🤖 Calling $agent_type agent...${NC}"

    # Build agent-specific system prompt
    local system_prompt=$(cat "$WORK_DIR/prompts/${agent_type}.txt")

    # Call claude -p with task context piped in
    # Agent reads task from stdin, writes result to output_file
    cat "$task_file" | claude -p \
        --append-system-prompt "$system_prompt" \
        --output-format json \
        "Execute the task provided in JSON format. Save your results to $output_file in this JSON structure:
{
  \"status\": \"success\" or \"failed\" or \"blocked\",
  \"summary\": \"brief description of what was done\",
  \"files_modified\": [\"list of files\"],
  \"tests_passed\": true/false (if applicable),
  \"issues_found\": [\"list of issues if any\"],
  \"next_steps\": [\"recommended next actions\"],
  \"blocker_reason\": \"reason if blocked\" (optional)
}

After generating the result, use the Bash tool to write it to $output_file" \
        > "$WORK_DIR/agent_raw_output.log" 2>&1

    local exit_code=$?

    if [ $exit_code -eq 0 ] && [ -f "$output_file" ]; then
        echo -e "${GREEN}✅ Agent completed successfully${NC}"
        return 0
    else
        echo -e "${RED}❌ Agent failed (exit code: $exit_code)${NC}"
        cat "$WORK_DIR/agent_raw_output.log"
        return 1
    fi
}

###############################################################################
# Validation Functions
###############################################################################

run_tests() {
    echo -e "${BLUE}🧪 Running all tests...${NC}"

    # Create test task
    cat > "$WORK_DIR/test_task.json" <<EOF
{
    "task_type": "run_tests",
    "test_command": "pytest agenthub_main/src/tests/ -v",
    "coverage_threshold": 80
}
EOF

    # Call test agent
    call_agent "test-orchestrator-agent" \
               "$WORK_DIR/test_task.json" \
               "$WORK_DIR/test_results.json"

    # Check if tests passed
    if [ -f "$WORK_DIR/test_results.json" ]; then
        local tests_passed=$(jq -r '.tests_passed' "$WORK_DIR/test_results.json")

        if [ "$tests_passed" = "true" ]; then
            touch "$WORK_DIR/tests_passed.flag"
            echo -e "${GREEN}✅ All tests passed!${NC}"
            return 0
        else
            rm -f "$WORK_DIR/tests_passed.flag"
            echo -e "${RED}❌ Tests failed${NC}"
            return 1
        fi
    fi

    return 1
}

run_code_review() {
    echo -e "${BLUE}👀 Running code review...${NC}"

    # Create review task
    cat > "$WORK_DIR/review_task.json" <<EOF
{
    "task_type": "code_review",
    "scope": "all_changes",
    "focus_areas": ["security", "performance", "best_practices"]
}
EOF

    # Call review agent
    call_agent "code-reviewer-agent" \
               "$WORK_DIR/review_task.json" \
               "$WORK_DIR/review_results.json"

    # Check if review approved
    if [ -f "$WORK_DIR/review_results.json" ]; then
        local review_approved=$(jq -r '.status' "$WORK_DIR/review_results.json")

        if [ "$review_approved" = "success" ]; then
            touch "$WORK_DIR/review_approved.flag"
            echo -e "${GREEN}✅ Code review approved!${NC}"
            return 0
        else
            rm -f "$WORK_DIR/review_approved.flag"
            echo -e "${RED}❌ Code review found issues${NC}"
            return 1
        fi
    fi

    return 1
}

run_security_audit() {
    echo -e "${BLUE}🔒 Running security audit...${NC}"

    # Create security task
    cat > "$WORK_DIR/security_task.json" <<EOF
{
    "task_type": "security_audit",
    "scope": "all_code",
    "checks": ["sql_injection", "xss", "authentication", "secrets"]
}
EOF

    # Call security agent
    call_agent "security-auditor-agent" \
               "$WORK_DIR/security_task.json" \
               "$WORK_DIR/security_results.json"

    # Check if security passed
    if [ -f "$WORK_DIR/security_results.json" ]; then
        local security_passed=$(jq -r '.status' "$WORK_DIR/security_results.json")

        if [ "$security_passed" = "success" ]; then
            touch "$WORK_DIR/security_passed.flag"
            echo -e "${GREEN}✅ Security audit passed!${NC}"
            return 0
        else
            rm -f "$WORK_DIR/security_passed.flag"
            echo -e "${RED}❌ Security issues found${NC}"
            return 1
        fi
    fi

    return 1
}

###############################################################################
# Workflow Completion Check
###############################################################################

check_workflow_complete() {
    local branch_id="$1"

    echo -e "${BLUE}📊 Checking workflow completion status...${NC}"

    # Check 1: All tasks complete
    local tasks=$(list_all_tasks "$branch_id")
    local pending_count=$(echo "$tasks" | jq '[.tasks[] | select(.status != "done")] | length')

    if [ "$pending_count" -gt 0 ]; then
        echo -e "${YELLOW}⏳ Still have $pending_count pending tasks${NC}"
        return 1
    fi

    # Check 2: Tests passed flag exists
    if [ ! -f "$WORK_DIR/tests_passed.flag" ]; then
        echo -e "${YELLOW}⏳ Tests have not passed yet${NC}"
        return 1
    fi

    # Check 3: Review approved flag exists
    if [ ! -f "$WORK_DIR/review_approved.flag" ]; then
        echo -e "${YELLOW}⏳ Code review not approved yet${NC}"
        return 1
    fi

    # Check 4: Security audit passed flag exists
    if [ ! -f "$WORK_DIR/security_passed.flag" ]; then
        echo -e "${YELLOW}⏳ Security audit not complete${NC}"
        return 1
    fi

    # All conditions met!
    echo -e "${GREEN}✅ All workflow completion conditions met!${NC}"
    touch "$WORK_DIR/workflow_complete.flag"
    return 0
}

###############################################################################
# Main Orchestration Loop
###############################################################################

main_loop() {
    local branch_id="$1"
    local iteration=0

    echo -e "${BLUE}🔄 Starting autonomous orchestration loop...${NC}"
    echo -e "${BLUE}📍 Git Branch ID: $branch_id${NC}"
    echo -e "${BLUE}🛑 Stop conditions: All tasks complete + Tests passed + Review approved + Security passed${NC}"

    while true; do
        iteration=$((iteration + 1))
        echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${BLUE}📍 Iteration #$iteration${NC}"
        echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

        # Check if workflow is complete
        if check_workflow_complete "$branch_id"; then
            echo -e "\n${GREEN}🎉 WORKFLOW COMPLETE! All conditions met.${NC}"
            break
        fi

        # Get next task from MCP
        echo -e "${BLUE}🔍 Fetching next task from MCP...${NC}"
        local next_task=$(get_next_task "$branch_id")

        # Check if we got a task
        if [ -z "$next_task" ] || [ "$next_task" = "null" ]; then
            echo -e "${YELLOW}⚠️ No tasks available from MCP${NC}"

            # No tasks but workflow not complete - run validation
            echo -e "${BLUE}🔍 Running final validation...${NC}"

            run_tests
            run_code_review
            run_security_audit

            # Recheck completion
            if check_workflow_complete "$branch_id"; then
                echo -e "\n${GREEN}🎉 WORKFLOW COMPLETE!${NC}"
                break
            fi

            echo -e "${YELLOW}⏳ Waiting for conditions to be met...${NC}"
            sleep $SLEEP_INTERVAL
            continue
        fi

        # Extract task details
        local task_id=$(echo "$next_task" | jq -r '.task.id')
        local task_title=$(echo "$next_task" | jq -r '.task.title')
        local agent_type=$(echo "$next_task" | jq -r '.task.assignees[0]')

        echo -e "${GREEN}📋 Next task: $task_title${NC}"
        echo -e "${GREEN}🤖 Agent: $agent_type${NC}"
        echo -e "${GREEN}🆔 Task ID: $task_id${NC}"

        # Save task to file
        echo "$next_task" > "$WORK_DIR/current_task.json"

        # Update task to in_progress
        update_task_status "$task_id" "in_progress" "Agent $agent_type started execution"

        # Call agent to execute task
        local result_file="$WORK_DIR/task_result_${task_id}.json"

        if call_agent "$agent_type" "$WORK_DIR/current_task.json" "$result_file"; then
            # Agent succeeded - read result
            local result=$(cat "$result_file")
            local status=$(echo "$result" | jq -r '.status')
            local summary=$(echo "$result" | jq -r '.summary')
            local tests_passed=$(echo "$result" | jq -r '.tests_passed')

            if [ "$status" = "success" ]; then
                # Complete task in MCP
                complete_task "$task_id" "$summary" "Tests passed: $tests_passed"
                echo -e "${GREEN}✅ Task completed successfully${NC}"
            elif [ "$status" = "blocked" ]; then
                # Task is blocked - need human intervention
                local blocker_reason=$(echo "$result" | jq -r '.blocker_reason')
                update_task_status "$task_id" "blocked" "Blocked: $blocker_reason"
                echo -e "${RED}🚫 Task blocked: $blocker_reason${NC}"

                # Create flag for human intervention needed
                touch "$WORK_DIR/human_intervention_needed.flag"
                echo "$blocker_reason" > "$WORK_DIR/blocker_details.txt"

                # Wait for human to resolve
                echo -e "${YELLOW}⏸️ Waiting for human intervention...${NC}"
                echo -e "${YELLOW}Remove $WORK_DIR/human_intervention_needed.flag when resolved${NC}"

                while [ -f "$WORK_DIR/human_intervention_needed.flag" ]; do
                    sleep 5
                done

                echo -e "${GREEN}▶️ Resuming after human intervention${NC}"
            else
                # Task failed
                update_task_status "$task_id" "in_progress" "Failed: $summary. Retrying..."
                echo -e "${RED}❌ Task failed: $summary${NC}"
            fi
        else
            # Agent execution failed
            echo -e "${RED}❌ Agent execution failed${NC}"
            update_task_status "$task_id" "blocked" "Agent execution failed"
        fi

        # Brief pause before next iteration
        sleep $SLEEP_INTERVAL
    done

    echo -e "\n${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}🎊 Autonomous workflow completed!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

###############################################################################
# Agent Prompt Templates
###############################################################################

create_agent_prompts() {
    echo -e "${BLUE}📝 Creating agent prompt templates...${NC}"

    # Coding agent prompt
    cat > "$WORK_DIR/prompts/coding-agent.txt" <<'EOF'
You are an expert coding agent. Your role is to implement features, fix bugs, and write clean code.

IMPORTANT OUTPUT FORMAT:
After completing your work, you MUST write a JSON result file using the Bash tool:

bash -c 'cat > /tmp/agenthub_autonomous/task_result_TASK_ID.json <<EOJ
{
  "status": "success",
  "summary": "Brief description of what was implemented",
  "files_modified": ["list", "of", "modified", "files"],
  "tests_passed": true,
  "issues_found": [],
  "next_steps": ["recommended next actions"]
}
EOJ'

Replace TASK_ID with the actual task ID from the input.
EOF

    # Test agent prompt
    cat > "$WORK_DIR/prompts/test-orchestrator-agent.txt" <<'EOF'
You are a test orchestration agent. Your role is to write and run tests.

IMPORTANT OUTPUT FORMAT:
After running tests, you MUST write a JSON result file using the Bash tool with test results.
EOF

    # Review agent prompt
    cat > "$WORK_DIR/prompts/code-reviewer-agent.txt" <<'EOF'
You are a code review agent. Your role is to review code for quality, security, and best practices.

IMPORTANT OUTPUT FORMAT:
After reviewing, you MUST write a JSON result file using the Bash tool with review findings.
EOF

    # Debug agent prompt
    cat > "$WORK_DIR/prompts/debugger-agent.txt" <<'EOF'
You are a debugging agent. Your role is to find and fix bugs.

IMPORTANT OUTPUT FORMAT:
After debugging, you MUST write a JSON result file using the Bash tool with findings and fixes.
EOF

    # Security agent prompt
    cat > "$WORK_DIR/prompts/security-auditor-agent.txt" <<'EOF'
You are a security audit agent. Your role is to find security vulnerabilities.

IMPORTANT OUTPUT FORMAT:
After auditing, you MUST write a JSON result file using the Bash tool with security findings.
EOF

    echo -e "${GREEN}✅ Agent prompts created${NC}"
}

###############################################################################
# Entry Point
###############################################################################

main() {
    echo -e "${BLUE}╔═══════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║   Autonomous Agent Orchestrator (File-Based)         ║${NC}"
    echo -e "${BLUE}║   Using: claude -p + MCP API + File Coordination     ║${NC}"
    echo -e "${BLUE}╚═══════════════════════════════════════════════════════╝${NC}\n"

    # Check required tools
    if ! command -v claude &> /dev/null; then
        echo -e "${RED}❌ Error: 'claude' CLI not found. Please install Claude Code.${NC}"
        exit 1
    fi

    if ! command -v jq &> /dev/null; then
        echo -e "${RED}❌ Error: 'jq' not found. Install with: sudo apt-get install jq${NC}"
        exit 1
    fi

    if ! command -v curl &> /dev/null; then
        echo -e "${RED}❌ Error: 'curl' not found.${NC}"
        exit 1
    fi

    # Check for git branch ID
    if [ -z "$GIT_BRANCH_ID" ]; then
        echo -e "${RED}❌ Error: GIT_BRANCH_ID environment variable not set${NC}"
        echo -e "${YELLOW}Usage: GIT_BRANCH_ID=<branch-uuid> $0${NC}"
        exit 1
    fi

    # Setup
    setup_workspace
    create_agent_prompts

    # Run main loop
    main_loop "$GIT_BRANCH_ID"

    echo -e "\n${GREEN}✅ Orchestrator exiting cleanly${NC}"
}

# Handle Ctrl+C gracefully
trap 'echo -e "\n${YELLOW}⚠️ Interrupted by user. Cleaning up...${NC}"; exit 0' INT TERM

# Run
main "$@"

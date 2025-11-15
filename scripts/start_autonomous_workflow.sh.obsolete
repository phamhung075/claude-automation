#!/bin/bash

###############################################################################
# Start Autonomous Workflow
#
# Helper script to start an autonomous workflow with a high-level goal
#
# Usage:
#   Option 1: ./start_autonomous_workflow.sh config.json
#   Option 2: ./start_autonomous_workflow.sh <project-id> <branch> <goal>
#
# Config File Format (JSON):
# {
#   "project_id": "uuid",
#   "git_branch_name": "feature/name",
#   "goal": "High-level goal description",
#   "requirements": ["requirement 1", "requirement 2", ...],
#   "agents_to_use": ["agent-1", "agent-2", ...],
#   "constraints": {
#     "max_time_hours": 24,
#     "must_pass_tests": true
#   }
# }
###############################################################################

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MCP_API_URL="http://localhost:8000/api"

# Color output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

usage() {
    echo "Usage:"
    echo "  Option 1: $0 <config-file.json>"
    echo "  Option 2: $0 <project-id> <git-branch-name> <goal-description>"
    echo ""
    echo "Examples:"
    echo "  # Using config file:"
    echo "  $0 examples/example-workflow-auth.json"
    echo ""
    echo "  # Using command-line arguments:"
    echo "  $0 proj-123 feature/auth \"Build complete authentication system with JWT\""
    exit 1
}

if [ $# -lt 1 ]; then
    usage
fi

# Check if first argument is a JSON config file
if [ -f "$1" ] && [[ "$1" == *.json ]]; then
    echo -e "${BLUE}📄 Loading configuration from file: $1${NC}\n"

    CONFIG_FILE="$1"

    # Load configuration from JSON file
    if ! jq empty "$CONFIG_FILE" 2>/dev/null; then
        echo -e "${RED}❌ Error: Invalid JSON in config file${NC}"
        exit 1
    fi

    PROJECT_ID=$(jq -r '.project_id' "$CONFIG_FILE")
    BRANCH_NAME=$(jq -r '.git_branch_name' "$CONFIG_FILE")
    GOAL_DESCRIPTION=$(jq -r '.goal' "$CONFIG_FILE")
    REQUIREMENTS=$(jq -r '.requirements // empty | join("\n- ")' "$CONFIG_FILE")
    AGENTS_TO_USE=$(jq -r '.agents_to_use // empty | join(", ")' "$CONFIG_FILE")
    MAX_TIME_HOURS=$(jq -r '.constraints.max_time_hours // 24' "$CONFIG_FILE")

    echo -e "${GREEN}✅ Configuration loaded from file${NC}"

    # Add requirements to goal if present
    if [ -n "$REQUIREMENTS" ] && [ "$REQUIREMENTS" != "null" ]; then
        GOAL_DESCRIPTION="$GOAL_DESCRIPTION

Requirements:
- $REQUIREMENTS"
    fi

    # Add agent preferences to goal if present
    if [ -n "$AGENTS_TO_USE" ] && [ "$AGENTS_TO_USE" != "null" ]; then
        GOAL_DESCRIPTION="$GOAL_DESCRIPTION

Preferred Agents: $AGENTS_TO_USE"
    fi

else
    # Use command-line arguments (backward compatible)
    if [ $# -lt 3 ]; then
        usage
    fi

    PROJECT_ID="$1"
    BRANCH_NAME="$2"
    GOAL_DESCRIPTION="$3"
fi

echo -e "${BLUE}🚀 Starting Autonomous Workflow${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Project: $PROJECT_ID${NC}"
echo -e "${BLUE}Branch: $BRANCH_NAME${NC}"
echo -e "${BLUE}Goal:${NC} $(echo "$GOAL_DESCRIPTION" | head -n 1)"

if [ -n "${CONFIG_FILE:-}" ]; then
    echo -e "${BLUE}Config File: $CONFIG_FILE${NC}"
    if [ -n "${MAX_TIME_HOURS:-}" ]; then
        echo -e "${BLUE}Max Time: $MAX_TIME_HOURS hours${NC}"
    fi
fi

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

# Step 1: Create or get git branch
echo -e "${BLUE}📍 Step 1: Getting git branch...${NC}"

BRANCH_RESPONSE=$(curl -s -X POST "$MCP_API_URL/manage_git_branch" \
    -H "Content-Type: application/json" \
    -d "{
        \"action\": \"create\",
        \"project_id\": \"$PROJECT_ID\",
        \"git_branch_name\": \"$BRANCH_NAME\",
        \"git_branch_description\": \"$GOAL_DESCRIPTION\"
    }")

GIT_BRANCH_ID=$(echo "$BRANCH_RESPONSE" | jq -r '.git_branch.git_branch_id')

if [ "$GIT_BRANCH_ID" = "null" ] || [ -z "$GIT_BRANCH_ID" ]; then
    echo -e "${YELLOW}Branch already exists, fetching existing...${NC}"

    BRANCH_LIST=$(curl -s -X POST "$MCP_API_URL/manage_git_branch" \
        -H "Content-Type: application/json" \
        -d "{
            \"action\": \"list\",
            \"project_id\": \"$PROJECT_ID\"
        }")

    GIT_BRANCH_ID=$(echo "$BRANCH_LIST" | jq -r ".git_branches[] | select(.git_branch_name == \"$BRANCH_NAME\") | .git_branch_id")
fi

echo -e "${GREEN}✅ Git Branch ID: $GIT_BRANCH_ID${NC}\n"

# Step 2: Use claude -p to generate task breakdown
echo -e "${BLUE}📋 Step 2: Generating task breakdown using AI...${NC}"

TASK_BREAKDOWN=$(claude -p --output-format json "
Analyze this goal and break it down into specific tasks:

Goal: $GOAL_DESCRIPTION

Generate a task breakdown with:
1. Main parent task title
2. List of subtasks (5-10 tasks)
3. For each subtask:
   - Title
   - Description
   - Recommended agent type (coding-agent, test-orchestrator-agent, code-reviewer-agent, security-auditor-agent, debugger-agent)
   - Dependencies (which task IDs must complete first, use numbers 1-N)
   - Estimated effort (in hours)

Return JSON format:
{
  \"parent_task\": {
    \"title\": \"...\",
    \"description\": \"...\"
  },
  \"subtasks\": [
    {
      \"id\": 1,
      \"title\": \"...\",
      \"description\": \"...\",
      \"agent\": \"coding-agent\",
      \"dependencies\": [],
      \"effort_hours\": 2
    }
  ]
}
")

echo "$TASK_BREAKDOWN" > /tmp/task_breakdown.json
echo -e "${GREEN}✅ Task breakdown generated${NC}\n"

# Step 3: Create parent task in MCP
echo -e "${BLUE}📝 Step 3: Creating parent task...${NC}"

PARENT_TITLE=$(echo "$TASK_BREAKDOWN" | jq -r '.parent_task.title')
PARENT_DESC=$(echo "$TASK_BREAKDOWN" | jq -r '.parent_task.description')

PARENT_TASK=$(curl -s -X POST "$MCP_API_URL/manage_task" \
    -H "Content-Type: application/json" \
    -d "{
        \"action\": \"create\",
        \"git_branch_id\": \"$GIT_BRANCH_ID\",
        \"title\": \"$PARENT_TITLE\",
        \"description\": \"$PARENT_DESC\",
        \"assignees\": \"master-orchestrator-agent\",
        \"details\": \"$GOAL_DESCRIPTION\"
    }")

PARENT_TASK_ID=$(echo "$PARENT_TASK" | jq -r '.task.task_id')
echo -e "${GREEN}✅ Parent task created: $PARENT_TASK_ID${NC}\n"

# Step 4: Create all subtasks
echo -e "${BLUE}📋 Step 4: Creating subtasks...${NC}"

SUBTASK_COUNT=$(echo "$TASK_BREAKDOWN" | jq '.subtasks | length')

for ((i=0; i<$SUBTASK_COUNT; i++)); do
    SUBTASK=$(echo "$TASK_BREAKDOWN" | jq ".subtasks[$i]")

    SUBTASK_TITLE=$(echo "$SUBTASK" | jq -r '.title')
    SUBTASK_DESC=$(echo "$SUBTASK" | jq -r '.description')
    SUBTASK_AGENT=$(echo "$SUBTASK" | jq -r '.agent')

    echo -e "  Creating: $SUBTASK_TITLE (Agent: $SUBTASK_AGENT)"

    curl -s -X POST "$MCP_API_URL/manage_subtask" \
        -H "Content-Type: application/json" \
        -d "{
            \"action\": \"create\",
            \"task_id\": \"$PARENT_TASK_ID\",
            \"title\": \"$SUBTASK_TITLE\",
            \"description\": \"$SUBTASK_DESC\",
            \"assignees\": \"$SUBTASK_AGENT\"
        }" > /dev/null
done

echo -e "${GREEN}✅ Created $SUBTASK_COUNT subtasks${NC}\n"

# Step 5: Start autonomous orchestrator
echo -e "${BLUE}🔄 Step 5: Starting autonomous orchestrator...${NC}"
echo -e "${YELLOW}Press Ctrl+C to stop${NC}\n"

export GIT_BRANCH_ID="$GIT_BRANCH_ID"
exec "$SCRIPT_DIR/autonomous_orchestrator.sh"

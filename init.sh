#!/bin/bash

###############################################################################
# Claude Automation Initialization Script
#
# Sets up the autonomous agent system for first use
###############################################################################

set -e

# Step results tracking
STEP_RESULTS=()

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Claude Automation System Initialization            ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════╝${NC}\n"

# Detect script location
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Step 1: Check prerequisites
echo -e "${BLUE}📋 Step 1: Checking prerequisites...${NC}"

STEP1_STATUS="passed"
STEP1_ERRORS=""

if ! command -v claude &> /dev/null; then
    echo -e "${RED}❌ Error: 'claude' CLI not found${NC}"
    echo -e "${YELLOW}Install Claude Code: https://docs.claude.com/en/docs/claude-code/installation${NC}"
    STEP1_STATUS="failed"
    STEP1_ERRORS="claude CLI not found"
    exit 1
fi
echo -e "${GREEN}✅ Claude Code CLI found${NC}"

if ! command -v jq &> /dev/null; then
    echo -e "${YELLOW}⚠️ Warning: 'jq' not found. Installing...${NC}"
    sudo apt-get update && sudo apt-get install -y jq
fi
echo -e "${GREEN}✅ jq found${NC}"

if ! command -v curl &> /dev/null; then
    echo -e "${RED}❌ Error: 'curl' not found${NC}"
    STEP1_STATUS="failed"
    STEP1_ERRORS="curl not found"
    exit 1
fi
echo -e "${GREEN}✅ curl found${NC}"

STEP_RESULTS+=("{\"step\":\"check_prerequisites\",\"status\":\"$STEP1_STATUS\",\"message\":\"Prerequisites checked\",\"errors\":\"$STEP1_ERRORS\"}")
echo ""

# Step 2: Make scripts executable
echo -e "${BLUE}📝 Step 2: Making scripts executable...${NC}"

STEP2_STATUS="passed"
STEP2_ERRORS=""
if chmod +x "$SCRIPT_DIR"/scripts/*.sh && chmod +x "$SCRIPT_DIR"/tests/*.sh; then
    echo -e "${GREEN}✅ Scripts are now executable${NC}"
else
    STEP2_STATUS="failed"
    STEP2_ERRORS="Failed to make scripts executable"
fi

STEP_RESULTS+=("{\"step\":\"make_scripts_executable\",\"status\":\"$STEP2_STATUS\",\"message\":\"Scripts made executable\",\"errors\":\"$STEP2_ERRORS\"}")
echo ""

# Step 3: Create working directory
echo -e "${BLUE}📂 Step 3: Setting up working directory...${NC}"

WORK_DIR="/tmp/agenthub_autonomous"
STEP3_STATUS="passed"
STEP3_ERRORS=""
if mkdir -p "$WORK_DIR" && mkdir -p "$WORK_DIR/prompts"; then
    echo -e "${GREEN}✅ Working directory: $WORK_DIR${NC}"
else
    STEP3_STATUS="failed"
    STEP3_ERRORS="Failed to create working directory"
fi

STEP_RESULTS+=("{\"step\":\"create_working_directory\",\"status\":\"$STEP3_STATUS\",\"message\":\"Working directory: $WORK_DIR\",\"errors\":\"$STEP3_ERRORS\"}")
echo ""

# Step 4: Initialize shared knowledge
echo -e "${BLUE}📚 Step 4: Initializing shared knowledge system...${NC}"

STEP4_STATUS="passed"
STEP4_ERRORS=""
if "$SCRIPT_DIR/scripts/shared_knowledge_manager.sh" init; then
    echo -e "${GREEN}✅ Shared knowledge initialized${NC}"
else
    STEP4_STATUS="failed"
    STEP4_ERRORS="Failed to initialize shared knowledge"
fi

STEP_RESULTS+=("{\"step\":\"initialize_shared_knowledge\",\"status\":\"$STEP4_STATUS\",\"message\":\"Shared knowledge initialized\",\"errors\":\"$STEP4_ERRORS\"}")
echo ""

# Step 5: Create agent prompts
echo -e "${BLUE}🤖 Step 5: Creating agent prompt templates...${NC}"

STEP5_STATUS="passed"
STEP5_ERRORS=""
if "$SCRIPT_DIR/scripts/agent_prompts_with_knowledge.sh"; then
    echo -e "${GREEN}✅ Agent prompts created${NC}"
else
    STEP5_STATUS="failed"
    STEP5_ERRORS="Failed to create agent prompts"
fi

STEP_RESULTS+=("{\"step\":\"create_agent_prompts\",\"status\":\"$STEP5_STATUS\",\"message\":\"Agent prompts created\",\"errors\":\"$STEP5_ERRORS\"}")
echo ""

# Step 6: Run tests
echo -e "${BLUE}🧪 Step 6: Running system tests...${NC}"

STEP6_STATUS="passed"
STEP6_ERRORS=""
if "$SCRIPT_DIR/tests/test_autonomous_system.sh"; then
    echo -e "${GREEN}✅ All tests passed!${NC}"
else
    STEP6_STATUS="warning"
    STEP6_ERRORS="Some tests failed"
    echo -e "${YELLOW}⚠️ Some tests failed, but setup is complete${NC}"
fi

STEP_RESULTS+=("{\"step\":\"run_tests\",\"status\":\"$STEP6_STATUS\",\"message\":\"System tests executed\",\"errors\":\"$STEP6_ERRORS\"}")
echo ""

# Step 7: Generate JSON initialization report
echo -e "${BLUE}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Initialization Complete (JSON Format)               ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════╝${NC}\n"

# Calculate stats
TOTAL_STEPS=${#STEP_RESULTS[@]}
PASSED_STEPS=$(printf '%s\n' "${STEP_RESULTS[@]}" | grep -c '"status":"passed"' || true)
FAILED_STEPS=$(printf '%s\n' "${STEP_RESULTS[@]}" | grep -c '"status":"failed"' || true)
WARNING_STEPS=$(printf '%s\n' "${STEP_RESULTS[@]}" | grep -c '"status":"warning"' || true)

# Build JSON report
STEP_RESULTS_JSON=$(printf '%s,' "${STEP_RESULTS[@]}" | sed 's/,$//')

INIT_REPORT="$WORK_DIR/init_report.json"
cat > "$INIT_REPORT" <<EOF
{
  "initialization": "Claude Automation System",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "status": "$([ $FAILED_STEPS -eq 0 ] && echo "success" || echo "partial")",
  "environment": {
    "script_dir": "$SCRIPT_DIR",
    "work_dir": "$WORK_DIR",
    "prompts_dir": "$WORK_DIR/prompts",
    "prerequisites": {
      "claude": "$(command -v claude)",
      "jq": "$(command -v jq)",
      "curl": "$(command -v curl)"
    }
  },
  "results": {
    "total_steps": $TOTAL_STEPS,
    "passed": $PASSED_STEPS,
    "failed": $FAILED_STEPS,
    "warnings": $WARNING_STEPS,
    "steps": [$STEP_RESULTS_JSON]
  },
  "configuration": {
    "working_directory": "$WORK_DIR",
    "scripts_location": "$SCRIPT_DIR/scripts",
    "agent_prompts": "$WORK_DIR/prompts",
    "mcp_api_endpoint": "http://localhost:8000"
  },
  "quick_start": {
    "start_workflow": {
      "command": "$SCRIPT_DIR/scripts/start_autonomous_workflow.sh",
      "example": "$SCRIPT_DIR/scripts/start_autonomous_workflow.sh \"project-id\" \"feature/branch\" \"Build feature with tests\""
    },
    "demo": {
      "command": "$SCRIPT_DIR/scripts/demo_agent_communication.sh"
    }
  },
  "documentation": {
    "readme": "$SCRIPT_DIR/README.md",
    "quick_reference": "$SCRIPT_DIR/QUICK_REFERENCE.md",
    "submodule_setup": "$SCRIPT_DIR/SETUP_AS_SUBMODULE.md",
    "docs_directory": "$SCRIPT_DIR/docs"
  },
  "next_steps": [
    "Configure MCP API endpoint (if not localhost:8000)",
    "Run demo: ./scripts/demo_agent_communication.sh",
    "Start your first workflow"
  ]
}
EOF

# Display formatted JSON
echo -e "${GREEN}Initialization Report:${NC}\n"
cat "$INIT_REPORT" | jq .

echo ""
if [ $FAILED_STEPS -eq 0 ]; then
    echo -e "${GREEN}✅ All $TOTAL_STEPS steps completed successfully!${NC}"
else
    echo -e "${YELLOW}⚠️  $FAILED_STEPS of $TOTAL_STEPS steps failed${NC}"
fi

if [ $WARNING_STEPS -gt 0 ]; then
    echo -e "${YELLOW}⚠️  $WARNING_STEPS step(s) completed with warnings${NC}"
fi

echo ""
echo -e "${BLUE}📄 Full report saved to: $INIT_REPORT${NC}"
echo ""
echo -e "${GREEN}🚀 Claude Automation is ready to use!${NC}\n"

# Launch Instructions
echo -e "${BLUE}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   How to Launch the Autonomous Agent System          ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════╝${NC}\n"

echo -e "${YELLOW}📋 STEP 1: Verify Prerequisites${NC}"
echo -e "   Make sure your MCP API backend is running:"
echo -e "   ${CYAN}curl http://localhost:8000/health${NC}\n"

echo -e "${YELLOW}🎯 STEP 2: Choose Your Launch Method${NC}\n"

echo -e "${GREEN}Option A: Quick Demo (Recommended First)${NC}"
echo -e "   See how agents communicate with each other:"
echo -e "   ${CYAN}cd $SCRIPT_DIR${NC}"
echo -e "   ${CYAN}./scripts/demo_agent_communication.sh${NC}\n"

echo -e "${GREEN}Option B: Start Autonomous Workflow${NC}"
echo -e "   Launch a complete autonomous workflow with multiple agents:\n"

echo -e "   ${YELLOW}Method 1: Using Configuration File (Recommended for Complex Workflows)${NC}"
echo -e "   ${CYAN}./scripts/start_autonomous_workflow.sh examples/example-workflow-auth.json${NC}\n"

echo -e "   ${YELLOW}Method 2: Using Command-Line Arguments${NC}"
echo -e "   ${CYAN}./scripts/start_autonomous_workflow.sh \\${NC}"
echo -e "   ${CYAN}    \"<project-uuid>\" \\${NC}"
echo -e "   ${CYAN}    \"<git-branch-name>\" \\${NC}"
echo -e "   ${CYAN}    \"<your-goal-description>\"${NC}\n"

echo -e "   ${BLUE}Example - Config File (Authentication System):${NC}"
echo -e "   ${CYAN}./scripts/start_autonomous_workflow.sh \\${NC}"
echo -e "   ${CYAN}    examples/example-workflow-auth.json${NC}\n"

echo -e "   ${BLUE}Example - CLI Arguments (Simple Calculator):${NC}"
echo -e "   ${CYAN}./scripts/start_autonomous_workflow.sh \\${NC}"
echo -e "   ${CYAN}    \"550e8400-e29b-41d4-a716-446655440000\" \\${NC}"
echo -e "   ${CYAN}    \"feature/calculator\" \\${NC}"
echo -e "   ${CYAN}    \"Create calculator with add, subtract, multiply, divide. Include unit tests.\"${NC}\n"

echo -e "${YELLOW}📊 STEP 3: Monitor Progress${NC}"
echo -e "   While workflow is running, open a new terminal and monitor:"
echo -e "   ${CYAN}# Watch file changes${NC}"
echo -e "   ${CYAN}watch -n 2 'ls -lh $WORK_DIR'${NC}\n"
echo -e "   ${CYAN}# View agent logs${NC}"
echo -e "   ${CYAN}tail -f $WORK_DIR/agent_raw_output.log${NC}\n"
echo -e "   ${CYAN}# Check shared knowledge${NC}"
echo -e "   ${CYAN}jq . $WORK_DIR/shared_knowledge.json${NC}\n"

echo -e "${YELLOW}🔧 STEP 4: Workflow Configuration (Recommended)${NC}"
echo -e "   ${GREEN}Benefits of Using Config Files:${NC}"
echo -e "   • Define complex workflows with multiple requirements"
echo -e "   • Specify agent preferences and constraints"
echo -e "   • Reusable configuration for similar tasks"
echo -e "   • Version control your workflow definitions\n"

echo -e "   ${CYAN}Create a custom workflow JSON file:${NC}"
echo -e "   ${CYAN}cp $SCRIPT_DIR/examples/example-workflow-auth.json my-workflow.json${NC}"
echo -e "   ${CYAN}# Edit my-workflow.json with your requirements${NC}"
echo -e "   ${CYAN}# Then run: ./scripts/start_autonomous_workflow.sh my-workflow.json${NC}\n"

echo -e "${YELLOW}⚙️  Configuration Options:${NC}"
echo -e "   ${CYAN}MCP API Endpoint:${NC} Set in autonomous_orchestrator.sh (default: localhost:8000)"
echo -e "   ${CYAN}Working Directory:${NC} $WORK_DIR"
echo -e "   ${CYAN}Sleep Interval:${NC} Set in autonomous_orchestrator.sh (default: 2 seconds)"
echo -e "   ${CYAN}Agent Prompts:${NC} Customize in $WORK_DIR/prompts/\n"

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}🎓 Quick Tips:${NC}"
echo -e "  • Start with the demo to understand agent communication"
echo -e "  • Use simple goals first (like calculator)"
echo -e "  • Monitor logs in real-time to see agent decisions"
echo -e "  • Agents will create flags when tasks complete"
echo -e "  • Human intervention needed if workflow blocks"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

echo -e "${BLUE}📚 Documentation:${NC}"
echo -e "  ${CYAN}README.md${NC}              - Full system overview"
echo -e "  ${CYAN}QUICK_REFERENCE.md${NC}     - Command cheat sheet"
echo -e "  ${CYAN}docs/architecture.md${NC}   - System design details"
echo -e "  ${CYAN}docs/usage-guide.md${NC}    - Complete usage instructions"
echo -e "  ${CYAN}docs/agent-communication.md${NC} - Agent coordination details\n"

echo -e "${BLUE}🆘 Troubleshooting:${NC}"
echo -e "  ${CYAN}Issue:${NC} MCP API not responding"
echo -e "    ${YELLOW}→${NC} Check: curl http://localhost:8000/health"
echo -e "    ${YELLOW}→${NC} Fix: Start your backend (docker-compose up -d)\n"
echo -e "  ${CYAN}Issue:${NC} Agents not writing results"
echo -e "    ${YELLOW}→${NC} Check: cat $WORK_DIR/prompts/coding-agent.txt"
echo -e "    ${YELLOW}→${NC} Ensure Bash tool instructions are present\n"
echo -e "  ${CYAN}Issue:${NC} Workflow won't complete"
echo -e "    ${YELLOW}→${NC} Check: ls -la $WORK_DIR/*.flag"
echo -e "    ${YELLOW}→${NC} Manually create missing flags if needed\n"

echo -e "${GREEN}✨ Ready to start your first autonomous workflow!${NC}\n"

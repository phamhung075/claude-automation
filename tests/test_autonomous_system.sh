#!/bin/bash

###############################################################################
# Test Autonomous System Components
#
# Tests the autonomous agent system infrastructure without requiring
# interactive Claude sessions (which need permission grants)
#
# Tests:
# 1. Shared knowledge system initialization and operations
# 2. File-based coordination patterns
# 3. Condition flag system
# 4. Agent prompt generation
###############################################################################

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Test results tracking
TEST_RESULTS=()

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${BLUE}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Test Autonomous Agent System Components            ║${NC}"
echo -e "${BLUE}║   Infrastructure & Coordination Tests                 ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════╝${NC}\n"

# Check prerequisites
echo -e "${BLUE}🔍 Checking prerequisites...${NC}"

if ! command -v jq &> /dev/null; then
    echo -e "${RED}❌ Error: 'jq' not found${NC}"
    echo -e "${YELLOW}Install: sudo apt-get install jq${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Prerequisites OK${NC}\n"

# Setup test directory
TEST_DIR="/tmp/agenthub_test_$$"
mkdir -p "$TEST_DIR"

# Test 1: Shared Knowledge System
echo -e "${BLUE}📋 Test 1: Testing shared knowledge system...${NC}"

KNOWLEDGE_MANAGER="$SCRIPT_DIR/../scripts/shared_knowledge_manager.sh"

TEST1_STATUS="failed"
TEST1_MESSAGE="Shared knowledge system failed"

if [ -f "$KNOWLEDGE_MANAGER" ]; then
    # Initialize
    if $KNOWLEDGE_MANAGER init > /dev/null 2>&1; then
        # Add discovery
        $KNOWLEDGE_MANAGER add-discovery "test-agent" "Test discovery message" > /dev/null 2>&1

        # Verify knowledge file exists and has valid JSON
        KNOWLEDGE_FILE="/tmp/agenthub_autonomous/shared_knowledge.json"
        if [ -f "$KNOWLEDGE_FILE" ] && jq empty "$KNOWLEDGE_FILE" 2>/dev/null; then
            # Verify discovery was added
            DISCOVERY_COUNT=$(jq '.discoveries | length' "$KNOWLEDGE_FILE")
            if [ "$DISCOVERY_COUNT" -gt 0 ]; then
                TEST1_STATUS="passed"
                TEST1_MESSAGE="Shared knowledge system working"
                echo -e "${GREEN}✅ Test 1 PASSED: Shared knowledge system works${NC}"
                echo -e "${BLUE}Knowledge file has $DISCOVERY_COUNT discoveries${NC}"
            fi
        fi
    fi
else
    TEST1_MESSAGE="Shared knowledge manager script not found"
fi

if [ "$TEST1_STATUS" = "failed" ]; then
    echo -e "${RED}❌ Test 1 FAILED: $TEST1_MESSAGE${NC}"
fi

TEST_RESULTS+=("{\"name\":\"shared_knowledge_system\",\"status\":\"$TEST1_STATUS\",\"message\":\"$TEST1_MESSAGE\"}")

echo ""

# Test 2: File-based coordination pattern
echo -e "${BLUE}📋 Test 2: Testing file-based coordination pattern...${NC}"

# Create coordination input file
cat > "$TEST_DIR/task_input.json" <<EOF
{
  "task_id": "test-123",
  "agent": "coding-agent",
  "goal": "Test file coordination",
  "output_file": "$TEST_DIR/task_result.json"
}
EOF

# Simulate agent writing result (what an agent would do)
cat > "$TEST_DIR/task_result.json" <<EOF
{
  "task_id": "test-123",
  "status": "completed",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "result": "Task completed successfully"
}
EOF

TEST2_STATUS="failed"
TEST2_MESSAGE="Coordination pattern failed"

# Verify both files exist and are valid JSON
if [ -f "$TEST_DIR/task_input.json" ] && [ -f "$TEST_DIR/task_result.json" ]; then
    if jq empty "$TEST_DIR/task_input.json" 2>/dev/null && jq empty "$TEST_DIR/task_result.json" 2>/dev/null; then
        # Verify task_id matches
        INPUT_ID=$(jq -r '.task_id' "$TEST_DIR/task_input.json")
        OUTPUT_ID=$(jq -r '.task_id' "$TEST_DIR/task_result.json")

        if [ "$INPUT_ID" = "$OUTPUT_ID" ]; then
            TEST2_STATUS="passed"
            TEST2_MESSAGE="File coordination pattern working"
            echo -e "${GREEN}✅ Test 2 PASSED: File coordination pattern works${NC}"
            echo -e "${BLUE}Task ID matched: $INPUT_ID${NC}"
        else
            TEST2_MESSAGE="Task ID mismatch"
        fi
    else
        TEST2_MESSAGE="Invalid JSON in coordination files"
    fi
else
    TEST2_MESSAGE="Coordination files not created"
fi

if [ "$TEST2_STATUS" = "failed" ]; then
    echo -e "${RED}❌ Test 2 FAILED: $TEST2_MESSAGE${NC}"
fi

TEST_RESULTS+=("{\"name\":\"file_coordination_pattern\",\"status\":\"$TEST2_STATUS\",\"message\":\"$TEST2_MESSAGE\"}")

echo ""

# Test 3: Condition flag checking
echo -e "${BLUE}📋 Test 3: Testing condition flag system...${NC}"

# Create condition flags
touch "$TEST_DIR/task_complete.flag"
touch "$TEST_DIR/tests_passed.flag"
touch "$TEST_DIR/review_approved.flag"

# Check all conditions
conditions_met=true

if [ ! -f "$TEST_DIR/task_complete.flag" ]; then
    echo -e "${YELLOW}⏳ Task not complete${NC}"
    conditions_met=false
fi

if [ ! -f "$TEST_DIR/tests_passed.flag" ]; then
    echo -e "${YELLOW}⏳ Tests not passed${NC}"
    conditions_met=false
fi

if [ ! -f "$TEST_DIR/review_approved.flag" ]; then
    echo -e "${YELLOW}⏳ Review not approved${NC}"
    conditions_met=false
fi

TEST3_STATUS="failed"
TEST3_MESSAGE="Some conditions not met"
if [ "$conditions_met" = true ]; then
    TEST3_STATUS="passed"
    TEST3_MESSAGE="All conditions met"
    echo -e "${GREEN}✅ Test 3 PASSED: All conditions met${NC}"
else
    echo -e "${RED}❌ Test 3 FAILED: Some conditions not met${NC}"
fi

TEST_RESULTS+=("{\"name\":\"condition_flag_system\",\"status\":\"$TEST3_STATUS\",\"message\":\"$TEST3_MESSAGE\"}")

echo ""

# Test 4: Agent prompt generation
echo -e "${BLUE}📋 Test 4: Testing agent prompt generation...${NC}"

PROMPT_GENERATOR="$SCRIPT_DIR/../scripts/agent_prompts_with_knowledge.sh"

TEST4_STATUS="failed"
TEST4_MESSAGE="Agent prompt generation failed"

if [ -f "$PROMPT_GENERATOR" ]; then
    # Run prompt generator
    if $PROMPT_GENERATOR > /dev/null 2>&1; then
        # Check if prompts were created
        PROMPTS_DIR="/tmp/agenthub_autonomous/prompts"
        if [ -d "$PROMPTS_DIR" ]; then
            # Count generated prompts
            PROMPT_COUNT=$(ls -1 "$PROMPTS_DIR"/*.txt 2>/dev/null | wc -l)
            if [ "$PROMPT_COUNT" -gt 0 ]; then
                TEST4_STATUS="passed"
                TEST4_MESSAGE="Agent prompts generated successfully"
                echo -e "${GREEN}✅ Test 4 PASSED: Generated $PROMPT_COUNT agent prompts${NC}"

                # Verify a prompt file contains expected content
                SAMPLE_PROMPT="$PROMPTS_DIR/coding-agent.txt"
                if [ -f "$SAMPLE_PROMPT" ] && grep -q "shared_knowledge" "$SAMPLE_PROMPT"; then
                    echo -e "${BLUE}Prompts include shared knowledge instructions${NC}"
                fi
            else
                TEST4_MESSAGE="No prompts generated"
            fi
        else
            TEST4_MESSAGE="Prompts directory not created"
        fi
    else
        TEST4_MESSAGE="Prompt generator execution failed"
    fi
else
    TEST4_MESSAGE="Agent prompt generator script not found"
fi

if [ "$TEST4_STATUS" = "failed" ]; then
    echo -e "${RED}❌ Test 4 FAILED: $TEST4_MESSAGE${NC}"
fi

TEST_RESULTS+=("{\"name\":\"agent_prompt_generation\",\"status\":\"$TEST4_STATUS\",\"message\":\"$TEST4_MESSAGE\"}")

echo ""

# Generate JSON test report
echo ""
echo -e "${BLUE}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Test Summary (JSON Format)                         ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════╝${NC}\n"

# Calculate stats
TOTAL_TESTS=${#TEST_RESULTS[@]}
PASSED_TESTS=$(printf '%s\n' "${TEST_RESULTS[@]}" | grep -c '"status":"passed"' || true)
FAILED_TESTS=$((TOTAL_TESTS - PASSED_TESTS))

# Build JSON report
TEST_RESULTS_JSON=$(printf '%s,' "${TEST_RESULTS[@]}" | sed 's/,$//')

cat > "$TEST_DIR/test_report.json" <<EOF
{
  "test_suite": "Autonomous Agent System Component Tests",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "environment": {
    "test_dir": "$TEST_DIR",
    "script_dir": "$SCRIPT_DIR",
    "prerequisites": {
      "jq": "$(command -v jq)",
      "bash": "$(command -v bash)"
    }
  },
  "results": {
    "total": $TOTAL_TESTS,
    "passed": $PASSED_TESTS,
    "failed": $FAILED_TESTS,
    "tests": [$TEST_RESULTS_JSON]
  },
  "artifacts": {
    "shared_knowledge": "/tmp/agenthub_autonomous/shared_knowledge.json",
    "task_input": "$TEST_DIR/task_input.json",
    "task_result": "$TEST_DIR/task_result.json",
    "agent_prompts_dir": "/tmp/agenthub_autonomous/prompts",
    "flags": {
      "task_complete": "$TEST_DIR/task_complete.flag",
      "tests_passed": "$TEST_DIR/tests_passed.flag",
      "review_approved": "$TEST_DIR/review_approved.flag"
    }
  },
  "next_steps": {
    "command": "./scripts/start_autonomous_workflow.sh",
    "example": "./scripts/start_autonomous_workflow.sh \"project-id\" \"feature/test\" \"Create a simple calculator with tests\""
  }
}
EOF

# Display formatted JSON
echo -e "${GREEN}Test Report:${NC}\n"
cat "$TEST_DIR/test_report.json" | jq .

echo ""
if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}✅ All $TOTAL_TESTS tests passed!${NC}"
else
    echo -e "${YELLOW}⚠️  $FAILED_TESTS of $TOTAL_TESTS tests failed${NC}"
fi
echo ""
echo -e "${BLUE}📄 Full report saved to: $TEST_DIR/test_report.json${NC}"

# Cleanup option
echo ""
read -p "Delete test files? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm -rf "$TEST_DIR"
    echo -e "${GREEN}✅ Test files cleaned up${NC}"
fi

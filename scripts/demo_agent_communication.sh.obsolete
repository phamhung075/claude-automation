#!/bin/bash

###############################################################################
# Demo: Agent Communication & Shared Knowledge
#
# Demonstrates how agents communicate and share knowledge during workflow
###############################################################################

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="/tmp/agenthub_autonomous"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

echo -e "${BLUE}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Agent Communication Demo                           ║${NC}"
echo -e "${BLUE}║   Shared Knowledge System                            ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════════╝${NC}\n"

# Setup
mkdir -p "$WORK_DIR"
chmod +x "$SCRIPT_DIR/shared_knowledge_manager.sh"

# Initialize shared knowledge
echo -e "${BLUE}📚 Initializing shared knowledge base...${NC}"
"$SCRIPT_DIR/shared_knowledge_manager.sh" init
echo ""

# Simulate Agent 1: Coding Agent
echo -e "${CYAN}🤖 AGENT 1: Coding Agent${NC}"
echo -e "${YELLOW}Starting work on JWT authentication...${NC}"

# Coding agent writes discoveries
"$SCRIPT_DIR/shared_knowledge_manager.sh" register "coding-agent"
"$SCRIPT_DIR/shared_knowledge_manager.sh" update-phase "implementation"

"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-discovery "coding-agent" \
    "JWT authentication best implemented with HS256 algorithm for symmetric signing"

"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-discovery "coding-agent" \
    "Token expiry set to 1 hour for security, refresh tokens valid for 7 days"

"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-pattern "coding-agent" \
    "JWT Token Generation" \
    "import jwt; token = jwt.encode(payload, secret, algorithm='HS256')" \
    "Generate secure JWT tokens for authentication"

# Coding agent sends message to test agent
"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-message "coding-agent" "test-agent" \
    "I created JWT auth module with 3 functions: generate_token(), verify_token(), refresh_token(). Please test all three with various scenarios including expired tokens."

echo -e "${GREEN}✅ Coding agent completed work and shared knowledge${NC}"
echo ""

sleep 1

# Simulate Agent 2: Test Agent reads knowledge
echo -e "${CYAN}🤖 AGENT 2: Test Agent${NC}"
echo -e "${YELLOW}Reading shared knowledge from team...${NC}"

# Test agent reads messages
echo -e "${BLUE}📬 Messages for me:${NC}"
"$SCRIPT_DIR/shared_knowledge_manager.sh" get-messages "test-agent" | jq -r '"\(.from) → \(.to): \(.message)"'
echo ""

# Test agent reads discoveries
echo -e "${BLUE}💡 Recent team discoveries:${NC}"
"$SCRIPT_DIR/shared_knowledge_manager.sh" get-discoveries | jq -r '.[] | "  • [\(.agent)] \(.discovery)"'
echo ""

# Test agent reads code patterns
echo -e "${BLUE}📝 Code patterns to test:${NC}"
jq -r '.code_patterns[] | "  • \(.pattern_name): \(.use_case)"' "$WORK_DIR/shared_knowledge.json"
echo ""

# Test agent does work
"$SCRIPT_DIR/shared_knowledge_manager.sh" register "test-agent"
"$SCRIPT_DIR/shared_knowledge_manager.sh" update-phase "testing"

echo -e "${YELLOW}Running tests based on team knowledge...${NC}"

# Test agent adds test results
jq '.test_results += [{
    "agent": "test-agent",
    "timestamp": "'$(date -Iseconds)'",
    "tests_run": 15,
    "tests_passed": 13,
    "tests_failed": 2,
    "coverage_percentage": 87,
    "failed_tests": ["test_token_expiry", "test_refresh_rotation"],
    "failure_details": {
        "test_token_expiry": "Token expired 5 seconds too early",
        "test_refresh_rotation": "Refresh token not being rotated on use"
    }
}]' "$WORK_DIR/shared_knowledge.json" > "$WORK_DIR/.knowledge.tmp"
mv "$WORK_DIR/.knowledge.tmp" "$WORK_DIR/shared_knowledge.json"

# Test agent sends message to debugger
"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-message "test-agent" "debugger-agent" \
    "2 test failures: test_token_expiry (auth/tests/test_jwt.py:45) - token expires 5s too early, test_refresh_rotation (auth/tests/test_jwt.py:78) - refresh token not rotating"

echo -e "${GREEN}✅ Test agent ran tests and reported failures to debugger${NC}"
echo ""

sleep 1

# Simulate Agent 3: Debugger reads knowledge
echo -e "${CYAN}🤖 AGENT 3: Debugger Agent${NC}"
echo -e "${YELLOW}Reading shared knowledge to understand failures...${NC}"

# Debugger reads messages
echo -e "${BLUE}📬 Messages for me:${NC}"
"$SCRIPT_DIR/shared_knowledge_manager.sh" get-messages "debugger-agent" | jq -r '"\(.from) → \(.to): \(.message)"'
echo ""

# Debugger reads test results
echo -e "${BLUE}🧪 Test results from team:${NC}"
jq -r '.test_results[-1] | "  Tests run: \(.tests_run)\n  Passed: \(.tests_passed)\n  Failed: \(.tests_failed)\n  Coverage: \(.coverage_percentage)%"' "$WORK_DIR/shared_knowledge.json"
echo ""

echo -e "${BLUE}❌ Failed tests:${NC}"
jq -r '.test_results[-1].failure_details | to_entries[] | "  • \(.key): \(.value)"' "$WORK_DIR/shared_knowledge.json"
echo ""

# Debugger reads discoveries (to understand implementation)
echo -e "${BLUE}💡 Understanding implementation from team discoveries:${NC}"
"$SCRIPT_DIR/shared_knowledge_manager.sh" get-discoveries | jq -r '.[] | "  • \(.discovery)"'
echo ""

# Debugger does work
"$SCRIPT_DIR/shared_knowledge_manager.sh" register "debugger-agent"

echo -e "${YELLOW}Debugging based on team knowledge...${NC}"

# Debugger adds findings
"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-discovery "debugger-agent" \
    "Token expiry bug: Timer calculation used seconds instead of milliseconds"

"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-discovery "debugger-agent" \
    "Refresh rotation bug: Missing database commit after token update"

# Debugger adds blocker resolution
jq '.blockers_resolved += [{
    "agent": "debugger-agent",
    "timestamp": "'$(date -Iseconds)'",
    "bug": "test_token_expiry failure",
    "root_cause": "Timer calculation error (seconds vs milliseconds)",
    "solution": "Changed expiry calculation from time() to time() * 1000",
    "files_modified": ["auth/jwt.py:23"],
    "prevention": "Add unit test for exact expiry timing"
},
{
    "agent": "debugger-agent",
    "timestamp": "'$(date -Iseconds)'",
    "bug": "test_refresh_rotation failure",
    "root_cause": "Missing database commit",
    "solution": "Added db.session.commit() after token update",
    "files_modified": ["auth/jwt.py:67"],
    "prevention": "Always commit database changes in same function"
}]' "$WORK_DIR/shared_knowledge.json" > "$WORK_DIR/.knowledge.tmp"
mv "$WORK_DIR/.knowledge.tmp" "$WORK_DIR/shared_knowledge.json"

# Debugger warns team
"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-warning "debugger-agent" \
    "Always commit database changes immediately after writes to avoid state inconsistency" \
    "high"

# Debugger sends message to test agent
"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-message "debugger-agent" "test-agent" \
    "Both bugs fixed: 1) Timer calculation corrected in auth/jwt.py:23, 2) Added db commit in auth/jwt.py:67. Please re-run tests."

# Debugger sends message to all agents
"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-message "debugger-agent" "all" \
    "IMPORTANT: All database writes must be immediately followed by db.session.commit()"

echo -e "${GREEN}✅ Debugger fixed bugs and shared debugging insights with team${NC}"
echo ""

sleep 1

# Simulate Agent 4: Code Reviewer reads everything
echo -e "${CYAN}🤖 AGENT 4: Code Review Agent${NC}"
echo -e "${YELLOW}Reading ALL shared knowledge before review...${NC}"

# Reviewer reads messages for them
echo -e "${BLUE}📬 Messages for me:${NC}"
messages=$("$SCRIPT_DIR/shared_knowledge_manager.sh" get-messages "code-reviewer-agent" | jq -s '.')
if [ "$messages" != "[]" ]; then
    echo "$messages" | jq -r '.[] | "\(.from) → \(.to): \(.message)"'
else
    echo "  (No specific messages, but reviewing based on team knowledge)"
fi
echo ""

# Reviewer reads all discoveries
echo -e "${BLUE}💡 Team discoveries to verify in code:${NC}"
"$SCRIPT_DIR/shared_knowledge_manager.sh" get-discoveries | jq -r '.[] | "  • [\(.agent)] \(.discovery)"'
echo ""

# Reviewer reads warnings
echo -e "${BLUE}⚠️ Team warnings to check:${NC}"
"$SCRIPT_DIR/shared_knowledge_manager.sh" get-warnings | jq -r '.[] | "  • [\(.severity)] \(.warning)"'
echo ""

# Reviewer reads bug resolutions
echo -e "${BLUE}🔧 Bug fixes to verify:${NC}"
jq -r '.blockers_resolved[] | "  • \(.bug): \(.solution) (\(.files_modified[]))"' "$WORK_DIR/shared_knowledge.json"
echo ""

# Reviewer does work
"$SCRIPT_DIR/shared_knowledge_manager.sh" register "code-reviewer-agent"
"$SCRIPT_DIR/shared_knowledge_manager.sh" update-phase "review"

echo -e "${YELLOW}Reviewing code based on team's collective knowledge...${NC}"

# Reviewer adds architectural note
jq '.architecture_notes += [{
    "agent": "code-reviewer-agent",
    "timestamp": "'$(date -Iseconds)'",
    "review_summary": "Code review complete - all implementations follow established patterns",
    "quality_score": 9,
    "positive_findings": [
        "JWT implementation follows team-discovered HS256 pattern",
        "Bug fixes properly address root causes",
        "Code includes preventive measures from debugger insights"
    ],
    "suggestions": [
        "Consider adding configuration file for token expiry times",
        "Add database transaction helper to ensure commits"
    ],
    "approval_status": "approved_with_minor_suggestions"
}]' "$WORK_DIR/shared_knowledge.json" > "$WORK_DIR/.knowledge.tmp"
mv "$WORK_DIR/.knowledge.tmp" "$WORK_DIR/shared_knowledge.json"

# Reviewer sends message to all
"$SCRIPT_DIR/shared_knowledge_manager.sh" \
    add-message "code-reviewer-agent" "all" \
    "Code review complete! Excellent teamwork - everyone's discoveries and fixes were high quality. Approved with minor suggestions."

echo -e "${GREEN}✅ Code reviewer approved work based on team's shared knowledge${NC}"
echo ""

# Show final shared knowledge
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📊 FINAL SHARED KNOWLEDGE SUMMARY${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

echo -e "${CYAN}Current Phase:${NC}"
jq -r '.current_phase' "$WORK_DIR/shared_knowledge.json"
echo ""

echo -e "${CYAN}Active Agents:${NC}"
jq -r '.agents_active[]' "$WORK_DIR/shared_knowledge.json"
echo ""

echo -e "${CYAN}Total Discoveries: $(jq '.discoveries | length' "$WORK_DIR/shared_knowledge.json")${NC}"
echo -e "${CYAN}Total Warnings: $(jq '.warnings | length' "$WORK_DIR/shared_knowledge.json")${NC}"
echo -e "${CYAN}Total Code Patterns: $(jq '.code_patterns | length' "$WORK_DIR/shared_knowledge.json")${NC}"
echo -e "${CYAN}Total Communications: $(jq '.communication_log | length' "$WORK_DIR/shared_knowledge.json")${NC}"
echo -e "${CYAN}Bugs Resolved: $(jq '.blockers_resolved | length' "$WORK_DIR/shared_knowledge.json")${NC}"
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}💬 COMMUNICATION FLOW${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

jq -r '.communication_log[] | "\(.from) → \(.to):\n  \(.message)\n"' "$WORK_DIR/shared_knowledge.json"

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🎓 KEY LEARNINGS FROM WORKFLOW${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

echo -e "${GREEN}1. Coding Agent discovered:${NC}"
jq -r '.discoveries[] | select(.agent == "coding-agent") | "   • \(.discovery)"' "$WORK_DIR/shared_knowledge.json"
echo ""

echo -e "${GREEN}2. Debugger Agent discovered:${NC}"
jq -r '.discoveries[] | select(.agent == "debugger-agent") | "   • \(.discovery)"' "$WORK_DIR/shared_knowledge.json"
echo ""

echo -e "${GREEN}3. Team Warnings (for future work):${NC}"
jq -r '.warnings[] | "   • [\(.severity)] \(.warning)"' "$WORK_DIR/shared_knowledge.json"
echo ""

echo -e "${GREEN}4. Code Patterns (reusable):${NC}"
jq -r '.code_patterns[] | "   • \(.pattern_name): \(.use_case)"' "$WORK_DIR/shared_knowledge.json"
echo ""

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

echo -e "${YELLOW}Full shared knowledge saved to:${NC}"
echo -e "${CYAN}$WORK_DIR/shared_knowledge.json${NC}\n"

echo -e "${YELLOW}View complete knowledge:${NC}"
echo -e "${CYAN}jq . $WORK_DIR/shared_knowledge.json${NC}\n"

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Key Takeaways:${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "✅ Agents read shared knowledge before starting work"
echo -e "✅ Agents write discoveries for others to learn"
echo -e "✅ Agents send messages to specific agents"
echo -e "✅ Agents warn team about potential issues"
echo -e "✅ Knowledge accumulates and improves over time"
echo -e "✅ Team learns collectively, not individually"
echo ""

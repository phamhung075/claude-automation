#!/bin/bash

###############################################################################
# Shared Knowledge Manager
#
# Manages shared knowledge file for agent communication
# Agents can read/write discoveries, learnings, warnings during workflow
###############################################################################

KNOWLEDGE_FILE="/tmp/agenthub_autonomous/shared_knowledge.json"

# Initialize knowledge file
init_knowledge() {
    cat > "$KNOWLEDGE_FILE" <<'EOF'
{
  "workflow_start_time": "",
  "current_phase": "initialization",
  "agents_active": [],
  "discoveries": [],
  "warnings": [],
  "decisions": [],
  "code_patterns": [],
  "test_results": [],
  "architecture_notes": [],
  "blockers_resolved": [],
  "communication_log": []
}
EOF

    # Set timestamp
    local timestamp=$(date -Iseconds)
    jq ".workflow_start_time = \"$timestamp\"" "$KNOWLEDGE_FILE" > "$KNOWLEDGE_FILE.tmp"
    mv "$KNOWLEDGE_FILE.tmp" "$KNOWLEDGE_FILE"

    echo "✅ Shared knowledge initialized"
}

# Agent writes discovery
add_discovery() {
    local agent="$1"
    local discovery="$2"
    local timestamp=$(date -Iseconds)

    jq ".discoveries += [{
        \"agent\": \"$agent\",
        \"discovery\": \"$discovery\",
        \"timestamp\": \"$timestamp\"
    }]" "$KNOWLEDGE_FILE" > "$KNOWLEDGE_FILE.tmp"

    mv "$KNOWLEDGE_FILE.tmp" "$KNOWLEDGE_FILE"
}

# Agent writes warning
add_warning() {
    local agent="$1"
    local warning="$2"
    local severity="$3"  # low, medium, high, critical
    local timestamp=$(date -Iseconds)

    jq ".warnings += [{
        \"agent\": \"$agent\",
        \"warning\": \"$warning\",
        \"severity\": \"$severity\",
        \"timestamp\": \"$timestamp\"
    }]" "$KNOWLEDGE_FILE" > "$KNOWLEDGE_FILE.tmp"

    mv "$KNOWLEDGE_FILE.tmp" "$KNOWLEDGE_FILE"
}

# Agent writes decision
add_decision() {
    local agent="$1"
    local decision="$2"
    local rationale="$3"
    local timestamp=$(date -Iseconds)

    jq ".decisions += [{
        \"agent\": \"$agent\",
        \"decision\": \"$decision\",
        \"rationale\": \"$rationale\",
        \"timestamp\": \"$timestamp\"
    }]" "$KNOWLEDGE_FILE" > "$KNOWLEDGE_FILE.tmp"

    mv "$KNOWLEDGE_FILE.tmp" "$KNOWLEDGE_FILE"
}

# Agent writes code pattern
add_code_pattern() {
    local agent="$1"
    local pattern_name="$2"
    local pattern_code="$3"
    local use_case="$4"
    local timestamp=$(date -Iseconds)

    jq ".code_patterns += [{
        \"agent\": \"$agent\",
        \"pattern_name\": \"$pattern_name\",
        \"pattern_code\": \"$pattern_code\",
        \"use_case\": \"$use_case\",
        \"timestamp\": \"$timestamp\"
    }]" "$KNOWLEDGE_FILE" > "$KNOWLEDGE_FILE.tmp"

    mv "$KNOWLEDGE_FILE.tmp" "$KNOWLEDGE_FILE"
}

# Agent writes communication message
add_communication() {
    local from_agent="$1"
    local to_agent="$2"
    local message="$3"
    local timestamp=$(date -Iseconds)

    jq ".communication_log += [{
        \"from\": \"$from_agent\",
        \"to\": \"$to_agent\",
        \"message\": \"$message\",
        \"timestamp\": \"$timestamp\"
    }]" "$KNOWLEDGE_FILE" > "$KNOWLEDGE_FILE.tmp"

    mv "$KNOWLEDGE_FILE.tmp" "$KNOWLEDGE_FILE"
}

# Get all knowledge
get_knowledge() {
    cat "$KNOWLEDGE_FILE"
}

# Get discoveries by agent
get_discoveries_by_agent() {
    local agent="$1"
    jq ".discoveries[] | select(.agent == \"$agent\")" "$KNOWLEDGE_FILE"
}

# Get recent discoveries (last N)
get_recent_discoveries() {
    local limit="${1:-5}"
    jq ".discoveries[-$limit:]" "$KNOWLEDGE_FILE"
}

# Get all warnings
get_warnings() {
    jq ".warnings" "$KNOWLEDGE_FILE"
}

# Get critical warnings
get_critical_warnings() {
    jq '.warnings[] | select(.severity == "critical")' "$KNOWLEDGE_FILE"
}

# Get messages for specific agent
get_messages_for_agent() {
    local agent="$1"
    jq ".communication_log[] | select(.to == \"$agent\" or .to == \"all\")" "$KNOWLEDGE_FILE"
}

# Update current phase
update_phase() {
    local phase="$1"  # e.g., "design", "implementation", "testing", "review"

    jq ".current_phase = \"$phase\"" "$KNOWLEDGE_FILE" > "$KNOWLEDGE_FILE.tmp"
    mv "$KNOWLEDGE_FILE.tmp" "$KNOWLEDGE_FILE"
}

# Register active agent
register_agent() {
    local agent="$1"

    # Check if already registered
    local exists=$(jq ".agents_active[] | select(. == \"$agent\")" "$KNOWLEDGE_FILE")

    if [ -z "$exists" ]; then
        jq ".agents_active += [\"$agent\"]" "$KNOWLEDGE_FILE" > "$KNOWLEDGE_FILE.tmp"
        mv "$KNOWLEDGE_FILE.tmp" "$KNOWLEDGE_FILE"
    fi
}

# Unregister agent
unregister_agent() {
    local agent="$1"

    jq ".agents_active = [.agents_active[] | select(. != \"$agent\")]" "$KNOWLEDGE_FILE" > "$KNOWLEDGE_FILE.tmp"
    mv "$KNOWLEDGE_FILE.tmp" "$KNOWLEDGE_FILE"
}

# Export knowledge to MCP context
export_to_mcp() {
    local task_id="$1"
    local knowledge=$(cat "$KNOWLEDGE_FILE")

    curl -s -X POST "http://localhost:8000/api/manage_context" \
         -H "Content-Type: application/json" \
         -d "{
             \"action\": \"update\",
             \"level\": \"task\",
             \"context_id\": \"$task_id\",
             \"data\": $(echo "$knowledge" | jq -c .)
         }"
}

# Main command handler
case "${1:-}" in
    init)
        init_knowledge
        ;;
    add-discovery)
        add_discovery "$2" "$3"
        ;;
    add-warning)
        add_warning "$2" "$3" "$4"
        ;;
    add-decision)
        add_decision "$2" "$3" "$4"
        ;;
    add-pattern)
        add_code_pattern "$2" "$3" "$4" "$5"
        ;;
    add-message)
        add_communication "$2" "$3" "$4"
        ;;
    get)
        get_knowledge
        ;;
    get-discoveries)
        if [ -n "$2" ]; then
            get_discoveries_by_agent "$2"
        else
            get_recent_discoveries 10
        fi
        ;;
    get-warnings)
        get_warnings
        ;;
    get-critical)
        get_critical_warnings
        ;;
    get-messages)
        get_messages_for_agent "$2"
        ;;
    update-phase)
        update_phase "$2"
        ;;
    register)
        register_agent "$2"
        ;;
    unregister)
        unregister_agent "$2"
        ;;
    export-mcp)
        export_to_mcp "$2"
        ;;
    *)
        echo "Usage: $0 {init|add-discovery|add-warning|add-decision|add-pattern|add-message|get|get-discoveries|get-warnings|get-critical|get-messages|update-phase|register|unregister|export-mcp}"
        exit 1
        ;;
esac

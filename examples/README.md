# Example Workflow Configurations

This directory contains example JSON configuration files for starting autonomous agent workflows. These examples demonstrate different types of tasks and show how to structure workflow configurations.

## Quick Start

Use any example configuration with the workflow starter script:

```bash
./scripts/start_autonomous_workflow.sh examples/example-workflow-auth.json
```

## Available Examples

### 1. Authentication System (`example-workflow-auth.json`)
**Type:** New Feature Implementation
**Complexity:** High
**Duration:** ~24 hours
**Agents:** 7 specialized agents

Demonstrates building a complete JWT authentication system with:
- Token generation and refresh rotation
- Password hashing and validation
- Full unit test coverage
- Security audit
- API documentation

**Best for:** Learning how to structure complex, multi-agent workflows

---

### 2. Simple Calculator (`example-workflow-simple-calculator.json`)
**Type:** Basic Application
**Complexity:** Low
**Duration:** ~4 hours
**Agents:** 3 agents

Demonstrates building a simple calculator with:
- Basic arithmetic operations
- Input validation
- Command-line interface
- Unit tests

**Best for:** Testing the system with a quick, straightforward task

---

### 3. API Refactoring (`example-workflow-api-refactor.json`)
**Type:** Code Optimization
**Complexity:** Medium
**Duration:** ~16 hours
**Agents:** 6 agents

Demonstrates refactoring existing code to:
- Eliminate code duplication
- Optimize database queries
- Improve maintainability
- Maintain backward compatibility

**Best for:** Improving existing codebases without breaking changes

---

### 4. Bug Fix (`example-workflow-bug-fix.json`)
**Type:** Investigation & Fix
**Complexity:** Medium
**Duration:** ~8 hours
**Agents:** 6 agents

Demonstrates debugging and fixing critical issues:
- Root cause analysis
- Memory leak investigation
- Regression test creation
- Postmortem documentation

**Best for:** Urgent production issues requiring fast resolution

## Configuration Structure

Each configuration file follows this structure:

```json
{
  "workflow_name": "Descriptive name for logs",
  "project_id": "Your MCP project UUID",
  "git_branch_name": "feature/branch-name",
  "goal": "High-level description of what to accomplish",

  "requirements": [
    "Specific requirement 1",
    "Specific requirement 2"
  ],

  "constraints": {
    "max_time_hours": 24,
    "must_pass_tests": true,
    "require_security_audit": false
  },

  "agents_to_use": [
    "agent-name-1",
    "agent-name-2"
  ],

  "expected_deliverables": {
    "code_files": [],
    "test_files": [],
    "documentation": []
  },

  "success_criteria": {
    "all_tests_passing": true,
    "test_coverage": ">=80%"
  }
}
```

## Customizing for Your Project

1. **Copy an example** that matches your task type
2. **Update project_id** with your MCP project UUID
3. **Change git_branch_name** to your target branch
4. **Modify goal and requirements** to match your needs
5. **Adjust agents_to_use** based on work needed
6. **Set constraints** appropriate for your timeline

## Finding Your Project ID

```bash
# List all projects
curl -X POST http://localhost:8000/api/manage_project \
  -H "Content-Type: application/json" \
  -d '{"action": "list"}'
```

## Available Agents

Common agent types you can use in configurations:

- `system-architect-agent` - High-level system design
- `coding-agent` - Primary development work
- `test-orchestrator-agent` - Test creation and execution
- `debugger-agent` - Bug investigation and fixes
- `security-auditor-agent` - Security vulnerability scanning
- `code-reviewer-agent` - Code quality review
- `documentation-agent` - Documentation creation
- `efficiency-optimization-agent` - Performance optimization
- `root-cause-analysis-agent` - Deep issue investigation

See `CLAUDE.md` in project root for the complete list of 31 specialized agents.

## Best Practices

1. **Start simple** - Use the calculator example first to validate your setup
2. **Be specific** - Detailed requirements lead to better results
3. **Set realistic time constraints** - Complex tasks need adequate time
4. **Choose appropriate agents** - Match agents to the work type
5. **Define success criteria** - Clear metrics help agents know when done
6. **Version control configs** - Keep successful configurations for reuse

## Monitoring Progress

While a workflow runs, monitor progress with:

```bash
# Watch file changes
watch -n 2 'ls -lh /tmp/agenthub_autonomous'

# View agent logs
tail -f /tmp/agenthub_autonomous/agent_raw_output.log

# Check shared knowledge
jq . /tmp/agenthub_autonomous/shared_knowledge.json
```

## Troubleshooting

**Configuration not loading:**
- Check JSON syntax: `jq empty your-config.json`
- Verify all required fields are present

**Agents not working:**
- Ensure MCP API is running: `curl http://localhost:8000/health`
- Verify project_id exists in your MCP database

**Workflow stuck:**
- Check `/tmp/agenthub_autonomous/*.flag` files
- Review agent logs for errors
- Verify git branch exists

## Creating New Examples

When creating new example configs:

1. Use descriptive workflow_name for logs
2. Include realistic time estimates
3. List all required deliverables
4. Define measurable success criteria
5. Choose minimum necessary agents
6. Document in this README

## Support

For issues or questions:
- Check the main `README.md` for system setup
- Review `QUICK_REFERENCE.md` for command reference
- See `docs/usage-guide.md` for detailed instructions

# Autonomous Agent Orchestration System

**Date**: 2025-11-14
**Status**: 🔵 Design Phase
**Purpose**: Build fully autonomous multi-agent system that runs without human intervention

---

## Executive Summary

Transform agenthub from **human-orchestrated** to **autonomous agent system** that:

1. **Runs 24/7** as background workers (no terminal dependency)
2. **Generates workflows** automatically (agents create task graphs)
3. **Self-coordinates** between agents (event-driven communication)
4. **Loops until complete** (continues until all tests pass)
5. **Human intervenes only** when blocked or incorrect

---

## Architecture Overview

### High-Level Components

```
┌──────────────────────────────────────────────────────────┐
│  1. HUMAN INTERFACE LAYER                                │
│     • Web UI: Submit goals, monitor progress             │
│     • API: POST /goals {"goal": "Build auth system"}     │
│     • Intervention: Only when agents request help        │
└────────────────────┬─────────────────────────────────────┘
                     │
┌────────────────────▼─────────────────────────────────────┐
│  2. MASTER ORCHESTRATOR SERVICE                          │
│     • Background daemon (systemd/Docker service)         │
│     • Polls for new goals from database                  │
│     • Generates workflow graphs using AI planning        │
│     • Monitors task completion events                    │
│     • Creates new tasks based on results                 │
│     • Loop: while(tasks_incomplete) { coordinate() }     │
└────────────────────┬─────────────────────────────────────┘
                     │
┌────────────────────▼─────────────────────────────────────┐
│  3. TASK QUEUE (Redis Queue / Celery)                    │
│     • Priority queue: blocking > in_progress > pending   │
│     • Task metadata: dependencies, retry count, timeout  │
│     • Dead letter queue for failed tasks                 │
└────────────────────┬─────────────────────────────────────┘
                     │
        ┌────────────┼────────────┬────────────┐
        │            │            │            │
┌───────▼──────┐ ┌──▼────────┐ ┌─▼─────────┐ ┌▼──────────┐
│ 4. AGENT     │ │ AGENT     │ │ AGENT     │ │ AGENT     │
│    WORKER 1  │ │ WORKER 2  │ │ WORKER 3  │ │ WORKER N  │
│              │ │           │ │           │ │           │
│ - Coding     │ │ - Testing │ │ - Review  │ │ - Debug   │
│ - Poll queue │ │ - Execute │ │ - Report  │ │ - Repeat  │
└───────┬──────┘ └──┬────────┘ └─┬─────────┘ └┬──────────┘
        │           │            │            │
        └───────────┼────────────┴────────────┘
                    │
┌───────────────────▼──────────────────────────────────────┐
│  5. EVENT BUS (Redis Pub/Sub / RabbitMQ)                 │
│     • task.completed events                              │
│     • agent.blocked events (requires human intervention) │
│     • workflow.finished events                           │
└────────────────────┬─────────────────────────────────────┘
                     │
┌────────────────────▼─────────────────────────────────────┐
│  6. STATE MANAGEMENT (PostgreSQL + Redis)                │
│     • Task states: pending → in_progress → done          │
│     • Workflow graphs: DAG of task dependencies          │
│     • Agent context: Shared knowledge between agents     │
│     • Checkpoint system: Resume after crashes            │
└──────────────────────────────────────────────────────────┘
```

---

## Component Details

### 1. Goal Input System

**Human submits high-level goal via API:**

```python
# Web UI or API call
POST /api/v1/goals
{
  "goal": "Build complete authentication system",
  "requirements": [
    "JWT tokens",
    "Refresh token rotation",
    "Password hashing",
    "Unit tests >80% coverage",
    "Security audit",
    "API documentation"
  ],
  "constraints": {
    "max_time": "24 hours",
    "must_pass_tests": true,
    "require_security_audit": true
  }
}

# Response:
{
  "goal_id": "goal-uuid-123",
  "status": "accepted",
  "estimated_completion": "2025-11-14T20:00:00Z",
  "workflow_generated": true
}
```

### 2. Master Orchestrator Service

**Core autonomous loop:**

```python
# File: agenthub_main/src/autonomous_orchestrator/master_service.py

import asyncio
from typing import List, Dict
from agenthub.mcp import manage_task, manage_subtask

class AutonomousMasterOrchestrator:
    """
    Runs 24/7 as background service
    Generates workflows and coordinates agents autonomously
    """

    def __init__(self):
        self.running = True
        self.active_workflows: Dict[str, WorkflowState] = {}

    async def main_loop(self):
        """
        Infinite loop - runs until explicitly stopped
        """
        while self.running:
            # Step 1: Check for new goals from humans
            new_goals = await self.fetch_pending_goals()
            for goal in new_goals:
                await self.generate_workflow(goal)

            # Step 2: Monitor active workflows
            for workflow_id, workflow in self.active_workflows.items():
                await self.advance_workflow(workflow)

            # Step 3: Check for completed/blocked workflows
            await self.cleanup_completed_workflows()

            # Sleep briefly to avoid CPU spinning
            await asyncio.sleep(1)  # Check every second

    async def generate_workflow(self, goal: Goal) -> WorkflowGraph:
        """
        Uses AI to generate task graph for achieving goal

        This is the KEY function - it creates the workflow automatically!
        """
        # Step 1: Analyze goal complexity using AI
        analysis_prompt = f"""
        Analyze this goal and break it into subtasks:

        Goal: {goal.description}
        Requirements: {goal.requirements}

        Generate a task graph with:
        - Task names
        - Dependencies (which tasks must complete first)
        - Estimated complexity
        - Recommended agent for each task

        Return as JSON task graph.
        """

        # Use Claude to generate workflow
        workflow_plan = await self.call_ai_planner(analysis_prompt)

        # Step 2: Create MCP tasks from workflow plan
        parent_task = await manage_task(
            action="create",
            git_branch_id=goal.branch_id,
            title=goal.description,
            assignees="master-orchestrator-agent",
            details=goal.requirements
        )

        # Step 3: Create subtasks with dependencies
        task_graph = []
        for task_node in workflow_plan.tasks:
            subtask = await manage_subtask(
                action="create",
                task_id=parent_task.id,
                title=task_node.title,
                assignees=task_node.recommended_agent,
                dependencies=task_node.dependencies,  # Key: dependency links!
                progress_notes=f"Auto-generated by orchestrator"
            )
            task_graph.append(subtask)

        # Step 4: Register workflow
        workflow = WorkflowState(
            goal_id=goal.id,
            parent_task_id=parent_task.id,
            task_graph=task_graph,
            status="running"
        )
        self.active_workflows[goal.id] = workflow

        # Step 5: Enqueue first tasks (those with no dependencies)
        initial_tasks = [t for t in task_graph if not t.dependencies]
        for task in initial_tasks:
            await self.enqueue_task(task)

        return workflow

    async def advance_workflow(self, workflow: WorkflowState):
        """
        Check workflow progress and enqueue next tasks

        This is the LOOP that keeps agents working!
        """
        # Step 1: Get all subtasks
        subtasks = await manage_subtask(
            action="list",
            task_id=workflow.parent_task_id
        )

        # Step 2: Check for completed tasks
        completed = [t for t in subtasks if t.status == "done"]
        in_progress = [t for t in subtasks if t.status == "in_progress"]
        pending = [t for t in subtasks if t.status == "todo"]
        blocked = [t for t in subtasks if t.status == "blocked"]

        # Step 3: Find tasks that can now run (dependencies met)
        for task in pending:
            deps_met = all(
                dep.status == "done"
                for dep in subtasks
                if dep.id in task.dependencies
            )
            if deps_met:
                # Dependencies satisfied - enqueue this task!
                await self.enqueue_task(task)

        # Step 4: Check if workflow is complete
        if len(completed) == len(subtasks):
            # All done! Run final validation
            await self.validate_and_complete_workflow(workflow)

        # Step 5: Check for blocked tasks (need human intervention)
        if blocked:
            await self.request_human_intervention(workflow, blocked)

    async def validate_and_complete_workflow(self, workflow: WorkflowState):
        """
        Final quality gate before marking workflow complete
        """
        # Step 1: Run all tests
        test_results = await self.run_all_tests(workflow)

        if test_results.all_passed:
            # Step 2: Security audit
            security_results = await self.run_security_audit(workflow)

            if security_results.passed:
                # Step 3: Code review
                review_results = await self.run_final_review(workflow)

                if review_results.approved:
                    # ✅ COMPLETE! Mark workflow as done
                    await manage_task(
                        action="complete",
                        task_id=workflow.parent_task_id,
                        completion_summary="All tasks completed, tests passed, security audit passed"
                    )

                    # Notify human
                    await self.notify_human_completion(workflow)

                    # Remove from active workflows
                    del self.active_workflows[workflow.goal_id]
                else:
                    # Code review failed - create rework task
                    await self.create_rework_tasks(workflow, review_results.issues)
        else:
            # Tests failed - create debug task
            await self.create_debug_tasks(workflow, test_results.failures)

    async def enqueue_task(self, task: Subtask):
        """
        Add task to worker queue for execution
        """
        await redis_queue.enqueue({
            "task_id": task.id,
            "agent_type": task.assignees,
            "priority": task.priority,
            "retry_count": 0,
            "max_retries": 3
        })

    async def request_human_intervention(self, workflow: WorkflowState, blocked_tasks: List[Subtask]):
        """
        Agent is stuck - notify human
        """
        notification = {
            "type": "intervention_required",
            "workflow_id": workflow.goal_id,
            "blocked_tasks": [
                {
                    "task_id": t.id,
                    "title": t.title,
                    "blocker_reason": t.blocker_details
                }
                for t in blocked_tasks
            ],
            "message": "Agents need your input to continue"
        }

        # Send to Web UI via WebSocket
        await websocket_broadcast(notification)

        # Send email notification
        await send_email_notification(notification)

        # Update workflow status
        workflow.status = "awaiting_human"


# Entry point - runs as daemon
if __name__ == "__main__":
    orchestrator = AutonomousMasterOrchestrator()
    asyncio.run(orchestrator.main_loop())
```

### 3. Agent Worker Pool

**Multiple worker processes poll queue and execute tasks:**

```python
# File: agenthub_main/src/autonomous_orchestrator/agent_worker.py

import asyncio
from redis import Redis
from anthropic import Anthropic

class AutonomousAgentWorker:
    """
    Worker process that executes tasks from queue
    Can be scaled horizontally (run multiple instances)
    """

    def __init__(self, agent_type: str):
        self.agent_type = agent_type  # e.g., "coding-agent"
        self.running = True
        self.anthropic_client = Anthropic()  # Uses API or Claude Code SDK

    async def main_loop(self):
        """
        Poll queue and execute tasks
        """
        while self.running:
            # Step 1: Get next task from queue (blocking wait)
            task = await self.fetch_next_task()

            if task:
                try:
                    # Step 2: Execute task
                    result = await self.execute_task(task)

                    # Step 3: Report completion
                    await self.report_completion(task, result)

                    # Step 4: Publish completion event
                    await self.publish_event("task.completed", {
                        "task_id": task.id,
                        "agent": self.agent_type,
                        "result": result
                    })

                except Exception as e:
                    # Step 5: Handle failure
                    await self.handle_failure(task, e)

            # Brief sleep to avoid tight loop
            await asyncio.sleep(0.1)

    async def execute_task(self, task: Dict) -> Dict:
        """
        Execute task using AI agent

        THIS IS WHERE THE ACTUAL WORK HAPPENS!
        """
        # Step 1: Load task context from MCP
        task_context = await manage_task(
            action="get",
            task_id=task["task_id"],
            include_context="true"  # Get full context
        )

        # Step 2: Load agent system prompt
        agent_config = await call_agent(self.agent_type)

        # Step 3: Build prompt with full context
        prompt = f"""
        {agent_config.system_prompt}

        TASK:
        {task_context.title}

        DETAILS:
        {task_context.details}

        CONTEXT FROM PREVIOUS AGENTS:
        {task_context.inherited_context}

        Execute this task autonomously. Report progress in JSON format.
        """

        # Step 4: Call AI model
        response = await self.anthropic_client.messages.create(
            model="claude-3-5-sonnet-20241022",
            messages=[{"role": "user", "content": prompt}],
            max_tokens=4000,
            temperature=0.7
        )

        # Step 5: Parse result
        result = self.parse_agent_response(response)

        # Step 6: Update task progress
        await manage_subtask(
            action="update",
            task_id=task_context.parent_id,
            subtask_id=task["task_id"],
            progress_percentage=100,
            progress_notes=f"Completed by {self.agent_type}: {result.summary}"
        )

        return result

    async def report_completion(self, task: Dict, result: Dict):
        """
        Mark task as complete in database
        """
        await manage_subtask(
            action="complete",
            task_id=task["parent_task_id"],
            subtask_id=task["task_id"],
            completion_summary=result["summary"],
            testing_notes=result.get("tests_run", "N/A"),
            insights_found=result.get("insights", [])
        )

    async def handle_failure(self, task: Dict, error: Exception):
        """
        Handle task failure - retry or escalate
        """
        retry_count = task.get("retry_count", 0)

        if retry_count < task.get("max_retries", 3):
            # Retry task
            task["retry_count"] = retry_count + 1
            await redis_queue.enqueue(task)
        else:
            # Max retries exceeded - mark as blocked
            await manage_subtask(
                action="update",
                task_id=task["parent_task_id"],
                subtask_id=task["task_id"],
                status="blocked",
                progress_notes=f"Failed after {retry_count} retries: {str(error)}"
            )

            # Notify orchestrator
            await self.publish_event("task.blocked", {
                "task_id": task["task_id"],
                "error": str(error),
                "retry_count": retry_count
            })


# Run multiple workers for different agent types
if __name__ == "__main__":
    import sys
    agent_type = sys.argv[1]  # e.g., "coding-agent"

    worker = AutonomousAgentWorker(agent_type)
    asyncio.run(worker.main_loop())
```

### 4. Docker Compose Configuration

**Run entire system as background services:**

```yaml
# File: docker-compose.autonomous.yml

version: '3.8'

services:
  # Master orchestrator (1 instance)
  orchestrator:
    build:
      context: ./agenthub_main
      dockerfile: Dockerfile.orchestrator
    environment:
      - DATABASE_URL=postgresql://user:pass@postgres:5432/agenthub
      - REDIS_URL=redis://redis:6379/0
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - postgres
      - redis
    restart: always
    command: python -m autonomous_orchestrator.master_service

  # Agent workers (scale horizontally!)
  coding-agent-worker:
    build:
      context: ./agenthub_main
      dockerfile: Dockerfile.worker
    environment:
      - AGENT_TYPE=coding-agent
      - REDIS_URL=redis://redis:6379/0
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - redis
      - orchestrator
    restart: always
    deploy:
      replicas: 3  # Run 3 coding workers in parallel!
    command: python -m autonomous_orchestrator.agent_worker coding-agent

  test-agent-worker:
    build:
      context: ./agenthub_main
      dockerfile: Dockerfile.worker
    environment:
      - AGENT_TYPE=test-orchestrator-agent
      - REDIS_URL=redis://redis:6379/0
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - redis
      - orchestrator
    restart: always
    deploy:
      replicas: 2  # Run 2 test workers
    command: python -m autonomous_orchestrator.agent_worker test-orchestrator-agent

  review-agent-worker:
    build:
      context: ./agenthub_main
      dockerfile: Dockerfile.worker
    environment:
      - AGENT_TYPE=code-reviewer-agent
      - REDIS_URL=redis://redis:6379/0
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - redis
      - orchestrator
    restart: always
    command: python -m autonomous_orchestrator.agent_worker code-reviewer-agent

  debug-agent-worker:
    build:
      context: ./agenthub_main
      dockerfile: Dockerfile.worker
    environment:
      - AGENT_TYPE=debugger-agent
      - REDIS_URL=redis://redis:6379/0
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - redis
      - orchestrator
    restart: always
    command: python -m autonomous_orchestrator.agent_worker debugger-agent

  # Infrastructure
  postgres:
    image: postgres:15
    environment:
      - POSTGRES_DB=agenthub
      - POSTGRES_USER=agenthub_user
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: always

  redis:
    image: redis:7
    restart: always

  # Web UI (for monitoring)
  web-ui:
    build:
      context: ./agenthub-frontend
    ports:
      - "3800:3800"
    environment:
      - API_URL=http://orchestrator:8000
    depends_on:
      - orchestrator
    restart: always

volumes:
  postgres_data:
```

**Start entire autonomous system:**

```bash
# Start all services (orchestrator + workers)
docker-compose -f docker-compose.autonomous.yml up -d

# Scale workers dynamically
docker-compose -f docker-compose.autonomous.yml up -d --scale coding-agent-worker=5

# Monitor logs
docker-compose -f docker-compose.autonomous.yml logs -f orchestrator
docker-compose -f docker-compose.autonomous.yml logs -f coding-agent-worker
```

---

## Usage Flow

### 1. Human Submits Goal

```bash
# Via API
curl -X POST http://localhost:8000/api/v1/goals \
  -H "Content-Type: application/json" \
  -d '{
    "goal": "Build authentication system with JWT",
    "requirements": ["Unit tests", "Security audit", "Documentation"]
  }'

# Response:
{
  "goal_id": "goal-123",
  "status": "workflow_generated",
  "estimated_tasks": 12,
  "estimated_time": "4 hours"
}
```

### 2. System Works Autonomously

```
[09:00:00] Orchestrator: Generated workflow with 12 tasks
[09:00:01] Orchestrator: Enqueued tasks: analyze-requirements, design-architecture
[09:00:02] Worker-1 (system-architect): Started task analyze-requirements
[09:05:23] Worker-1: Completed analysis, found 3 core components
[09:05:24] Orchestrator: Analysis complete, enqueuing: implement-jwt-module
[09:05:25] Worker-2 (coding-agent): Started implementing JWT module
[09:15:47] Worker-2: Completed JWT module (auth/jwt.py, 245 lines)
[09:15:48] Orchestrator: Implementation complete, enqueuing: write-tests
[09:15:49] Worker-3 (test-agent): Started writing tests
[09:20:12] Worker-3: Tests written (87% coverage), running tests...
[09:20:45] Worker-3: ❌ 2 tests failed
[09:20:46] Orchestrator: Tests failed, enqueuing: debug-test-failures
[09:20:47] Worker-4 (debugger-agent): Started debugging
[09:25:33] Worker-4: Found issue in token refresh logic (auth/jwt.py:123)
[09:25:34] Orchestrator: Bug found, enqueuing: fix-token-refresh
[09:25:35] Worker-2 (coding-agent): Started fixing bug
[09:30:21] Worker-2: Bug fixed, running tests again...
[09:30:55] Worker-3: ✅ All tests passing!
[09:30:56] Orchestrator: Tests passed, enqueuing: security-audit
[09:31:00] Worker-5 (security-agent): Started security audit
[09:35:42] Worker-5: Security audit passed, no vulnerabilities
[09:35:43] Orchestrator: Security passed, enqueuing: code-review
[09:35:44] Worker-6 (reviewer-agent): Started final review
[09:40:15] Worker-6: Code review approved, ready for deployment
[09:40:16] Orchestrator: ✅ All tasks complete! Notifying human...
```

### 3. Human Receives Notification

```
📧 Email: "Authentication system completed successfully"
🔔 Web UI: "Goal 'Build authentication system' is ready for review"

Human logs into Web UI:
- Sees all task history
- Reviews code changes
- Runs manual tests
- Approves or requests changes
```

### 4. If Human Requests Changes

```bash
# Human clicks "Request changes" in UI with feedback
POST /api/v1/goals/goal-123/rework
{
  "feedback": "Please add 2FA support",
  "priority": "high"
}

# System automatically:
[10:00:00] Orchestrator: Human requested rework, generating new tasks
[10:00:01] Orchestrator: Enqueued: implement-2fa-support
[10:00:02] Worker-1 (coding-agent): Started implementing 2FA...
# ... loop continues autonomously ...
```

---

## Key Differences from Current System

| Feature | Current agenthub | Autonomous System |
|---------|-----------------|-------------------|
| **Human Role** | Orchestrates every step | Submits goals, monitors, intervenes when needed |
| **Agent Execution** | Sequential, wait for human | Parallel workers, autonomous loop |
| **Session Dependency** | Requires Claude Code terminal | Background services (Docker) |
| **Workflow Generation** | Human designs workflow | AI generates workflow automatically |
| **Error Handling** | Human fixes issues | Auto-retry, auto-debug, only escalate when stuck |
| **Scalability** | 1 agent at a time | N workers in parallel |
| **Persistence** | Session-based | 24/7 daemon |
| **Stopping Condition** | Human ends session | Workflow complete OR blocked |

---

## Implementation Roadmap

### Phase 1: Core Infrastructure (Week 1-2)
- [ ] Redis task queue setup
- [ ] Worker pool architecture
- [ ] Event bus (Redis pub/sub)
- [ ] Background orchestrator service

### Phase 2: Workflow Generation (Week 3-4)
- [ ] AI-powered workflow planner
- [ ] Task dependency graph builder
- [ ] Dynamic task enqueuing
- [ ] Completion validation

### Phase 3: Agent Workers (Week 5-6)
- [ ] Agent worker processes
- [ ] Task execution engine
- [ ] Result reporting
- [ ] Failure handling & retry

### Phase 4: Autonomous Loop (Week 7-8)
- [ ] Workflow advancement logic
- [ ] Test-driven completion validation
- [ ] Auto-debug on failures
- [ ] Human intervention requests

### Phase 5: Monitoring & UI (Week 9-10)
- [ ] Real-time progress dashboard
- [ ] Task history visualization
- [ ] Intervention interface
- [ ] Workflow analytics

---

## Cost Considerations

### API Usage
- Each agent call uses Anthropic API (if using API key)
- Autonomous loop → many API calls
- **Solution**: Use Claude Code subscription if possible, or implement caching

### Infrastructure
- Redis (lightweight)
- PostgreSQL (already have)
- Worker processes (CPU/memory based on agent count)

**Estimated Costs**:
- **API calls**: 100-500 calls per workflow (depends on complexity)
- **Infrastructure**: $50-100/month for small scale (5-10 workers)

---

## Next Steps

1. **Decide on LLM access method**:
   - Option A: Anthropic API (requires API key, costs money)
   - Option B: Claude Code SDK (uses subscription, need to check if supports background processes)
   - Option C: Local LLM (Ollama, LLaMA) - free but lower quality

2. **Start with Phase 1**:
   - Set up Redis queue
   - Build basic worker architecture
   - Test with simple task execution

3. **Iterate**:
   - Start simple (1 worker, manual workflow)
   - Add autonomous features gradually
   - Test thoroughly at each phase

---

**Ready to start building?** Let me know which phase you want to tackle first!

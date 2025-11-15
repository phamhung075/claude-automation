#!/usr/bin/env node

/**
 * WebSocket Auto-Input Coordinator - Simple Proof of Concept
 *
 * Demonstrates:
 * 1. Connecting to AgentHub WebSocket
 * 2. Receiving real-time events
 * 3. Building context from events
 * 4. (Simulated) Injecting into Claude sessions
 *
 * Usage:
 *   node simple-poc.js [ws://localhost:8000/ws]
 */

const WebSocket = require('ws');
const fs = require('fs');
const path = require('path');

const WS_URL = process.argv[2] || 'ws://localhost:8000/ws';
const WORK_DIR = '/tmp/agenthub_autonomous';

// Ensure work directory exists
if (!fs.existsSync(WORK_DIR)) {
  fs.mkdirSync(WORK_DIR, { recursive: true });
}

console.log(`
╔═══════════════════════════════════════════════════════╗
║   WebSocket Auto-Input Coordinator - POC             ║
║   Connecting to: ${WS_URL.padEnd(34)} ║
╚═══════════════════════════════════════════════════════╝
`);

// Simple event router
const eventHistory = [];
const dependencyGraph = new Map();
const agentQueues = new Map();

// Connect to WebSocket
const ws = new WebSocket(WS_URL);

ws.on('open', () => {
  console.log('✅ Connected to AgentHub WebSocket\n');

  // Subscribe to events
  ws.send(JSON.stringify({
    type: 'subscribe',
    channels: ['tasks', 'subtasks', 'agents']
  }));

  console.log('📡 Subscribed to channels: tasks, subtasks, agents\n');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
  console.log('Listening for events... (Press Ctrl+C to stop)\n');
});

ws.on('message', (data) => {
  try {
    const event = JSON.parse(data.toString());
    handleEvent(event);
  } catch (err) {
    console.error('Failed to parse message:', err);
  }
});

ws.on('error', (err) => {
  console.error('❌ WebSocket error:', err.message);
});

ws.on('close', () => {
  console.log('\n🔌 WebSocket disconnected');
  saveEventHistory();
  process.exit(0);
});

function handleEvent(event) {
  const timestamp = new Date().toISOString();

  // Store in history
  eventHistory.push({ ...event, received_at: timestamp });

  // Display event
  console.log(`━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`);
  console.log(`📨 Event: ${event.event_type}`);
  console.log(`⏰ Time: ${timestamp}`);

  // Route by type
  switch (event.event_type) {
    case 'task.created':
      handleTaskCreated(event);
      break;

    case 'subtask.created':
      handleSubtaskCreated(event);
      break;

    case 'subtask.updated':
      handleSubtaskUpdated(event);
      break;

    case 'subtask.completed':
      handleSubtaskCompleted(event);
      break;

    default:
      console.log(`📄 Payload:`, JSON.stringify(event.payload, null, 2));
  }

  console.log(''); // Blank line for readability
}

function handleTaskCreated(event) {
  const { title, id } = event.payload;
  console.log(`📋 Task Created: ${title}`);
  console.log(`🆔 Task ID: ${id}`);
}

function handleSubtaskCreated(event) {
  const { id, title, assignees, dependencies } = event.payload;

  console.log(`📝 Subtask: ${title}`);
  console.log(`🤖 Agents: ${assignees.join(', ')}`);
  console.log(`🔗 Dependencies: ${dependencies?.length || 0} tasks`);

  // Build dependency graph
  dependencyGraph.set(id, {
    title,
    assignees,
    dependencies: dependencies || [],
    dependents: [],
    status: 'pending'
  });

  // Update dependents for upstream tasks
  for (const depId of (dependencies || [])) {
    if (dependencyGraph.has(depId)) {
      dependencyGraph.get(depId).dependents.push(id);
    }
  }

  // Check if ready to start (no dependencies)
  if (!dependencies || dependencies.length === 0) {
    console.log(`✅ Ready to start immediately!`);

    // Simulate: Would inject into Claude session here
    for (const agent of assignees) {
      simulateContextInjection(agent, {
        type: 'task_ready',
        subtask_id: id,
        title,
        context: 'No dependencies - ready to start'
      });
    }
  } else {
    console.log(`⏸️ Waiting for ${dependencies.length} dependencies to complete`);
  }
}

function handleSubtaskUpdated(event) {
  const { id, progress_percentage, details } = event.payload;

  console.log(`🔄 Subtask Updated: ${id}`);
  console.log(`📊 Progress: ${progress_percentage}%`);
  if (details) {
    console.log(`📝 Details: ${details}`);
  }

  // Update dependency graph
  if (dependencyGraph.has(id)) {
    dependencyGraph.get(id).progress = progress_percentage;
  }
}

function handleSubtaskCompleted(event) {
  const { id, completion_summary, insights_found } = event.payload;

  console.log(`✅ Subtask Completed: ${id}`);
  console.log(`📄 Summary: ${completion_summary}`);

  if (insights_found && insights_found.length > 0) {
    console.log(`💡 Insights:`);
    insights_found.forEach(insight => {
      console.log(`   • ${insight}`);
    });
  }

  // Update dependency graph
  if (dependencyGraph.has(id)) {
    const node = dependencyGraph.get(id);
    node.status = 'completed';

    console.log(`\n🔗 Checking ${node.dependents.length} dependent task(s)...`);

    // Check dependent tasks
    for (const dependentId of node.dependents) {
      const canStart = checkDependenciesMet(dependentId);

      if (canStart) {
        const depNode = dependencyGraph.get(dependentId);
        console.log(`\n🚀 Triggering dependent task: ${depNode.title}`);

        // Build rich context
        const context = buildCompletionContext(id, {
          completion_summary,
          insights_found
        });

        // Simulate: Would inject into Claude session here
        for (const agent of depNode.assignees) {
          simulateContextInjection(agent, {
            type: 'dependency_completed',
            subtask_id: dependentId,
            upstream_task: id,
            context
          });
        }
      }
    }
  }
}

function checkDependenciesMet(subtaskId) {
  const node = dependencyGraph.get(subtaskId);
  if (!node || !node.dependencies) return true;

  // Check if all dependencies are completed
  for (const depId of node.dependencies) {
    const depNode = dependencyGraph.get(depId);
    if (!depNode || depNode.status !== 'completed') {
      return false;
    }
  }

  return true;
}

function buildCompletionContext(subtaskId, { completion_summary, insights_found }) {
  const node = dependencyGraph.get(subtaskId);

  return {
    upstream_title: node?.title || 'Unknown',
    summary: completion_summary,
    insights: insights_found || [],
    timestamp: new Date().toISOString()
  };
}

function simulateContextInjection(agentType, contextData) {
  console.log(`\n💉 [SIMULATED] Context injection for ${agentType}:`);
  console.log(`   Type: ${contextData.type}`);

  // Enqueue for agent
  if (!agentQueues.has(agentType)) {
    agentQueues.set(agentType, []);
  }

  agentQueues.get(agentType).push({
    ...contextData,
    enqueued_at: new Date().toISOString()
  });

  // Write to file (for actual Claude sessions to read)
  const queueFile = path.join(WORK_DIR, `${agentType}_queue.json`);
  fs.writeFileSync(
    queueFile,
    JSON.stringify(agentQueues.get(agentType), null, 2)
  );

  console.log(`   ✅ Written to: ${queueFile}`);

  // In real implementation, this would do:
  // 1. Find running Claude session for agentType
  // 2. Write to session stdin pipe
  // 3. Claude receives context in real-time and continues work
  console.log(`   [In production: Would inject into claude -p session via stdin]`);
}

function saveEventHistory() {
  const historyFile = path.join(WORK_DIR, 'event_history.json');
  fs.writeFileSync(historyFile, JSON.stringify(eventHistory, null, 2));
  console.log(`\n💾 Saved ${eventHistory.length} events to: ${historyFile}`);

  // Save dependency graph
  const graphFile = path.join(WORK_DIR, 'dependency_graph.json');
  const graphData = {};
  for (const [id, node] of dependencyGraph) {
    graphData[id] = node;
  }
  fs.writeFileSync(graphFile, JSON.stringify(graphData, null, 2));
  console.log(`📊 Saved dependency graph to: ${graphFile}`);
}

// Graceful shutdown
process.on('SIGINT', () => {
  console.log('\n\n🛑 Shutting down...');
  saveEventHistory();
  ws.close();
  process.exit(0);
});

// Periodic status
setInterval(() => {
  console.log(`\n📊 Status: ${eventHistory.length} events | ${dependencyGraph.size} tasks in graph`);
}, 30000); // Every 30 seconds

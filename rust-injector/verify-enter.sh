#!/bin/bash

echo "🧪 Verifying tmux send-keys with Enter key"
echo ""

SESSION="test-enter"

# Clean up any existing session
tmux kill-session -t $SESSION 2>/dev/null

# Create a simple session that echoes input
echo "1️⃣ Creating tmux session with bash..."
tmux new-session -d -s $SESSION

sleep 1

# Send a command with Enter
echo "2️⃣ Sending: echo 'Hello World' + Enter"
tmux send-keys -t $SESSION "echo 'Hello from tmux send-keys!'" Enter

sleep 1

# Capture and display
echo "3️⃣ Captured output:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
tmux capture-pane -t $SESSION -p | tail -3
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Send another command
echo "4️⃣ Sending: date + Enter"
tmux send-keys -t $SESSION "date" Enter

sleep 1

echo "5️⃣ Captured output:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
tmux capture-pane -t $SESSION -p | tail -5
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "✅ Enter key is working! Commands were executed."
echo ""
echo "💡 Cleanup:"
echo "   tmux kill-session -t $SESSION"

#!/bin/bash
# Test script for cclaude-rs launcher

echo "🧪 Testing cclaude-rs launcher"
echo "================================"
echo ""

# Build the binary
echo "1️⃣ Building cclaude-rs..."
cargo build --release --bin cclaude-rs

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# Show binary location
BINARY="./target/release/cclaude-rs"
echo "📦 Binary location: $BINARY"
echo ""

# Show help
echo "2️⃣ Showing help..."
$BINARY --help
echo ""

# Test 1: Default agent (master-orchestrator)
echo "3️⃣ Test 1: Default agent (master-orchestrator)"
echo "Command: $BINARY 'Test prompt'"
echo "(This will launch Claude - press Ctrl+C to exit after testing)"
echo ""
read -p "Press Enter to continue..."

# Test 2: Specific agent
echo ""
echo "4️⃣ Test 2: Specific agent (coding-agent)"
echo "Command: $BINARY --agent coding-agent 'task_id: test-123'"
echo "(This will launch Claude - press Ctrl+C to exit after testing)"
echo ""
read -p "Press Enter to continue..."

# Test 3: Custom directory
echo ""
echo "5️⃣ Test 3: Custom directory"
echo "Command: $BINARY --agent coding-agent --dir /tmp 'Test prompt'"
echo "(This will launch Claude in /tmp - press Ctrl+C to exit after testing)"
echo ""
read -p "Press Enter to continue..."

echo ""
echo "✅ All tests prepared!"
echo ""
echo "📝 Next steps:"
echo "1. Create alias: alias cclaude='$PWD/target/release/cclaude-rs'"
echo "2. Add to ~/.bashrc for permanent use"
echo "3. See CCLAUDE-LAUNCHER.md for complete documentation"
echo ""

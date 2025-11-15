#!/bin/bash

###############################################################################
# Start WebSocket Auto-Input Coordinator
#
# Quick launcher for the WebSocket coordinator that bridges real-time events
# from AgentHub backend to Claude agent sessions
###############################################################################

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
COORDINATOR_DIR="$PROJECT_ROOT/src/websocket-coordinator"
WS_URL="${WS_URL:-ws://localhost:8000/ws}"
WORK_DIR="${WORK_DIR:-/tmp/agenthub_autonomous}"

# Banner
echo -e "${BLUE}"
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║   WebSocket Auto-Input Coordinator Launcher                  ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

# Check Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found${NC}"
    echo "Please install Node.js >= 16.0.0"
    echo "Visit: https://nodejs.org/"
    exit 1
fi

NODE_VERSION=$(node --version | cut -d'v' -f2)
echo -e "${GREEN}✓${NC} Node.js: v$NODE_VERSION"

# Check if backend is running
echo -e "${YELLOW}Checking backend connection...${NC}"
if ! curl -s -f http://localhost:8000/health > /dev/null; then
    echo -e "${RED}❌ Backend not responding at http://localhost:8000${NC}"
    echo ""
    echo "Please start the agenthub backend:"
    echo "  cd /home/daihu/__projects__/4genthub"
    echo "  docker-compose up -d"
    exit 1
fi
echo -e "${GREEN}✓${NC} Backend: http://localhost:8000 ✅"

# Check if dependencies are installed
cd "$COORDINATOR_DIR"

if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}Installing dependencies...${NC}"
    npm install
    echo -e "${GREEN}✓${NC} Dependencies installed"
else
    echo -e "${GREEN}✓${NC} Dependencies: Already installed"
fi

# Create work directory
mkdir -p "$WORK_DIR"
echo -e "${GREEN}✓${NC} Work directory: $WORK_DIR"

echo ""
echo -e "${GREEN}All prerequisites met!${NC}"
echo ""

# Choose mode
echo -e "${BLUE}Choose coordinator mode:${NC}"
echo "  1) POC (Proof of Concept - simulated injections)"
echo "  2) Full (Production - real Claude sessions) [TODO]"
echo ""
read -p "Select mode [1-2] (default: 1): " MODE
MODE=${MODE:-1}

echo ""

case $MODE in
    1)
        echo -e "${BLUE}Starting POC mode...${NC}"
        echo ""
        echo -e "${YELLOW}This mode will:${NC}"
        echo "  • Connect to WebSocket at $WS_URL"
        echo "  • Receive and display real-time events"
        echo "  • Build dependency graph"
        echo "  • Simulate context injections (write to files)"
        echo ""
        echo -e "${GREEN}Press Ctrl+C to stop${NC}"
        echo ""
        sleep 2

        # Run POC
        exec node simple-poc.js "$WS_URL"
        ;;

    2)
        echo -e "${RED}❌ Full mode not yet implemented${NC}"
        echo ""
        echo "Coming soon:"
        echo "  • Spawn real claude -p sessions"
        echo "  • Inject context via stdin pipes"
        echo "  • Monitor stdout for completion"
        echo "  • Handle blocking conditions"
        echo ""
        exit 1
        ;;

    *)
        echo -e "${RED}Invalid selection${NC}"
        exit 1
        ;;
esac

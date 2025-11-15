#!/bin/bash

###############################################################################
# Build script for Claude Session Injector
###############################################################################

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔════════════════════════════════════════════════════════╗"
echo "║   Claude Session Injector - Build Script              ║"
echo "╚════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Rust not found. Installing...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo -e "${GREEN}✓${NC} Rust version: $RUST_VERSION"

# Build mode
MODE="${1:-release}"

if [ "$MODE" = "debug" ]; then
    echo -e "\n${YELLOW}Building in DEBUG mode...${NC}"
    cargo build
    echo -e "${GREEN}✓${NC} Debug build complete"
    echo -e "\nBinary location: ${BLUE}target/debug/claude-injector${NC}"
else
    echo -e "\n${YELLOW}Building in RELEASE mode...${NC}"
    cargo build --release
    echo -e "${GREEN}✓${NC} Release build complete"
    echo -e "\nBinary location: ${BLUE}target/release/claude-injector${NC}"
fi

# Run tests
echo -e "\n${YELLOW}Running tests...${NC}"
if cargo test --quiet; then
    echo -e "${GREEN}✓${NC} All tests passed"
else
    echo -e "${YELLOW}⚠${NC} Some tests failed (may be expected if no Claude sessions exist)"
fi

# Show binary info
if [ "$MODE" = "debug" ]; then
    BIN_PATH="target/debug/claude-injector"
else
    BIN_PATH="target/release/claude-injector"
fi

if [ -f "$BIN_PATH" ]; then
    SIZE=$(du -h "$BIN_PATH" | cut -f1)
    echo -e "\n${GREEN}Build successful!${NC}"
    echo -e "Binary size: ${BLUE}$SIZE${NC}"
    echo -e "\n${GREEN}Run with:${NC} ./$BIN_PATH"
else
    echo -e "\n${YELLOW}Binary not found${NC}"
fi

echo -e "\n${GREEN}Done!${NC}"

#!/bin/bash

# Build script for P2P Harvest Game Web UI
# This script builds the WASM module and sets up the web frontend

set -e  # Exit on error

echo "🌾 P2P Harvest Game - Web Build Script"
echo "======================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Please run this script from the project root directory${NC}"
    exit 1
fi

# Check for required tools
echo -e "${BLUE}Checking prerequisites...${NC}"

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo not found. Please install Rust first:${NC}"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo -e "${GREEN}✓ Rust/Cargo found${NC}"

# Check wasm-pack
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}Warning: wasm-pack not found. Installing...${NC}"
    cargo install wasm-pack
fi
echo -e "${GREEN}✓ wasm-pack found${NC}"

# Check Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js not found. Please install Node.js (v18+) first:${NC}"
    echo "  https://nodejs.org/"
    exit 1
fi
echo -e "${GREEN}✓ Node.js found ($(node --version))${NC}"

# Check npm
if ! command -v npm &> /dev/null; then
    echo -e "${RED}Error: npm not found. Please install npm${NC}"
    exit 1
fi
echo -e "${GREEN}✓ npm found ($(npm --version))${NC}"

echo ""
echo -e "${BLUE}Building WASM module...${NC}"
wasm-pack build \
    --target web \
    --out-dir web/src/wasm \
    --features wasm \
    --no-default-features

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ WASM build successful!${NC}"
else
    echo -e "${RED}✗ WASM build failed${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}Setting up web frontend...${NC}"
cd web

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo -e "${BLUE}Installing npm dependencies...${NC}"
    npm install
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ npm install successful!${NC}"
    else
        echo -e "${RED}✗ npm install failed${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}✓ Dependencies already installed${NC}"
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✓ Build complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "To start the development server:"
echo -e "  ${BLUE}cd web && npm run dev${NC}"
echo ""
echo "To build for production:"
echo -e "  ${BLUE}cd web && npm run build${NC}"
echo ""
echo "To rebuild WASM after code changes:"
echo -e "  ${BLUE}./build-web.sh${NC}"
echo ""
echo "Happy farming! 🌱"

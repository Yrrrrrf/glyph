#!/usr/bin/env bash

# Glyph WASM Build Script
# Run this to rebuild WASM and regenerate bindings

set -e  # Exit on error

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}Starting Glyph WASM build workflow...${NC}"

# Clean old bindings
echo -e "${YELLOW}Cleaning old WASM bindings...${NC}"
rm -rf src/lib/pkg
echo -e "${GREEN}✓ Cleaned${NC}"

# Clean Rust build
echo -e "${YELLOW}Cleaning Rust build cache...${NC}"
cargo clean
echo -e "${GREEN}✓ Cleaned${NC}"

# Build WASM
echo -e "${YELLOW}Building WASM (release)...${NC}"
cargo build --lib --target wasm32-unknown-unknown --release
echo -e "${GREEN}✓ Built${NC}"

# Generate bindings
echo -e "${YELLOW}Generating JS bindings...${NC}"
~/.cargo/bin/wasm-bindgen --target web --out-dir src/lib/pkg ./target/wasm32-unknown-unknown/release/glyph.wasm
echo -e "${GREEN}✓ Bindings generated${NC}"

# Format code
echo -e "${YELLOW}Formatting code...${NC}"
deno fmt
cargo fmt
echo -e "${BLUE}✓ Formatted${NC}"

echo -e "${GREEN}Build complete!${NC}"

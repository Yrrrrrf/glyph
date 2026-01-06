#!/usr/bin/env bash

# ANSI color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# --- 1. Validation ---
# Check if an argument was provided
if [ -z "$1" ]; then
    echo -e "${RED}Error:${NC} Usage: run-asm <filename.asm>"
    exit 1
fi
SOURCE_FILE="$1"
# Check if the provided file exists
if [ ! -f "$SOURCE_FILE" ]; then
    echo -e "${RED}Error:${NC} File '${SOURCE_FILE}' not found."
    exit 1
fi

# --- 2. Compilation & Execution ---
# Get the base name of the file (e.g., "hello" from "scripts/hello.asm")
BASENAME=$(basename "$SOURCE_FILE" .asm)
OUTPUT_FILE="$BASENAME.com"
echo -e "${BLUE}--- Compiling:${NC} ${BOLD}$SOURCE_FILE${NC} ➜ ${BOLD}$OUTPUT_FILE${NC}"
# Compile the .asm file into a .com binary using nasm
nasm -f bin "$SOURCE_FILE" -o "$OUTPUT_FILE"
# Check if compilation was successful
if [ $? -ne 0 ]; then
    echo -e "${RED}Error:${NC} Compilation failed."
    exit 1
fi

echo -e "${GREEN}--- Executing:${NC} ${BOLD}$OUTPUT_FILE${NC} with emu2"
# Execute the compiled program with the emu2 emulator
emu2 "$OUTPUT_FILE" "${@:2}"
echo -e "${GREEN}--- Done ---${NC}"

# --- 3. Cleanup ---
echo -e "${YELLOW}--- Cleaning up:${NC} Removing ${BOLD}$OUTPUT_FILE${NC}"
rm "$OUTPUT_FILE"

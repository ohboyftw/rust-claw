#!/bin/bash
set -e

# Build the project
echo "Building project..."
cargo build

# Path to binary
BIN_PATH="./target/debug/app"

# Create a temporary input file
INPUT_FILE=$(mktemp)
echo "Hello from sandbox!" > "$INPUT_FILE"
# Add a second message to trigger the mock LLM
echo "What's the meaning of life?" >> "$INPUT_FILE"

# Run the app, feeding it the input file, and capture output
echo "Running application in sandbox..."
# The app reads stdin until EOF, so this works perfectly.
# We pipe stderr to stdout to catch logs too.
# We use `timeout` to ensure it doesn't hang if something goes wrong.
# Allow timeout to exit with 124 (which is expected since it's a long-running process)
OUTPUT=$(timeout 5s "$BIN_PATH" < "$INPUT_FILE" 2>&1 || true)

# Cleanup
rm "$INPUT_FILE"

# Check output
echo "----------------------------------------"
echo "$OUTPUT"
echo "----------------------------------------"

if echo "$OUTPUT" | grep -q "Echo: Hello from sandbox!"; then
    echo "SUCCESS: Found echo response."
else
    echo "FAILURE: Did not find echo response."
    exit 1
fi

if echo "$OUTPUT" | grep -q "Echo: What's the meaning of life?"; then
     echo "SUCCESS: Found second echo response."
else
     echo "FAILURE: Did not find second echo response."
     exit 1
fi

echo "Sandbox test passed successfully."

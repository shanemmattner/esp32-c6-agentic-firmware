#!/bin/bash
# test-all-lessons.sh
# Quick build test for all lessons in esp32-c6-agentic-firmware
#
# Usage: ./test-all-lessons.sh [repo_path]
#
# This script tests all lessons by building them and reporting status.
# Use this for smoke testing to verify all lessons compile.

REPO_PATH="${1:-/Users/shanemattner/Desktop/esp32-c6-agentic-firmware}"

if [ ! -d "$REPO_PATH/lessons" ]; then
  echo "ERROR: lessons directory not found at $REPO_PATH"
  exit 1
fi

echo "=== Testing All Lessons ==="
echo "Repository: $REPO_PATH"
echo ""

cd "$REPO_PATH" || exit 1

PASS_COUNT=0
FAIL_COUNT=0

for lesson_dir in lessons/*/; do
  name=$(basename "$lesson_dir")
  echo "=== $name ==="

  # Build and capture exit code
  cargo build --release --manifest-path "${lesson_dir}Cargo.toml" >/dev/null 2>&1
  BUILD_STATUS=$?

  if [ $BUILD_STATUS -eq 0 ]; then
    echo "✅ PASS"
    PASS_COUNT=$((PASS_COUNT + 1))
  else
    echo "❌ FAIL"
    FAIL_COUNT=$((FAIL_COUNT + 1))
    # Show last few lines of error
    cargo build --release --manifest-path "${lesson_dir}Cargo.toml" 2>&1 | tail -5
  fi
  echo ""
done

echo "=== Summary ==="
echo "Passed: $PASS_COUNT"
echo "Failed: $FAIL_COUNT"
echo "Total:  $((PASS_COUNT + FAIL_COUNT))"

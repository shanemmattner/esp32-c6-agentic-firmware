#!/bin/bash
# check-cargo-locks.sh
# Check for stale Cargo.lock files in lesson directories
#
# Usage: ./check-cargo-locks.sh [repo_path] [max_age_days]
#
# This script identifies lessons with Cargo.lock files older than max_age_days
# and can optionally update them.

REPO_PATH="${1:-/Users/shanemattner/Desktop/esp32-c6-agentic-firmware}"
MAX_AGE_DAYS="${2:-7}"

if [ ! -d "$REPO_PATH/lessons" ]; then
  echo "ERROR: lessons directory not found at $REPO_PATH"
  exit 1
fi

echo "=== Checking Cargo.lock Freshness ==="
echo "Repository: $REPO_PATH"
echo "Max age: $MAX_AGE_DAYS days"
echo ""

cd "$REPO_PATH" || exit 1

STALE_COUNT=0
FRESH_COUNT=0

# Check timestamps
echo "Lesson Cargo.lock timestamps:"
ls -lh lessons/*/Cargo.lock 2>/dev/null | awk '{print $6, $7, $9}' | sed 's/lessons\///' | sed 's/\/Cargo.lock//'

echo ""
echo "Stale locks (>$MAX_AGE_DAYS days old):"

for lesson_dir in lessons/*/; do
  lock_file="${lesson_dir}Cargo.lock"
  if [ -f "$lock_file" ]; then
    # Find files modified more than MAX_AGE_DAYS ago
    if find "$lock_file" -mtime +$MAX_AGE_DAYS | grep -q .; then
      name=$(basename "$lesson_dir")
      age=$(find "$lock_file" -printf '%TY-%Tm-%Td\n' 2>/dev/null || stat -f "%Sm" -t "%Y-%m-%d" "$lock_file")
      echo "  ⚠️  $name (last modified: $age)"
      STALE_COUNT=$((STALE_COUNT + 1))
    else
      FRESH_COUNT=$((FRESH_COUNT + 1))
    fi
  fi
done

if [ $STALE_COUNT -eq 0 ]; then
  echo "  (none - all locks are fresh)"
fi

echo ""
echo "=== Summary ==="
echo "Fresh: $FRESH_COUNT"
echo "Stale: $STALE_COUNT"

if [ $STALE_COUNT -gt 0 ]; then
  echo ""
  echo "Run 'cargo update' in stale lesson directories to refresh dependencies."
  echo ""
  echo "Or run this to update all lessons:"
  echo "  for dir in lessons/*/; do (cd \$dir && cargo update); done"
fi

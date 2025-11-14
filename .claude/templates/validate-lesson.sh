#!/bin/bash
# validate-lesson.sh
# Comprehensive validation script for a single lesson
#
# Usage: ./validate-lesson.sh <lesson_dir>
#
# This script builds a lesson, checks binary size, and reports status.
# Use this for detailed validation of individual lessons.

LESSON_DIR="$1"

if [ -z "$LESSON_DIR" ]; then
  echo "Usage: $0 lessons/XX-name/"
  echo ""
  echo "Example: $0 lessons/01-button-neopixel/"
  exit 1
fi

if [ ! -d "$LESSON_DIR" ]; then
  echo "ERROR: Directory not found: $LESSON_DIR"
  exit 1
fi

LESSON_NAME=$(basename "$LESSON_DIR")
MANIFEST="$LESSON_DIR/Cargo.toml"

if [ ! -f "$MANIFEST" ]; then
  echo "ERROR: Cargo.toml not found at $MANIFEST"
  exit 1
fi

echo "=== Validating $LESSON_NAME ==="
echo ""

# Check for stale Cargo.lock
if [ -f "$LESSON_DIR/Cargo.lock" ]; then
  LOCK_AGE=$(find "$LESSON_DIR/Cargo.lock" -mtime +7 2>/dev/null | wc -l)
  if [ "$LOCK_AGE" -gt 0 ]; then
    echo "⚠️  Cargo.lock is >7 days old - consider running 'cargo update'"
  fi
fi

# Build the lesson
echo "Building..."
BUILD_OUTPUT=$(cargo build --release --manifest-path "$MANIFEST" 2>&1)
BUILD_STATUS=$?

if [ $BUILD_STATUS -eq 0 ]; then
  echo "✅ Build succeeded"
  echo ""

  # Find the actual binary (package name may vary)
  TARGET_DIR="$LESSON_DIR/target/riscv32imac-unknown-none-elf/release"

  if [ -d "$TARGET_DIR" ]; then
    # Find executable files (lesson-* or main)
    BINARY=$(find "$TARGET_DIR" -type f -perm +111 2>/dev/null | grep -E '(lesson-|main$)' | head -1)

    if [ -n "$BINARY" ]; then
      echo "Binary found:"
      ls -lh "$BINARY" | awk '{print "  Path: " $9 "\n  Size: " $5}'

      # Show size breakdown if available
      if command -v size &>/dev/null; then
        echo ""
        echo "Size breakdown:"
        size "$BINARY" | awk 'NR==1{print "  " $0} NR==2{print "  " $0}'
      fi
    else
      echo "⚠️  Binary not found in target directory"
      echo "Target dir: $TARGET_DIR"
      ls -lh "$TARGET_DIR" 2>/dev/null | head -10
    fi
  else
    echo "⚠️  Target directory not found: $TARGET_DIR"
  fi

  # Check for warnings
  WARNING_COUNT=$(echo "$BUILD_OUTPUT" | grep -c "warning:")
  if [ "$WARNING_COUNT" -gt 0 ]; then
    echo ""
    echo "⚠️  $WARNING_COUNT warning(s) during build"
    echo "$BUILD_OUTPUT" | grep "warning:" | head -5
  fi

else
  echo "❌ Build failed"
  echo ""
  echo "Error output:"
  echo "$BUILD_OUTPUT" | grep -E '(error|error\[)' | head -20
  echo ""
  echo "Suggestion: Try 'cargo update' first:"
  echo "  cd $LESSON_DIR && cargo update && cargo build --release"
  exit 1
fi

echo ""
echo "=== Validation Complete ==="

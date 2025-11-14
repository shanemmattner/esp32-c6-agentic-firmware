---
description: Comprehensive repository review for esp32-c6-agentic-firmware - validate lessons, docs, structure, and prepare for community sharing
---

# /review-repo - Complete Repository Review & Cleanup

**Purpose**: Thoroughly review the entire esp32-c6-agentic-firmware repository to ensure it's ready for sharing with the embedded community. This command validates every lesson, documentation, project structure, and tooling.

**Target Audience**: Repository maintainer preparing to share with embedded Rust community

**Use when**: Before major releases, community sharing, or periodic quality audits

---

## Review Goals

1. **Educational Quality**: Ensure lessons progressively teach esp-hal 1.0.0 effectively
2. **Technical Accuracy**: Verify all code builds, flashes, and runs on hardware
3. **Documentation Clarity**: Check that docs are comprehensive but not verbose
4. **Community Readiness**: Ensure repo is approachable for external contributors
5. **LLM Optimization**: Validate Claude Code integration and debugging workflows

---

## Phase 1: Repository Structure & Documentation Review

### Step 1.1: Analyze Repository Structure

**Examine the overall project layout:**

```bash
# Review top-level structure
# Try tree first, fall back to find if not available
if command -v tree &>/dev/null; then
  tree -L 2 -I 'target|.git'
else
  # Fallback: use find to show structure
  echo "=== Repository Structure ==="
  find . -maxdepth 2 -type d | grep -v -E '(target|\.git|node_modules)' | sort
fi

# Check for consistency across lessons
find lessons -name "Cargo.toml" | wc -l
find lessons -name "README.md" | wc -l
find lessons -name "TEST.md" | wc -l
```

**Questions to answer:**
- Does the structure make intuitive sense for learners?
- Are lesson directories consistently organized?
- Are there any orphaned or unnecessary files?
- Is there clear separation between lessons, tools, and infrastructure?

**Actions:**
- [ ] Review directory tree and identify inconsistencies
- [ ] List any files that should be deleted or consolidated
- [ ] Check if lesson numbering makes logical sense (01, 02, 03...)
- [ ] Verify `.gitignore` is comprehensive

---

### Step 1.2: Review CLAUDE.md

**Read and analyze** `/CLAUDE.md`:

1. **Check accuracy**: Are all guidelines still current with esp-hal 1.0.0?
2. **Validate conventions**: Do they match actual codebase practices?
3. **Find gaps**: What's missing that would help Claude Code?
4. **Simplify**: Can any sections be clearer or more concise?

**Specific checks:**
- [ ] Memory map constants (RAM_START/RAM_END) documented correctly
- [ ] UART debugging workflow matches Lesson 08
- [ ] Bash execution best practices are accurate
- [ ] Hardware testing infrastructure documented
- [ ] esp-hal 1.0.0 API patterns up to date
- [ ] File operation guidelines clear (Task vs Write/Edit)

**Output**: List of CLAUDE.md improvements needed

---

### Step 1.3: Review README.md

**Read** `/README.md` (if exists, or note if missing):

**First, verify lesson list accuracy:**

```bash
# Get actual lesson names
ls -1 lessons/

# Compare to what README claims
# Read README and check lesson section matches reality
```

**If discrepancies found, note them immediately before proceeding.**

**Questions:**
- Does it explain the repo purpose clearly?
- Does it target the right audience (engineers learning esp-hal)?
- Are prerequisites listed (Rust, esp-hal, hardware)?
- Is setup/installation documented?
- Does it explain the LLM-enhanced debugging approach?
- Are there clear "Getting Started" instructions?

**Actions:**
- [ ] Validate lesson list matches actual lesson directories
- [ ] Check if README exists and is comprehensive
- [ ] Verify hardware requirements listed
- [ ] Check if software dependencies documented
- [ ] Ensure community contribution guidelines present

---

### Step 1.4: Review .claude/ Infrastructure

**Examine each file in `.claude/`:**

```bash
# List all Claude Code infrastructure
find .claude -type f -name "*.md" -o -name "*.py" -o -name "*.sh"
```

**For each command in `.claude/commands/`:**
- [ ] `/gen-lesson` - Still relevant? Works as documented?
- [ ] `/test-lesson` - Comprehensive enough? Needs updates?
- [ ] `/test-debug-infrastructure` - Covers all debug workflows?
- [ ] `/setup-hardware-lesson` - Template up to date?
- [ ] `/test-uart-pins` - Utility still needed?
- [ ] `/improve-command` - Meta-tool working well?
- [ ] `/esp32-debug` - Debug command useful?

**For `.claude/templates/`:**
- [ ] `uart_test_minimal.rs` - Uses esp-hal 1.0.0 correctly?
- [ ] `read_uart.py` - Works reliably across platforms?
- [ ] Any other templates needed?

**Actions:**
- List commands to delete, update, or add
- Note any templates that need refreshing

---

## Best Practices for Bulk Operations

**CRITICAL:** Due to shell execution limitations in the LLM environment, follow these patterns:

### ✅ Use Temp Scripts for Bulk Testing

**For testing multiple lessons, ALWAYS use temp scripts:**

**Option 1: Use provided template script**

```bash
# Use the pre-made template
.claude/templates/test-all-lessons.sh
```

**Option 2: Create custom temp script**

```bash
# ✅ RECOMMENDED: Temp script approach
cat > /tmp/test-all-lessons.sh << 'EOF'
#!/bin/bash
cd /Users/shanemattner/Desktop/esp32-c6-agentic-firmware

for lesson_dir in lessons/*/; do
  name=$(basename "$lesson_dir")
  echo "=== Testing $name ==="
  cargo build --release --manifest-path "$lesson_dir/Cargo.toml" 2>&1 | tail -5
  echo ""
done
EOF
chmod +x /tmp/test-all-lessons.sh
/tmp/test-all-lessons.sh
```

### ❌ Avoid Inline Loops with Command Substitution

**These patterns WILL FAIL in eval context:**

```bash
# ❌ BAD: Command substitution in loops
for lesson in lessons/*/; do echo "$(basename $lesson)"; done

# ❌ BAD: Complex glob patterns with variables
for num in 01 02 03; do cargo build --manifest-path "lessons/$num"-*/Cargo.toml; done
```

### ✅ Use --manifest-path Instead of cd

**Follow CLAUDE.md convention:**

```bash
# ✅ GOOD: Use --manifest-path to avoid cd
cargo build --release --manifest-path lessons/01-button-neopixel/Cargo.toml

# ❌ AVOID: cd into directory
cd lessons/01-button-neopixel && cargo build --release && cd ../..
```

### ✅ Filter Output for Efficiency

**Reduce token usage by filtering verbose output:**

```bash
# For successful builds, show only summary
cargo build --release --manifest-path lessons/01-button-neopixel/Cargo.toml 2>&1 | tail -5

# For failed builds, show errors
cargo build --release --manifest-path lessons/01-button-neopixel/Cargo.toml 2>&1 | grep -E '(error|warning)' | head -20
```

---

## Phase 2: Lesson-by-Lesson Review

### Step 2.0: Check Dependency Freshness

**CRITICAL: Before building lessons, check for stale Cargo.lock files.**

**Why this matters:**
- esp-hal ecosystem evolves rapidly
- Stale `Cargo.lock` can cause build failures with newer Rust nightly
- Fresh checkout won't have `Cargo.lock` (gitignored), but repo maintainer might
- Dependency drift can create false build failures

**Check Cargo.lock timestamps:**

**Option 1: Use provided template script**

```bash
# Use the pre-made checker
.claude/templates/check-cargo-locks.sh
```

**Option 2: Manual check**

```bash
# Check when lessons were last updated
ls -lh lessons/*/Cargo.lock | awk '{print $6, $7, $9}'

# Identify stale locks (>7 days old)
find lessons -name "Cargo.lock" -mtime +7 -exec ls -lh {} \;
```

**If stale Cargo.lock files found (or if any lesson fails to build):**

```bash
# Update dependencies for all lessons (use temp script)
cat > /tmp/update-deps.sh << 'EOF'
#!/bin/bash
cd /Users/shanemattner/Desktop/esp32-c6-agentic-firmware

for lesson_dir in lessons/*/; do
  name=$(basename "$lesson_dir")
  echo "=== Updating $name ==="
  (cd "$lesson_dir" && cargo update)
done
EOF
chmod +x /tmp/update-deps.sh
/tmp/update-deps.sh
```

**Proceed to build validation only after ensuring dependencies are fresh.**

---

### Step 2.1: Validate Each Lesson

**For EACH lesson (01 through 08), perform these checks:**

#### Build & Flash Validation

**Use temp script for systematic testing:**

**Option 1: Use provided template script**

```bash
# Use the pre-made validation script
.claude/templates/validate-lesson.sh lessons/01-button-neopixel/
.claude/templates/validate-lesson.sh lessons/02-task-scheduler/
# ... continue for each lesson
```

**Option 2: Create custom validation script**

```bash
# Create build validation script
cat > /tmp/validate-lesson.sh << 'EOF'
#!/bin/bash
LESSON_DIR="$1"

if [ -z "$LESSON_DIR" ]; then
  echo "Usage: $0 lessons/XX-name/"
  exit 1
fi

LESSON_NAME=$(basename "$LESSON_DIR")
echo "=== Validating $LESSON_NAME ==="

# Build the lesson
echo "Building..."
cargo build --release --manifest-path "$LESSON_DIR/Cargo.toml" 2>&1 | tail -10

if [ ${PIPESTATUS[0]} -eq 0 ]; then
  echo "✅ Build succeeded"

  # Find the actual binary (package name may vary)
  BINARY=$(find "$LESSON_DIR/target/riscv32imac-unknown-none-elf/release/" -type f -perm +111 2>/dev/null | grep -E '(lesson-|main$)' | head -1)

  if [ -n "$BINARY" ]; then
    echo "Binary: $(ls -lh "$BINARY" | awk '{print $9, $5}')"
    command -v size &>/dev/null && size "$BINARY" || true
  else
    echo "⚠️ Binary not found in target directory"
  fi
else
  echo "❌ Build failed"
  echo "Trying with cargo update first..."
  (cd "$LESSON_DIR" && cargo update)
  cargo build --release --manifest-path "$LESSON_DIR/Cargo.toml" 2>&1 | grep -E '(error|warning)' | head -20
fi
EOF

chmod +x /tmp/validate-lesson.sh

# Test each lesson
/tmp/validate-lesson.sh lessons/01-button-neopixel/
/tmp/validate-lesson.sh lessons/02-task-scheduler/
# ... continue for each lesson
```

**For individual lesson testing:**

```bash
# Simple build test
cargo build --release --manifest-path lessons/01-button-neopixel/Cargo.toml 2>&1 | tail -5
```

**If build fails with dependency errors:**

```bash
# Try updating dependencies first
cd lessons/XX-name/
cargo update
cargo build --release 2>&1 | tail -10
```

**Questions:**
- Does it build without errors?
- Are warnings acceptable or should they be fixed?
- Is binary size reasonable (<100KB for simple lessons)?

#### Documentation Review

**Read `lessons/XX-name/README.md`:**

- [ ] Purpose clearly stated
- [ ] Hardware wiring documented with pin numbers
- [ ] Expected behavior described
- [ ] Build/flash instructions present
- [ ] Troubleshooting section included
- [ ] Learning objectives clear
- [ ] Language appropriate (not too complex, not too simple)
- [ ] Length reasonable (<300 lines, as per CLAUDE.md)

**Read `lessons/XX-name/TEST.md` (if exists):**

- [ ] Test procedures defined
- [ ] Success criteria clear
- [ ] Hardware validation steps listed

#### Code Quality Review

**Read `lessons/XX-name/src/bin/main.rs`:**

- [ ] Code follows esp-hal 1.0.0 patterns
- [ ] Comments explain "why" not "what"
- [ ] No hardcoded magic numbers (use constants)
- [ ] Memory safety practices followed
- [ ] Appropriate logging/debugging output
- [ ] Code length reasonable (~100-150 lines for simple lessons)
- [ ] No over-engineering (keep it lean)

**Check dependencies in `Cargo.toml`:**

- [ ] Only necessary dependencies included
- [ ] esp-hal version is 1.0.0
- [ ] `[[bin]]` section present and correct

#### Lesson Progression Check

**Compare to previous lesson:**
- Does complexity increase gradually?
- Does it build on prior concepts?
- Is the jump in difficulty appropriate?

---

### Step 2.2: Specific Lesson Reviews

**Review each lesson for specific concerns:**

#### Lesson 01: Button + NeoPixel
- [ ] Simplest possible intro to GPIO and delays
- [ ] Button input working reliably?
- [ ] NeoPixel timing correct?
- [ ] Good starting point for beginners?

#### Lesson 02: Task Scheduler
- [ ] Task scheduling concepts clear?
- [ ] Code demonstrates cooperative multitasking?
- [ ] Too complex too early? Should it move later?

#### Lesson 03: MPU9250
- [ ] I2C driver clear and well-documented?
- [ ] Sensor initialization explained?
- [ ] Error handling appropriate?

#### Lesson 04: Static Color Navigator
- [ ] Name makes sense? (typo: "statig"?)
- [ ] UI/interaction patterns clear?
- [ ] Dependencies reasonable?

#### Lesson 05: Unit and Integration Testing
- [ ] Test patterns applicable to other lessons?
- [ ] Both unit and integration tests present?
- [ ] Valuable for teaching testing practices?

#### Lesson 06: UART Terminal
- [ ] UART configuration clear?
- [ ] Terminal interaction documented?
- [ ] Prepares well for Lesson 07/08?

#### Lesson 07: GDB Debugging
- [ ] GDB workflow documented?
- [ ] Breakpoints, inspection examples shown?
- [ ] Works with current esp-hal?

#### Lesson 08: UART + GDB Tandem
- [ ] Combines UART streaming + GDB inspection?
- [ ] RAM bounds correct (0x40800000 - 0x40880000)?
- [ ] Memory-safe variable streaming working?
- [ ] Good capstone lesson?

---

## Phase 3: Hardware Testing (Critical)

### Step 3.1: Test Each Lesson on Hardware

**For each lesson, perform actual hardware validation:**

```bash
# Use the test-lesson command
/test-lesson 01
/test-lesson 02
# ... through 08
```

**For each lesson:**
- [ ] Builds successfully
- [ ] Flashes without errors
- [ ] Runs as documented
- [ ] Expected output matches README
- [ ] No unexpected errors or warnings
- [ ] Hardware behavior matches description

**Document any failures** and fix them before proceeding.

---

## Phase 4: Documentation Audit

### Step 4.1: Check Documentation Completeness

**Review all markdown files:**

```bash
find . -name "*.md" -type f | grep -v target | grep -v node_modules
```

**For each document:**
- [ ] Grammar and spelling correct
- [ ] Technical accuracy verified
- [ ] Links work (no broken references)
- [ ] Code examples match current esp-hal 1.0.0
- [ ] Tone appropriate for target audience

### Step 4.2: Validate Inline Documentation

**Check code comments across all lessons:**

- [ ] Comments explain "why" not "what"
- [ ] Complex sections have explanatory comments
- [ ] No outdated or misleading comments
- [ ] Function/struct docs present where needed

---

## Phase 5: Simplification & Clarity Pass

### Step 5.1: Identify Over-Complexity

**Review each lesson for unnecessary complexity:**

**Questions:**
- Can code be simplified without losing educational value?
- Are there abstractions that obscure learning?
- Is language in docs too technical/verbose?
- Are there "clever" patterns that should be straightforward?

**Actions:**
- [ ] List lessons that need simplification
- [ ] Identify verbose documentation to trim
- [ ] Find over-engineered code to refactor

### Step 5.2: Enhance Clarity

**Review for clarity improvements:**

**Questions:**
- Are learning objectives explicit?
- Do examples clearly demonstrate concepts?
- Are error messages helpful?
- Is troubleshooting guidance comprehensive?

**Actions:**
- [ ] List areas needing clearer explanations
- [ ] Identify confusing code patterns
- [ ] Note missing examples or diagrams

---

## Phase 6: Firmware & Software Gaps Analysis

### Step 6.1: Firmware Side Review

**Questions:**
- **Peripheral Coverage**: Are key ESP32-C6 peripherals covered?
  - GPIO ✓ (Lesson 01)
  - UART ✓ (Lessons 06, 08)
  - I2C ✓ (Lesson 03)
  - SPI? (missing?)
  - Timers? (in Lesson 02, but dedicated lesson?)
  - ADC? (missing?)
  - PWM? (missing?)
  - WiFi? (missing?)
  - BLE? (missing?)

- **Advanced Concepts**: What's missing?
  - DMA? (planned in Lesson 08 v2?)
  - Interrupts? (touched in Lesson 01, but dedicated?)
  - Sleep modes?
  - RTC?
  - Flash storage?

- **Debugging Techniques**: Fully covered?
  - esp-println ✓
  - GDB ✓ (Lesson 07)
  - UART streaming ✓ (Lesson 08)
  - probe-rs?
  - RTT? (replaced with UART)

**Output**: List of firmware topics to add

---

### Step 6.2: Software Side Review

**Questions:**
- **Tooling**: What tools are missing?
  - Python UART reader ✓
  - GDB scripts? (Python API usage?)
  - Log analyzers?
  - Test automation?
  - CI/CD? (GitHub Actions?)

- **Scripts**: What utilities would help?
  - Port detection ✓ (find-esp32-ports.sh)
  - Automated testing?
  - Log parsing/visualization?
  - Performance profiling?

- **Documentation Tools**:
  - API docs generation? (rustdoc)
  - Lesson dependency graph?
  - Hardware setup diagrams?

**Output**: List of software/tooling to add

---

## Phase 7: Community Readiness

### Step 7.1: Contribution Guidelines

**Check if repo has:**
- [ ] CONTRIBUTING.md
- [ ] CODE_OF_CONDUCT.md
- [ ] LICENSE file
- [ ] Issue templates
- [ ] PR templates
- [ ] Beginner-friendly labels

### Step 7.2: Example Projects

**Check if there are:**
- [ ] Real-world example projects
- [ ] Reference implementations
- [ ] Common use-case templates

### Step 7.3: External Dependencies

**Audit external dependencies:**
- [ ] All crates are from crates.io (no git dependencies)
- [ ] Version pins are appropriate
- [ ] No unnecessary dependencies

---

## Phase 8: Final Cleanup & Organization

### Step 8.1: Delete Excess Files

**Identify files to remove:**
```bash
# Find potential cruft
find . -name "*.bak" -o -name "*.tmp" -o -name ".DS_Store"
find . -type d -name "target" -o -name "node_modules"
```

**Check for:**
- [ ] Orphaned test files
- [ ] Unused scripts
- [ ] Old/deprecated code
- [ ] Backup files
- [ ] Build artifacts in git

### Step 8.2: Git Hygiene

**Review git status:**
- [ ] No uncommitted changes
- [ ] .gitignore comprehensive
- [ ] No large binaries in history
- [ ] Commit messages descriptive
- [ ] Branches organized

### Step 8.3: Final Structure Validation

**Ensure consistent structure across all lessons:**

```
lessons/XX-name/
├── src/
│   ├── bin/
│   │   └── main.rs
│   └── lib.rs (optional)
├── .cargo/
│   └── config.toml
├── Cargo.toml
├── rust-toolchain.toml
├── build.rs (if needed)
├── README.md
└── TEST.md (if applicable)
```

---

## Phase 9: Generate Review Report

### Step 9.1: Summarize Findings

**Create a comprehensive report with:**

#### Section 1: Lessons Status
- Table showing each lesson's build/test status
- List of lessons needing fixes
- Recommendations for lesson order/numbering

#### Section 2: Documentation Quality
- CLAUDE.md improvements needed
- README.md status (exists? complete?)
- Lesson docs that need updates
- New documentation needed

#### Section 3: Code Quality
- Lessons following best practices
- Code needing refactoring
- Warnings to address
- Performance concerns

#### Section 4: Missing Features
- Firmware side gaps (peripherals, concepts)
- Software side gaps (tools, scripts)
- Infrastructure improvements

#### Section 5: Community Readiness
- Contribution guidelines status
- License/legal compliance
- Example projects needed
- Issue/PR templates

#### Section 6: Cleanup Actions
- Files to delete
- Directories to reorganize
- Git cleanup needed

#### Section 7: Priority Recommendations
1. **Critical** (must fix before sharing)
2. **Important** (should fix soon)
3. **Nice-to-have** (future improvements)

---

## Execution Instructions

**When this command is run, Claude should:**

1. **Create todo list** with all phases (use TodoWrite)
2. **Work systematically** through each phase
3. **Document findings** as you go (create `/tmp/review-findings.md`)
4. **Ask questions** when decisions needed (use AskUserQuestion)
5. **Test on hardware** where possible (use /test-lesson)
6. **Summarize at end** with actionable recommendations

**Estimated time**: 2-4 hours for thorough review

---

## Output Deliverables

At completion, provide:

1. **Review Report** (`/tmp/esp32-c6-repo-review-YYYY-MM-DD.md`)
   - Executive summary
   - Detailed findings by phase
   - Prioritized action items

2. **Lesson Status Matrix**
   ```
   | Lesson | Build | Flash | Run | Docs | Tests | Status |
   |--------|-------|-------|-----|------|-------|--------|
   | 01     | ✓     | ✓     | ✓   | ✓    | -     | PASS   |
   | 02     | ✓     | ✗     | -   | ✓    | -     | FAIL   |
   ...
   ```

3. **Action Plan** (prioritized list of fixes/improvements)

4. **Questions for User** (decisions needed before proceeding)

---

## Notes

- **Be thorough but efficient** - don't get stuck on minor details
- **Test on actual hardware** - this is critical for validation
- **Ask questions early** - don't guess at user intent
- **Document everything** - findings should be actionable
- **Prioritize ruthlessly** - separate critical from nice-to-have

---

**Usage**:
```bash
/review-repo
```

**Follow-up commands after review:**
```bash
# Fix specific lesson
/gen-lesson "Fix Lesson 02 based on review findings"

# Update documentation
# (manual editing of CLAUDE.md, README.md)

# Re-test after fixes
/test-lesson 02
```

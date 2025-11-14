---
description: Complete ESP32-C6 lesson development workflow - from PRD to hardware-validated PR with GDB-based progressive commits
---

# /gen-lesson - ESP32-C6 GDB Lesson Development Workflow

**Purpose**: End-to-end lesson development for ESP32-C6 firmware using esp-hal 1.0.0 with GDB-based discovery pedagogy. Creates progressively revealed lessons through commit-by-commit exploration where students use GDB to discover solutions.

**Target Audience**: Engineers familiar with embedded systems learning Rust esp-hal 1.0.0 on ESP32-C6 using Claude Code + GDB for interactive debugging.

**Pedagogy**: Discovery-based learning through progressive commits. Each commit reveals functionality step-by-step. Students use GDB to investigate and discover solutions, not just read completed code.

**Use when**: Creating new GDB-integrated lessons (01-07) following the curriculum in `GDB_LESSON_PLANS.md`.

---

## Workflow Overview

```
/gen-lesson "Create Lesson 03: I2C + GDB"
  ├─ Phase 0: Review Lesson Plan (read GDB_LESSON_PLANS.md for this lesson)
  ├─ Phase 1: Proactive Hardware Testing (/test-hardware before implementation)
  ├─ Phase 2: Project Setup (branch, directory structure, starter code)
  ├─ Phase 3: Progressive Commit Development
  │   ├─ Commit 1: "Broken" or minimal firmware (discovery phase)
  │   ├─ Commit 2: GDB technique #1 (investigation)
  │   ├─ Commit 3: GDB technique #2 (solution)
  │   └─ Commit 4+: Additional techniques as needed
  ├─ Phase 4: Documentation (README with commit-by-commit walkthrough)
  ├─ Phase 5: Hardware Validation (user confirms)
  └─ Phase 6: Push branch and create PR

**Time estimate**: 2-4 hours depending on complexity

---

## Phase 0: Review Lesson Plan

**Goal**: Understand the lesson structure, GDB techniques, and commit strategy BEFORE writing code.

### Step 0.1: Read Lesson Specification

**MANDATORY**: Read the lesson plan from `GDB_LESSON_PLANS.md` for the target lesson number.

**Extract key information**:
1. **GDB Techniques** - Which 2-3 techniques will be taught?
2. **Commit Structure** - What does each commit demonstrate?
3. **Demo Scripts** - What GDB commands will students run?
4. **Hardware Requirements** - What components are needed?
5. **Learning Objectives** - What's the "wow moment"?

**Example for Lesson 01**:
- **Techniques**: Memory inspection/writes, GDB variables (bit math), function calls
- **Commits**:
  - Commit 1: Broken firmware (missing GPIO enable)
  - Commit 2: GDB register control (bit math)
  - Commit 3: Function calls (remote control)
- **Wow Moment**: Calling `led_toggle()` from GDB while firmware runs

### Step 0.2: Ask Clarifying Questions (If Needed)

**Only ask if lesson plan is unclear or missing details**:
- Pin assignments not specified?
- Hardware setup ambiguous?
- Scope unclear?

**Otherwise**: Proceed directly to hardware testing.

---

## Phase 1: Proactive Hardware Testing

**Goal**: Verify hardware works BEFORE writing lesson code. Never teach with untested hardware.

**CRITICAL**: Use `/test-hardware` command to validate hardware setup.

### Step 1.1: Run Hardware Test

```
/test-hardware {peripheral} {pins}
```

**Examples**:
```
/test-hardware gpio 12
/test-hardware uart 16 17
/test-hardware i2c 6 7
```

**What this does**:
- Creates minimal test firmware
- Builds and flashes to hardware
- Reports success/failure
- Saves working configuration

### Step 1.2: Validate Hardware

**If test succeeds**: Document working pins and proceed

**If test fails**:
- Troubleshoot hardware (wrong pins, loose connections, etc.)
- Fix and re-test
- DO NOT proceed until hardware works

**Output**:
> ✅ Hardware validation complete
> - GPIO12 working
> - LED blinks as expected
> - Pins saved to hardware-config.md
>
> Proceeding to project setup...

---

## Phase 2: Project Setup

**Goal**: Create lesson branch and directory structure with starter code.

### Step 2.1: Create Lesson Branch

```bash
# From main branch
git checkout -b lesson-{NN}-{name}
```

**Example**: `git checkout -b lesson-03-i2c-gdb`

### Step 2.2: Create Directory Structure

**Copy from previous lesson** (fastest):
```bash
cp -r lessons/01-blinky-gdb/ lessons/{NN}-{name}/
```

**Then update**:
- `Cargo.toml` - Package name, version
- `src/bin/main.rs` - Lesson number, description
- `.cargo/config.toml` - Verify espflash runner
- `rust-toolchain.toml` - Verify stable channel
- `build.rs` - Copy unchanged

**Standard structure**:
```
lessons/{NN}-{name}/
├── src/
│   ├── bin/
│   │   └── main.rs          # Main firmware (progressive commits)
│   └── lib.rs               # Empty (not used for simple lessons)
├── .cargo/
│   └── config.toml          # espflash runner
├── Cargo.toml               # Dependencies + [[bin]] section
├── rust-toolchain.toml      # Rust stable + RISC-V target
├── build.rs                 # Helpful linker errors
└── README.md                # Commit-by-commit walkthrough (created in Phase 4)
```

### Step 2.3: Verify Build

```bash
cd lessons/{NN}-{name}
cargo build --release
```

**If build fails**: Fix dependencies, configuration, then rebuild

**If build succeeds**: Proceed to progressive commits

---

## Phase 3: Progressive Commit Development

**Goal**: Implement lesson through progressive commits following the commit structure from `GDB_LESSON_PLANS.md`.

**Pedagogy**: Each commit builds on the previous, revealing functionality step-by-step. Students investigate with GDB between commits.

### Step 3.1: Commit 1 - Discovery Phase

**Purpose**: Give students something broken or minimal to investigate with GDB.

**Pattern A: Broken Firmware** (Lesson 01, 04)
- Code that compiles but doesn't work
- Missing configuration (GPIO enable, I2C clock, etc.)
- Students use GDB to find the issue

**Pattern B: Minimal Firmware** (Lesson 02, 03)
- Bare minimum initialization
- No actual functionality yet
- Students inspect registers to understand state

**Example (Lesson 01, Commit 1)**:
```rust
// Broken LED blink - missing GPIO enable!
#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Missing: GPIO enable configuration!

    loop {
        // Try to toggle LED - won't work
        unsafe {
            let gpio_out = 0x60091008 as *mut u32;
            *gpio_out ^= 1 << 12;  // Won't work - GPIO not enabled
        }
        delay_ms(500);
    }
}
```

**Commit message**:
```
feat(lesson-01): Commit 1 - Broken LED blink

LED won't toggle. Use GDB to investigate:
- Inspect GPIO registers
- Find missing enable configuration
- Discover the bug

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**Build and flash**:
```bash
cargo build --release
cargo run --release
```

**Verify**: LED doesn't blink (expected behavior for commit 1)

### Step 3.2: Commit 2 - First GDB Technique

**Purpose**: Introduce first GDB technique from lesson plan.

**Common patterns**:
- Memory inspection/writes (Lesson 01)
- Watchpoints (Lesson 02)
- Disassembly (Lesson 05)
- Remote memory dumps (Lesson 06)

**Example (Lesson 01, Commit 2)**:
```rust
// Remove LED code, control via GDB
#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Enable GPIO12
    unsafe {
        let gpio_enable = 0x60091024 as *mut u32;
        *gpio_enable |= 1 << 12;
    }

    // Empty loop - control LED from GDB
    loop {
        // Students will use GDB to write to GPIO_OUT register
        delay_ms(100);
    }
}
```

**Commit message**:
```
feat(lesson-01): Commit 2 - GDB register control

Enable GPIO, then control LED via GDB:
- set $gpio = 12
- set $mask = 1 << $gpio
- set *(uint32_t*)0x60091008 = $mask  # LED ON
- set *(uint32_t*)0x6009100C = $mask  # LED OFF

Teaches: Memory writes + GDB variables (bit math)

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**Build and flash**:
```bash
cargo build --release
cargo run --release
# Then connect GDB and run commands from commit message
```

### Step 3.3: Commit 3 - Second GDB Technique

**Purpose**: Introduce second GDB technique (usually the "wow moment").

**Example (Lesson 01, Commit 3)**:
```rust
// Add functions callable from GDB
#[no_mangle]
pub extern "C" fn led_on() {
    unsafe {
        let gpio_out = 0x60091008 as *mut u32;
        *gpio_out |= 1 << 12;
    }
}

#[no_mangle]
pub extern "C" fn led_off() {
    unsafe {
        let gpio_out = 0x6009100C as *mut u32;
        *gpio_out |= 1 << 12;
    }
}

#[no_mangle]
pub extern "C" fn led_toggle() {
    unsafe {
        let gpio_out = 0x60091018 as *mut u32;
        *gpio_out |= 1 << 12;
    }
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    unsafe {
        let gpio_enable = 0x60091024 as *mut u32;
        *gpio_enable |= 1 << 12;
    }

    loop {
        delay_ms(100);
    }
}
```

**Commit message**:
```
feat(lesson-01): Commit 3 - GDB function calls

Add led_on(), led_off(), led_toggle() functions.
Call from GDB while firmware runs:

(gdb) call led_on()
(gdb) call led_off()
(gdb) call led_toggle()

"Wow moment" - remote control from debugger!

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### Step 3.4: Additional Commits (If Needed)

**Follow lesson plan** - some lessons have 4-5 commits.

**Lesson 02 example**:
- Commit 1: UART init (minimal)
- Commit 2: Add data streaming
- Commit 3: Introduce DMA
- Commit 4: Watchpoints for buffer overflow
- Commit 5: Conditional breakpoints for errors

**Pattern**:
1. Build on previous commit
2. Add one new concept or GDB technique
3. Keep commit focused and small
4. Test on hardware before committing

### Step 3.5: Build-Flash-Test Loop

**For each commit**:

1. **Write code** following lesson plan
2. **Build**: `cargo build --release`
3. **Flash**: `cargo run --release`
4. **Test with GDB** (run demo commands from lesson plan)
5. **Verify expected behavior** (LED blinks, UART streams, etc.)
6. **Commit** with descriptive message
7. **Repeat** for next commit

**If issues occur**:
- Use GDB to debug (practice what you preach!)
- Fix and re-test
- Don't commit broken code

---

## Phase 4: Documentation

**Goal**: Create README with commit-by-commit walkthrough guiding students through discovery process.

### Step 4.1: Create README.md

**Structure** (follow lesson-01 pattern):

```markdown
# Lesson {NN}: {Feature} + GDB

{Brief description of what students will discover}

## Learning Objectives

By working through this lesson's commits, you'll learn:
- {GDB technique 1}
- {GDB technique 2}
- {GDB technique 3}
- {Peripheral concept}

## Hardware Requirements

- ESP32-C6 development board
- {External components}
- {Wiring details}

### Wiring Diagram

```
ESP32-C6        {Component}
--------        ----------
GPIO{X}    -->  {Pin}
GND        -->  GND
```

## Quick Start

```bash
cd lessons/{NN}-{name}

# Checkout first commit to start discovery
git checkout {commit-hash-1}

# Build and flash
cargo build --release
cargo run --release

# Follow commit-by-commit walkthrough below
```

## Commit-by-Commit Walkthrough

This lesson uses progressive commits to reveal functionality step-by-step.
Each commit builds on the previous, teaching new GDB techniques.

### Commit 1: {Title}

**What it does**: {Brief description}

**Expected behavior**: {What happens when you flash}

**Your task**: {What student should investigate with GDB}

**GDB commands to try**:
```gdb
{Demo commands from GDB_LESSON_PLANS.md}
```

**What you'll discover**: {Key insight}

**Next**: `git checkout {commit-hash-2}` to see the solution

---

### Commit 2: {Title}

**What it does**: {Brief description}

**Expected behavior**: {What happens}

**Your task**: {Investigation task}

**GDB commands**:
```gdb
{Demo commands}
```

**What you'll learn**: {Key GDB technique}

**Next**: `git checkout {commit-hash-3}`

---

### Commit 3: {Title}

**What it does**: {Brief description}

**Expected behavior**: {What happens}

**The "wow moment"**: {Most impressive GDB capability}

**GDB commands**:
```gdb
{Demo commands}
```

**What you'll discover**: {Key insight}

---

## Key Concepts

### {Concept 1}
{Explanation}

### {Concept 2}
{Explanation}

## GDB Reference

**Connecting GDB**:
```bash
# Terminal 1: Run firmware
cargo run --release

# Terminal 2: Attach GDB
riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/{binary}
(gdb) target remote :3333
```

**Useful commands**:
- `info registers` - Show all registers
- `x/16x 0x60091008` - Examine memory (hex)
- `set *(uint32_t*)0x60091008 = 0x1000` - Write memory
- `call function_name()` - Call Rust function
- `break main` - Set breakpoint
- `continue` - Resume execution
- `Ctrl-C` - Pause execution

## Troubleshooting

| Issue | Solution |
|-------|----------|
| {Common issue 1} | {Solution} |
| {Common issue 2} | {Solution} |

## Next Steps

- **Lesson {NN+1}**: {Next lesson topic}
- Experiment: {Suggested modifications}

## References

- [GDB_LESSON_PLANS.md](../../GDB_LESSON_PLANS.md) - Full curriculum
- [GDB_REFERENCE.md](../../GDB_REFERENCE.md) - All GDB capabilities
- [esp-hal {Peripheral} Docs](link)
- [ESP32-C6 Technical Reference](link)
```

### Step 4.2: Generate Commit Hashes

**After all commits are done**, generate commit hashes for README:

```bash
git log --oneline | head -n {commit_count}
```

**Replace placeholders** in README with actual commit hashes.

### Step 4.3: Update CLAUDE.md (Lesson-Specific)

**Create `lessons/{NN}-{name}/CLAUDE.md`** with lesson-specific pedagogy:

```markdown
# CLAUDE.md - Lesson {NN} Pedagogy

## Teaching Philosophy

**Discovery-based learning** - Students investigate with GDB, not just read code.

**Progressive commits** - Each commit reveals one concept. Students:
1. Checkout commit
2. Flash firmware
3. Investigate with GDB
4. Discover insight
5. Move to next commit

**Collaborative pair programming** - Not formal teaching. Conversational, exploratory.

## Lesson Structure

### Commit 1: {Purpose}
- {What student sees}
- {What they investigate}
- {What they discover}

### Commit 2: {Purpose}
- {What student sees}
- {GDB technique introduced}
- {Key insight}

### Commit 3: {Purpose}
- {The "wow moment"}
- {Most impressive capability}
- {Why this matters}

## GDB Techniques Taught

1. **{Technique 1}**: {Brief description}
   - Commands: {list}
   - Use case: {when to use}

2. **{Technique 2}**: {Brief description}
   - Commands: {list}
   - Use case: {when to use}

3. **{Technique 3}**: {Brief description}
   - Commands: {list}
   - Use case: {when to use}

## Interaction Style

**✅ DO**:
- Ask: "What do you see when you inspect that register?"
- Suggest: "Try calling led_toggle() from GDB"
- Collaborate: "Let's investigate the GPIO enable register together"
- Encourage: "That's interesting! What happens if you write 0x1000?"

**❌ DON'T**:
- Lecture: "The GPIO peripheral requires..."
- Formal: "In this lesson you will learn..."
- Passive: "Here's the solution, type this"

**Tone**: Pair programming buddy, not professor.

## Hardware Testing

**ALWAYS test before teaching**:
```bash
/test-hardware {peripheral} {pins}
```

**Never teach with untested hardware.**

## Student Questions

**Proactive guidance**:
- Link to GDB_REFERENCE.md for deep dives
- Point to commit history for comparison
- Encourage experimentation: "What if you change that value?"

**When stuck**:
- Review GDB commands together
- Inspect registers collaboratively
- Suggest alternative investigation approaches

---

**Last Updated**: {date}
**Lesson Status**: {Complete/In Progress}
```

---

## Phase 5: Hardware Validation

**Goal**: User confirms lesson works on real hardware through all commits.

### Step 5.1: Validation Instructions

> 📋 **Hardware Validation Required**
>
> The lesson is ready for validation. Please test the commit progression:
>
> **Setup**:
> 1. Connect hardware: {components}
> 2. Navigate to lesson: `cd lessons/{NN}-{name}`
>
> **Test each commit**:
>
> **Commit 1**:
> ```bash
> git checkout {hash-1}
> cargo run --release
> # Expected: {behavior}
> # GDB commands: {list}
> ```
>
> **Commit 2**:
> ```bash
> git checkout {hash-2}
> cargo run --release
> # Expected: {behavior}
> # GDB commands: {list}
> ```
>
> **Commit 3**:
> ```bash
> git checkout {hash-3}
> cargo run --release
> # Expected: {behavior}
> # GDB commands: {list}
> ```
>
> **Validation checklist**:
> - [ ] All commits build successfully
> - [ ] Hardware behaves as expected for each commit
> - [ ] GDB commands work as documented
> - [ ] "Wow moment" is impressive
> - [ ] Progression makes sense (discovery → solution)

### Step 5.2: User Checkpoint

**Ask for confirmation**:
> **Validation Results**
>
> Did all commits work correctly? Is the progression clear?
>
> - Type **"yes"** if everything worked
> - Type **"issue"** and describe what went wrong

**Wait for user response**

**If approved**: Proceed to PR
**If issues**: Fix and re-validate

---

## Phase 6: Push Branch and Create PR

**Goal**: Push lesson branch and create pull request for review.

### Step 6.1: Final Checks

```bash
# Format code
cargo fmt

# Check for warnings
cargo clippy

# Final build
cargo build --release
```

### Step 6.2: Push Branch

```bash
git push -u origin lesson-{NN}-{name}
```

### Step 6.3: Create Pull Request

```bash
gh pr create --title "Lesson {NN}: {Feature} + GDB" --body "$(cat <<'EOF'
## Summary

Implements Lesson {NN} demonstrating {peripheral} with {GDB techniques}.

**Learning objectives**:
- {Objective 1}
- {Objective 2}
- {Objective 3}

**GDB techniques taught**:
1. {Technique 1}
2. {Technique 2}
3. {Technique 3}

**Commit progression**:
- Commit 1: {Purpose}
- Commit 2: {Purpose}
- Commit 3: {Purpose}

**Hardware**:
- ESP32-C6 + {components}
- GPIO{X}: {function}

**Validation**:
- ✅ All commits build
- ✅ Hardware tested on ESP32-C6
- ✅ GDB commands verified
- ✅ Documentation complete

## Test Plan

```bash
cd lessons/{NN}-{name}

# Test commit progression
git checkout {hash-1} && cargo run --release
git checkout {hash-2} && cargo run --release
git checkout {hash-3} && cargo run --release
```

**Expected behavior**: {Summary of what should happen}

## References

- [GDB_LESSON_PLANS.md](../../GDB_LESSON_PLANS.md) - Lesson specification
- [Lesson {NN-1}](../lessons/{NN-1}-{prev-name}/) - Previous lesson

🤖 Generated with [Claude Code](https://claude.com/claude-code)
EOF
)"
```

### Step 6.4: Completion Summary

> ✅ **Lesson development complete!**
>
> **Summary**:
> - Lesson {NN}: {Feature} + GDB
> - Branch: `lesson-{NN}-{name}`
> - Commits: {N} progressive commits
> - Hardware validated: ✅
> - PR created: {PR URL}
>
> **Files Created**:
> - `lessons/{NN}-{name}/src/bin/main.rs` (progressive commits)
> - `lessons/{NN}-{name}/README.md` (commit walkthrough)
> - `lessons/{NN}-{name}/CLAUDE.md` (pedagogy guide)
>
> **Validation**:
> - ✅ All commits build
> - ✅ Hardware works through all commits
> - ✅ GDB commands verified
> - ✅ Documentation complete
>
> Lesson is ready for review!

---

## Key Principles

### 1. Discovery-Based Learning

**Students investigate, not just read**:
- Give them broken or minimal code
- Guide them to use GDB to discover issues
- Let them find solutions through exploration
- Build understanding through hands-on debugging

**Not**:
- Presenting complete solutions
- Formal lectures
- Passive reading

### 2. Progressive Commits

**Each commit reveals one concept**:
- Commit 1: Problem or minimal state
- Commit 2: First GDB technique (investigation)
- Commit 3: Second GDB technique ("wow moment")
- Commit 4+: Additional techniques as needed

**Students learn by**:
- Checking out each commit
- Flashing firmware
- Using GDB commands
- Discovering insights
- Moving to next commit

### 3. GDB-First Mindset

**GDB is not optional**:
- Every lesson teaches 2-3 GDB techniques
- GDB commands are in commit messages
- README includes GDB reference section
- Students use GDB throughout lesson

**Reference materials**:
- `GDB_LESSON_PLANS.md` - What to teach per lesson
- `GDB_REFERENCE.md` - All GDB capabilities
- `GDB_EXECUTIVE_SUMMARY.md` - Quick reference

### 4. Proactive Hardware Testing

**Test BEFORE teaching**:
```bash
/test-hardware {peripheral} {pins}
```

**Never teach with untested hardware**:
- Verify pins work
- Confirm expected behavior
- Document working configuration
- Save time debugging during lesson

### 5. Collaborative Pair Programming

**Tone**: Buddy exploring together, not professor teaching

**Good interactions**:
- "Let's inspect that register together"
- "What do you see in the GPIO_OUT register?"
- "Try calling led_toggle() from GDB - it's pretty cool!"

**Bad interactions**:
- "In this lesson you will learn..."
- "The correct answer is..."
- "Here's the solution, type this"

### 6. Keep It Simple

**Lean lessons**:
- 100-150 lines per commit (type-able in YouTube video)
- 3-5 commits total
- Focus on core concepts
- Skip edge cases
- Clear, readable code

**Not**:
- Over-engineering
- Massive documentation
- Exhaustive error handling
- Production-grade code

---

## Lesson Curriculum Reference

**7-lesson GDB curriculum** (from `GDB_LESSON_PLANS.md`):

| Lesson | Peripheral | GDB Techniques | Complexity | Duration |
|--------|------------|----------------|------------|----------|
| 01 | GPIO (LED) | Memory ops, GDB variables, function calls | ⭐⭐☆☆☆ | 60-90 min |
| 02 | UART + DMA | Watchpoints, conditional breakpoints, call stack | ⭐⭐⭐☆☆ | 90-120 min |
| 03 | I2C (sensor) | Reverse continue, register diff, tracepoints | ⭐⭐⭐☆☆ | 90-120 min |
| 04 | SPI (OLED) | Python scripting, macro debugger, memory compare | ⭐⭐⭐⭐☆ | 120-150 min |
| 05 | PWM (servo) | Disassembly, instruction stepping, performance analysis | ⭐⭐⭐☆☆ | 90-120 min |
| 06 | Multi-peripheral | Core dumps, remote memory, checkpoint restore | ⭐⭐⭐⭐☆ | 120-180 min |
| 07 | Production debug | Automated test harness, trace analysis, historical debugging | ⭐⭐⭐⭐⭐ | 150-240 min |

**Always consult** `GDB_LESSON_PLANS.md` for detailed commit structures and demo scripts.

---

## When to Use This Workflow

| Use Case | Use `/gen-lesson` | Notes |
|----------|------------------|-------|
| New GDB lesson (01-07) | ✅ Yes | Full workflow with progressive commits |
| New peripheral (I2C, SPI, PWM) | ✅ Yes | Integrate GDB techniques |
| Bug fix in existing lesson | ⚠️ Maybe | Fix and test, update README |
| Documentation update | ❌ No | Just edit directly |
| Hardware troubleshooting | ❌ No | Use `/test-hardware` |

---

## Tips for Best Results

**Good lesson requests**:
- ✅ "Create Lesson 03: I2C + GDB"
- ✅ "Implement Lesson 02: UART + DMA with watchpoints"
- ✅ "Add Lesson 05: PWM with disassembly"

**Poor lesson requests**:
- ❌ "Make I2C work" (which lesson? what GDB techniques?)
- ❌ "Fix the code" (which code? what issue?)

**Prepare for success**:
- Read `GDB_LESSON_PLANS.md` for the target lesson
- Have ESP32-C6 connected and ready
- Have external components available
- Know which GPIO pins to use
- Be ready to test all commits

---

**This is your GDB-integrated ESP32-C6 lesson development workflow. Use it for systematic firmware development with progressive commits and discovery-based learning for engineers mastering esp-hal 1.0.0 Rust + GDB debugging.**

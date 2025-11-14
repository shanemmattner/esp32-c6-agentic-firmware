# Lesson 01: Commit Breakdown Plan

This document outlines how to break the complete Lesson 01 into instructional commits that students will step through sequentially.

## Current State

**Single commit:** `bdb1c97` - Complete lesson with all features

**Goal:** Break into 6-8 commits that reveal the lesson progressively

---

## Proposed Commit Sequence

### Commit 1: Project Structure
**Branch point:** `lesson-01-step-1`

**What's included:**
```
lessons/01-gdb-blinky/
├── .cargo/config.toml
├── Cargo.toml
├── build.rs
├── rust-toolchain.toml
└── src/
    ├── bin/
    │   └── main.rs (empty template)
    └── lib.rs (empty)
```

**Message:**
```
feat(lesson-01): Initialize project structure

- Create Cargo.toml with minimal dependencies
- Add build configuration (.cargo/config.toml)
- Set up Rust toolchain (nightly)
- Empty source files

Students learn: Project setup for embedded Rust
```

**Student instruction:**
> "This is a standard ESP32-C6 Rust project structure. Notice:
> - `Cargo.toml` has minimal deps (just esp-hal, no GPIO drivers)
> - `.cargo/config.toml` configures espflash as runner
> - `build.rs` handles linker scripts
> - Source code doesn't exist yet - we'll add it next"

---

### Commit 2: Minimal Blank Firmware
**Branch point:** `lesson-01-step-2`

**What's added:**
```
src/bin/main.rs (complete implementation with comments)
```

**Message:**
```
feat(lesson-01): Add minimal blank firmware

- Infinite loop with 500ms delays
- No GPIO configuration
- No LED control code
- Just timing infrastructure

The firmware provides timing; GDB will provide control.
```

**Student instruction:**
> "Look at main.rs - notice what's NOT here:
> - No GPIO configuration
> - No LED control
> - Just a delay loop
>
> This is intentional. The firmware provides timing infrastructure.
> GDB will do everything else.
>
> Try building it: `cargo build --release`"

**Claude Code task:**
> "Student: Can you build and flash this firmware?"
> Claude: *Runs build command, flashes, captures boot output*
> Claude: "Firmware is running! Notice the boot message says 'No LED control code'.
>         This is correct - we'll use GDB to control the LED."

---

### Commit 3: Register Discovery Tool
**Branch point:** `lesson-01-step-3`

**What's added:**
```
scripts/find-registers.py
.claude/commands/find-registers.md
```

**Message:**
```
feat(lesson-01): Add register discovery tool

- Python script to search PAC crate for registers
- Finds peripheral base addresses and offsets
- Generates GDB commands
- Claude Code command wrapper

Teaching: How to find hardware registers from source code
```

**Student instruction:**
> "Before we can control the LED, we need to know the GPIO register addresses.
>
> Instead of reading the 1000-page datasheet, we'll search the esp32c6 PAC crate.
>
> Try: `python3 scripts/find-registers.py GPIO`
>
> Or use Claude: `/find-registers GPIO`"

**Claude Code interaction:**
```
Student: "How do I find the GPIO registers?"

Claude: "Let me search the PAC crate..."
[runs find-registers.py]

Claude: "Found them! Here's what I discovered:

Base Address: 0x60091000

Key registers for LED control:
- ENABLE_W1TS (0x24): 0x60091024 - Enable GPIO as output
- OUT_W1TS (0x08): 0x60091008 - Set GPIO high (ON)
- OUT_W1TC (0x0C): 0x6009100C - Clear GPIO low (OFF)

For GPIO8, use bit mask: 0x100 (that's 1 << 8)

This discovery process is one of Rust's superpowers - the hardware
abstraction is self-documenting!"
```

---

### Commit 4: GPIO Register Documentation
**Branch point:** `lesson-01-step-4`

**What's added:**
```
GPIO_REGISTERS.md
```

**Message:**
```
docs(lesson-01): Document ESP32-C6 GPIO registers

- Complete GPIO register map from PAC crate
- Address calculations (base + offset)
- GDB command examples
- Bit field explanations

Reference for manual GDB control
```

**Student instruction:**
> "Open GPIO_REGISTERS.md - this documents what we discovered.
>
> Key sections:
> 1. Base address: 0x60091000
> 2. Register offsets
> 3. Absolute addresses (base + offset)
> 4. Example GDB commands
>
> This is your reference manual for controlling GPIO via GDB."

---

### Commit 5: Manual GDB Control Script
**Branch point:** `lesson-01-step-5`

**What's added:**
```
lessons/01-gdb-blinky/gdb_scripts/manual_control.gdb
```

**Message:**
```
feat(lesson-01): Add interactive GDB control script

- Step-by-step LED control commands
- Teaches register manipulation
- Verifiable state inspection
- Educational GDB workflow

Students manually control LED to understand concepts
```

**Student instruction:**
> "Now let's control the LED manually using GDB.
>
> 1. Start debug server: `probe-rs attach --chip esp32c6 target/.../main`
> 2. Connect GDB: `riscv32-esp-elf-gdb target/.../main`
> 3. Load script: `(gdb) source gdb_scripts/manual_control.gdb`
> 4. Follow prompts: `step1`, `step2`, `step3`, `step4`
>
> Each step teaches one concept:
> - step1: Enable GPIO output
> - step2: Turn LED ON
> - step3: Turn LED OFF
> - step4: Read register state"

**Expected experience:**
```
(gdb) source gdb_scripts/manual_control.gdb
Interactive Learning Mode
Type: step1

(gdb) step1
Step 1: Enable GPIO8 as output
Command: set *(uint32_t*)0x60091024 = 0x100
✓ GPIO8 enabled as output
Next: Type 'step2' to turn LED ON

(gdb) step2
Step 2: Turn LED ON
Command: set *(uint32_t*)0x60091008 = 0x100
✓ LED should be ON now!
Check the LED. Is it on?
```

---

### Commit 6: Automated Blinking Script
**Branch point:** `lesson-01-step-6`

**What's added:**
```
lessons/01-gdb-blinky/gdb_scripts/blinky.gdb
```

**Message:**
```
feat(lesson-01): Add automated blinking GDB script

- Breakpoint-based automation
- 500ms blink interval
- Toggle function with state tracking
- Manual control commands (led_on, led_off, toggle_led)

Demonstrates GDB automation capabilities
```

**Student instruction:**
> "Manual control proves the concept. Now let's automate it!
>
> 1. Connect GDB (if not already)
> 2. `(gdb) source gdb_scripts/blinky.gdb`
> 3. `(gdb) continue`
>
> The LED should blink automatically every 500ms.
>
> How it works:
> - Breakpoint on delay loop (500ms interval)
> - Breakpoint commands toggle LED automatically
> - No firmware code involved!"

---

### Commit 7: Claude Code Interactive Guide
**Branch point:** `lesson-01-step-7`

**What's added:**
```
.claude/commands/gdb-blinky.md
```

**Message:**
```
feat(lesson-01): Add Claude Code interactive guide

- Socratic teaching approach
- Guides register discovery process
- Helps troubleshoot issues
- Provides encouragement and context

Students learn WITH Claude, not just FROM Claude
```

**Student instruction:**
> "Want a guided experience? Use Claude Code!
>
> `/gdb-blinky`
>
> Claude will:
> 1. Help you discover GPIO registers
> 2. Guide you through manual control
> 3. Explain what's happening
> 4. Troubleshoot problems
> 5. Answer questions along the way
>
> This is agentic learning - you're discovering concepts, not just copying code."

**Example interaction:**
```
Student: /gdb-blinky

Claude: "Welcome to Lesson 01! We're going to make an LED blink using
        ONLY GDB commands. No firmware code.

        Before we begin, do you know where to find GPIO register addresses?"

Student: "No"

Claude: "Great question! There are three ways:
        1. Read the 1000-page TRM (slow)
        2. Use SVD files (needs tools)
        3. Search the PAC crate (fastest!)

        Want me to search the esp32c6 PAC crate for you?"

Student: "Yes"

Claude: [shows register discovery process]
        "Found it! Let me explain what each register does..."
```

---

### Commit 8: Comprehensive Documentation
**Branch point:** `lesson-01-step-8` (final)

**What's added:**
```
lessons/01-gdb-blinky/README.md
```

**Message:**
```
docs(lesson-01): Add comprehensive README

- Quick start guide
- Three learning paths (automated, interactive, guided)
- Concept explanations
- Troubleshooting guide
- Challenges for practice
- Connection to next lesson

Complete reference for Lesson 01
```

**Student instruction:**
> "Congratulations! You've completed Lesson 01.
>
> Review README.md for:
> - Summary of what you learned
> - Alternative learning paths
> - Practice challenges
> - Troubleshooting tips
>
> Next: Lesson 02 - High-Speed UART with DMA"

---

## Student Progression Flow

```
1. Clone repo
2. `git checkout lesson-01-step-1`
3. Read commit message
4. Explore files
5. Run Claude Code (automatic context loading)
6. Complete step's objectives
7. `git checkout lesson-01-step-2`
8. Repeat until lesson-01-step-8
```

## Claude Code Auto-Context

Each checkout automatically triggers:

```python
# .claude/hooks/on_git_checkout.py
def on_checkout(commit_hash):
    step = extract_step_number(commit_hash)

    context = f"""
    Student is on Lesson 01, Step {step}

    Files in this commit:
    {list_changed_files(commit_hash)}

    Lesson objectives for this step:
    {load_step_objectives(step)}

    Your role: Guide discovery, don't give answers immediately.
    Celebrate milestones. Be encouraging.
    """

    return context
```

Claude knows where the student is and what they should learn.

---

## Implementation Steps

### Option A: Interactive Rebase (Recommended)

```bash
git checkout lesson-01
git reset --soft <commit-before-lesson-01>
# Now stage and commit files in the order above
```

### Option B: Cherry-pick Approach

```bash
# Create step-1 branch
git checkout -b lesson-01-step-1 <base-commit>
git add lessons/01-gdb-blinky/Cargo.toml ...
git commit -m "feat(lesson-01): Initialize project structure"

# Create step-2 branch from step-1
git checkout -b lesson-01-step-2 lesson-01-step-1
git add src/bin/main.rs
git commit -m "feat(lesson-01): Add minimal blank firmware"

# Continue for all 8 steps
```

### Option C: Use Script

```bash
./scripts/create-lesson-commits.sh lesson-01
```

---

## Validation

After creating commits, verify:

1. **Each commit builds:**
   ```bash
   git checkout lesson-01-step-N
   cd lessons/01-gdb-blinky
   cargo build --release
   ```

2. **Logical progression:**
   - Step 1: Just structure (can't run)
   - Step 2: Can flash and run (but LED doesn't work)
   - Step 3-4: Can discover registers
   - Step 5: Can manually control LED
   - Step 6: Can automate blinking
   - Step 7-8: Full experience with docs

3. **Claude Code integration:**
   - Each step loads appropriate context
   - Commands are available when needed
   - Hints are step-appropriate

---

## YouTube Video Structure

With commit-based lessons:

**[00:00-01:00] Introduction**
- Show final result (LED blinking via GDB)
- "This lesson has 8 steps. Watch Claude build it."

**[01:00-02:00] Step 1: Project Setup**
- `git checkout lesson-01-step-1`
- Claude creates project structure
- No typing from me

**[02:00-04:00] Step 2: Blank Firmware**
- `git checkout lesson-01-step-2`
- Claude builds and flashes
- LED doesn't work (yet!)

**[04:00-07:00] Step 3-4: Register Discovery**
- Claude searches PAC crate
- Shows discovery process
- Generates GPIO register map

**[07:00-10:00] Step 5-6: LED Control**
- Manual GDB commands
- Automated blinking
- LED finally works!

**[10:00-12:00] Step 7-8: Polish**
- Claude Code guide
- Documentation
- Challenges

**[12:00-13:00] What We Learned**
- Review concepts
- Preview Lesson 02

---

## Success Criteria

Students should be able to:

✅ Build an ESP32-C6 project from scratch
✅ Find hardware registers using PAC crates
✅ Control GPIO via GDB commands
✅ Understand memory-mapped I/O
✅ Automate debugging with GDB scripts
✅ Work collaboratively with Claude Code

**Without writing GPIO control code themselves.**

---

This commit structure enables:
1. Progressive revelation of concepts
2. Checkpoint-based learning (students can stop and resume)
3. Video-friendly pacing (one commit = one video section)
4. Claude Code contextual awareness (knows student progress)
5. Reusable for other lessons (template approach)

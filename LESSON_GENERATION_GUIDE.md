# Lesson Generation Guide

This guide explains how to use the `/gen-lesson` and `/gen-all-lessons` commands to create the ESP32-C6 + GDB curriculum.

## Quick Start

### Generate All Lessons (Recommended for Complete Curriculum)

```bash
# From main branch
/gen-all-lessons
```

This will sequentially create all 7 lessons following the GDB curriculum in `GDB_LESSON_PLANS.md`.

**Time estimate**: 14-28 hours total (spread over 4 days recommended)

### Generate Single Lesson

```bash
# From main branch
/gen-lesson "Create Lesson 01: GPIO + GDB Fundamentals"
```

Use this for creating individual lessons or testing the workflow.

---

## Available Commands

### `/gen-lesson`

**Purpose**: Generate a single GDB lesson with progressive commits

**Location**: `.claude/commands/gen-lesson.md`

**Workflow**:
1. Review lesson plan from `GDB_LESSON_PLANS.md`
2. Proactive hardware testing with `/test-hardware`
3. Create lesson branch
4. Progressive commit development (3-6 commits)
5. Documentation with commit walkthrough
6. Hardware validation
7. PR creation

**Example**:
```bash
/gen-lesson "Create Lesson 03: I2C + GDB"
```

### `/gen-all-lessons`

**Purpose**: Orchestrate creation of all 7 lessons sequentially

**Location**: `.claude/commands/gen-all-lessons.md`

**Workflow**:
- Pre-flight checklist (hardware, software, repo state)
- Sequential lesson generation (01 → 07)
- Hardware validation checkpoints
- PR creation and merge workflow
- Progress tracking

**Example**:
```bash
/gen-all-lessons
```

---

## GDB Curriculum Overview

From `GDB_LESSON_PLANS.md`:

| Lesson | Peripheral | GDB Techniques | Complexity | Duration |
|--------|------------|----------------|------------|----------|
| 01 | GPIO (LED) | Memory ops, GDB variables, function calls | ⭐⭐☆☆☆ | 60-90 min |
| 02 | UART + DMA | Watchpoints, conditional breakpoints, call stack | ⭐⭐⭐☆☆ | 90-120 min |
| 03 | I2C (sensor) | Reverse continue, register diff, tracepoints | ⭐⭐⭐☆☆ | 90-120 min |
| 04 | SPI (OLED) | Python scripting, macro debugger, memory compare | ⭐⭐⭐⭐☆ | 120-150 min |
| 05 | PWM (servo) | Disassembly, instruction stepping, performance analysis | ⭐⭐⭐☆☆ | 90-120 min |
| 06 | Multi-peripheral | Core dumps, remote memory, checkpoint restore | ⭐⭐⭐⭐☆ | 120-180 min |
| 07 | Production debug | Automated test harness, trace analysis, historical debugging | ⭐⭐⭐⭐⭐ | 150-240 min |

---

## Reference Documentation

### Planning Documents

- **`GDB_EXECUTIVE_SUMMARY.md`** (5KB) - Quick reference for lesson planning
  - 13 GDB capability categories
  - 7-lesson curriculum overview
  - Top 10 "wow moments"
  - Decision matrix for technique selection

- **`GDB_LESSON_PLANS.md`** (17KB) - Detailed lesson implementations
  - Full breakdown of all 7 lessons
  - Commit structures for each lesson
  - Demo GDB scripts
  - Pedagogical rationale
  - Hardware requirements

- **`GDB_REFERENCE.md`** (28KB) - Comprehensive technical reference
  - All 13 GDB capability categories with examples
  - ESP32-C6 peripheral register map
  - Command syntax reference
  - Advanced debugging workflows

### Command Documentation

- **`.claude/commands/gen-lesson.md`** (23KB) - Single lesson generation workflow
- **`.claude/commands/gen-all-lessons.md`** (18KB) - Complete curriculum orchestration

---

## Pedagogy

### Discovery-Based Learning

**Students investigate with GDB, not just read code**:
- Give them broken or minimal code
- Guide them to use GDB to discover issues
- Let them find solutions through exploration
- Build understanding through hands-on debugging

### Progressive Commits

**Each commit reveals one concept**:
- Commit 1: Problem or minimal state (discovery phase)
- Commit 2: First GDB technique (investigation)
- Commit 3: Second GDB technique ("wow moment")
- Commit 4+: Additional techniques as needed

**Students learn by**:
1. Checking out each commit
2. Flashing firmware
3. Using GDB commands
4. Discovering insights
5. Moving to next commit

### GDB-First Mindset

**GDB is not optional**:
- Every lesson teaches 2-3 GDB techniques
- GDB commands are in commit messages
- README includes GDB reference section
- Students use GDB throughout lesson

---

## Prerequisites

### Hardware

- ESP32-C6 development board (USB-C cable)
- LED + 220Ω resistor (Lesson 01)
- FTDI UART adapter (Lesson 02)
- I2C sensor - BME280 or MPU6050 (Lesson 03)
- SPI OLED display - SSD1306 (Lesson 04)
- Servo motor (Lesson 05)
- Breadboard, jumper wires

### Software

```bash
# Verify tooling
cargo --version          # Rust
espflash --version       # ESP flashing
probe-rs --version       # Debugging
riscv32-esp-elf-gdb --version  # GDB for RISC-V

# Verify esp-hal version
cargo search esp-hal     # Should show 1.0.0+
```

### Repository State

```bash
# Must be on main branch
git branch --show-current  # Should show "main"

# Must have clean working directory
git status  # Should show "nothing to commit, working tree clean"

# Must have GDB documentation
ls -la GDB_*.md  # Should show 3 files
```

---

## Workflow Example

### Complete Curriculum Generation

```bash
# 1. Verify prerequisites
git checkout main
git status  # Clean working directory
ls -la GDB_*.md  # 3 files present

# 2. Start curriculum generation
/gen-all-lessons

# 3. Follow prompts for each lesson:
#    - Lesson 01: GPIO + GDB
#      → Hardware test → Implementation → Validation → PR
#    - Lesson 02: UART + DMA
#      → Hardware test → Implementation → Validation → PR
#    - ... (continue for all 7 lessons)

# 4. Merge PRs as lessons complete
gh pr merge <PR-number> --squash
git checkout main
git pull origin main

# 5. Track progress
# Update tracking document after each lesson
```

### Single Lesson Generation

```bash
# 1. Review lesson specification
cat GDB_LESSON_PLANS.md  # Read Lesson 03 section

# 2. Test hardware first
/test-hardware i2c 6 7

# 3. Generate lesson
/gen-lesson "Create Lesson 03: I2C + GDB"

# 4. Agent will:
#    - Create branch lesson-03-i2c-gdb
#    - Implement 4 progressive commits
#    - Create README with commit walkthrough
#    - Request hardware validation

# 5. Validate on hardware
cd lessons/03-i2c-gdb
git checkout <commit-1-hash> && cargo run --release
git checkout <commit-2-hash> && cargo run --release
# ... test all commits

# 6. Confirm validation
# Type "yes" to proceed

# 7. Review and merge PR
gh pr view <PR-number>
gh pr merge <PR-number> --squash
```

---

## Tips for Success

### Prepare Hardware Ahead of Time

- Have all components ready before starting
- Test each peripheral individually with `/test-hardware`
- Document working pin configurations
- Keep breadboard organized

### Take Breaks

- Don't try to do all 7 lessons in one session
- Review lesson plan between lessons
- Test thoroughly at each checkpoint
- Recommended: 1-2 lessons per session

### Reference Materials

- Keep `GDB_LESSON_PLANS.md` open while working
- Consult `GDB_REFERENCE.md` for GDB commands
- Review `GDB_EXECUTIVE_SUMMARY.md` for quick lookup

### Checkpoint Strategy

**After each lesson**:
1. Merge PR to main
2. Update progress tracker
3. Take a break (15-30 minutes)
4. Review next lesson specification
5. Proceed to next lesson

---

## Troubleshooting

### Hardware Issues

**Problem**: Hardware test fails

**Solution**:
1. Check wiring (refer to lesson wiring diagrams)
2. Verify component works (test with multimeter)
3. Try different GPIO pins
4. Consult ESP32-C6 datasheet for pin capabilities

### Build Issues

**Problem**: Cargo build fails

**Solution**:
1. Verify esp-hal version: `cargo tree | grep esp-hal`
2. Check `Cargo.toml` has correct dependencies
3. Review esp-hal 1.0.0 migration guide
4. Consult `.claude/commands/gen-lesson.md` for correct structure

### GDB Issues

**Problem**: GDB commands don't work

**Solution**:
1. Verify GDB connected: `target remote :3333`
2. Check firmware is running: `cargo run --release` in separate terminal
3. Review `GDB_REFERENCE.md` for correct syntax
4. Ensure probe-rs or OpenOCD is running

---

## Expected Outcomes

After running `/gen-all-lessons`, you'll have:

### Repository Structure

```
esp32-c6-agentic-firmware/
├── lessons/
│   ├── 01-gpio-gdb/
│   │   ├── src/bin/main.rs (3 progressive commits)
│   │   ├── README.md (commit walkthrough)
│   │   └── CLAUDE.md (pedagogy guide)
│   ├── 02-uart-dma/
│   │   ├── src/bin/main.rs (5 progressive commits)
│   │   └── ...
│   ├── 03-i2c-gdb/
│   ├── 04-spi-oled/
│   ├── 05-pwm-servo/
│   ├── 06-multi-peripheral/
│   └── 07-production-debug/
├── GDB_EXECUTIVE_SUMMARY.md
├── GDB_LESSON_PLANS.md
├── GDB_REFERENCE.md
└── LESSON_GENERATION_GUIDE.md (this file)
```

### Documentation

- 7 lesson READMEs with commit-by-commit walkthroughs
- 7 lesson CLAUDE.md files with pedagogy guidance
- Hardware wiring diagrams for each lesson
- GDB command references
- Troubleshooting guides

### Learning Path

- Students can follow lessons 01 → 07 sequentially
- Each lesson builds on previous GDB knowledge
- Progressive commits reveal concepts step-by-step
- Complete curriculum covers 13+ GDB capabilities

---

## Next Steps

1. **Read this guide completely**
2. **Review `GDB_LESSON_PLANS.md`** to understand curriculum
3. **Verify prerequisites** (hardware, software, repo state)
4. **Choose workflow**:
   - `/gen-all-lessons` for complete curriculum
   - `/gen-lesson` for individual lessons
5. **Start generating lessons!**

---

**Good luck building the ESP32-C6 + GDB discovery-based learning curriculum!**

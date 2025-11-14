# ESP32-C6 Agentic Firmware Development

![ESP32-C6](https://img.shields.io/badge/ESP32--C6-Rust-orange)
![esp-hal](https://img.shields.io/badge/esp--hal-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Overview

**Revolutionary approach to embedded Rust development:** Learn debugging tools FIRST, then build features with full hardware visibility.

This repository teaches embedded systems development for ESP32-C6 using **esp-hal 1.0.0** with a **debugging-first methodology** and **AI-assisted learning** through Claude Code.

**What makes this different:**
- ✅ GDB + UART debugging infrastructure taught in Lesson 01
- ✅ All subsequent lessons use these tools for rapid development
- ✅ Claude Code integration for agentic learning
- ✅ Commit-based progressive lessons (watch AI work, not typing)
- ✅ PAC-based register discovery (no datasheet diving)

---

## New Lesson Sequence (Debugging-First)

**Branch:** `lesson-01` (🚀 Ready for testing)

### Lesson 01: GDB-Only LED Blinky ✅ Complete
**Status:** Compilation tested, ready for hardware validation

**What you'll learn:**
- Control hardware via GDB without writing firmware code
- Find registers by searching PAC crate source
- Memory-mapped I/O fundamentals
- GDB automation with breakpoint commands
- Agentic development with Claude Code

**Key innovation:** Make LED blink using ONLY GDB commands - no GPIO code in firmware!

**Files:** `lessons/01-gdb-blinky/`
- Minimal firmware (just timing loops)
- Register discovery tool (`scripts/find-registers.py`)
- GDB automation scripts (blinky.gdb, manual_control.gdb)
- Claude Code integration (`/gdb-blinky`, `/find-registers`)
- Complete documentation (README, QUICKSTART, test report)

**Quick start:**
```bash
git checkout lesson-01
cd lessons/01-gdb-blinky
cargo build --release
# See README.md for GDB workflow
```

---

### Lesson 02: UART + DMA (Planned)
**Goal:** Build high-speed UART with DMA using GDB to develop rapidly

**What you'll learn:**
- UART peripheral configuration
- DMA for zero-CPU streaming
- Baud rate tuning experiments
- `/improve-command` - AI self-improvement from conversation logs
- GDB register inspection during development

**Sources:** Will use code from archived lessons 06 + 08

---

### Lesson 03: GDB + UART Tandem Debugging (Planned)
**Goal:** Combine GDB + UART for bidirectional hardware control

**What you'll learn:**
- Variable streaming over UART
- GDB slot redirection
- Memory-safe pointer system
- Autonomous debugging with Claude Code
- RTT comparison and analysis

**Sources:** Will use code from archived lesson 08

---

## Archived Lessons (Reference)

Previous lesson sequence preserved in `lessons-archive/`:
- 01-button-neopixel (GPIO, RMT)
- 02-task-scheduler (Atomics, scheduling)
- 03-mpu9250 (I2C, sensors)
- 04-static-color-navigator (State machines)
- 05-unit-and-integration-testing
- 06-uart-terminal ⭐ (Will use for new Lesson 02)
- 07-gdb-debugging ⭐ (Merged into new Lesson 01)
- 08-uart-gdb-tandem ⭐ (Will use for new Lesson 03)

**See [ARCHIVED_LESSONS.md](./ARCHIVED_LESSONS.md) for detailed documentation and reusability matrix.**

**Why archived:** New sequence teaches debugging tools first (GDB, UART) before building complex features. Pedagogically superior for AI-assisted development.

---

## Quick Start

### Prerequisites

```bash
# Install Rust and RISC-V target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add riscv32imac-unknown-none-elf

# Install ESP32 tools
cargo install espflash probe-rs --locked

# Optional: GDB for debugging
# macOS: brew install riscv-gnu-toolchain
# Linux: apt install gdb-multiarch
```

### Try Lesson 01

```bash
git clone https://github.com/shanemmattner/esp32-c6-agentic-firmware
cd esp32-c6-agentic-firmware
git checkout lesson-01

cd lessons/01-gdb-blinky
cargo build --release

# See README.md for complete workflow
```

---

## Key Features

### 1. PAC-Based Register Discovery

Instead of reading 1000-page datasheets:

```bash
# Find any peripheral's registers
python3 scripts/find-registers.py GPIO

# Output:
# Base Address: 0x60091000
# OUT_W1TS (0x08): 0x60091008 - Write 1 to set
# OUT_W1TC (0x0C): 0x6009100C - Write 1 to clear
```

**Or use Claude Code:**
```
/find-registers UART0
```

This teaches the **methodology** of finding hardware information from Rust source code.

### 2. GDB as Hardware Controller

Control peripherals directly via GDB:

```gdb
# Enable GPIO8 as output
(gdb) set *(uint32_t*)0x60091024 = 0x100

# Turn LED ON
(gdb) set *(uint32_t*)0x60091008 = 0x100

# Turn LED OFF
(gdb) set *(uint32_t*)0x6009100C = 0x100
```

No firmware code needed - pure register manipulation!

### 3. Claude Code Integration

Every lesson includes custom slash commands:

- `/gdb-blinky` - Interactive GDB lesson guide
- `/find-registers` - Register discovery helper
- `/improve-command` - AI learns from your conversations (Lesson 02)

**Agentic learning:** Claude guides discovery, doesn't just give answers.

### 4. Commit-Based Lessons

Each lesson broken into 6-8 progressive commits:

```bash
git checkout lesson-01-step-1  # Project structure
git checkout lesson-01-step-2  # Blank firmware
git checkout lesson-01-step-3  # Register discovery
...
git checkout lesson-01-step-8  # Complete lesson
```

**Perfect for:**
- YouTube videos ("watch Claude work")
- Self-paced learning
- Checkpointed progress

---

## Documentation

### Lesson 01 Documentation
- **[lessons/01-gdb-blinky/README.md](./lessons/01-gdb-blinky/README.md)** - Comprehensive guide (426 lines)
- **[lessons/01-gdb-blinky/QUICKSTART.md](./lessons/01-gdb-blinky/QUICKSTART.md)** - 5-minute quick start
- **[GPIO_REGISTERS.md](./GPIO_REGISTERS.md)** - Complete ESP32-C6 GPIO register map
- **[LESSON_01_COMMIT_PLAN.md](./LESSON_01_COMMIT_PLAN.md)** - 8-step commit breakdown strategy
- **[LESSON_01_TEST_REPORT.md](./LESSON_01_TEST_REPORT.md)** - Compilation and tooling test results

### General Documentation
- **[CLAUDE.md](./CLAUDE.md)** - Guidelines for Claude Code development
- **[ARCHIVED_LESSONS.md](./ARCHIVED_LESSONS.md)** - Old lessons reference
- **[Official esp-hal Docs](https://docs.esp-rs.org/esp-hal/)** - HAL reference

---

## Project Structure

```
├── lessons/
│   └── 01-gdb-blinky/          # ✅ Complete (lesson-01 branch)
├── lessons-archive/             # Previous lesson sequence (reference)
│   ├── 01-button-neopixel/
│   ├── 02-task-scheduler/
│   ├── 03-mpu9250/
│   ├── 04-static-color-navigator/
│   ├── 05-unit-and-integration-testing/
│   ├── 06-uart-terminal/       # ⭐ Use for new Lesson 02
│   ├── 07-gdb-debugging/       # ⭐ Merged into new Lesson 01
│   └── 08-uart-gdb-tandem/     # ⭐ Use for new Lesson 03
├── scripts/
│   ├── find-registers.py       # Register discovery tool
│   └── find-esp32-ports.sh     # Auto port detection
├── .claude/
│   ├── commands/               # Slash commands for Claude Code
│   │   ├── gdb-blinky.md      # Interactive GDB lesson
│   │   └── find-registers.md  # Register search helper
│   └── templates/              # Code templates
└── README.md                    # This file
```

---

## Development Philosophy

### Debugging-First Approach

**Traditional:** Build features → hope they work → debug when they fail

**Our approach:**
1. **Lesson 01:** Learn GDB + register discovery
2. **Lesson 02:** Learn UART + DMA streaming
3. **Lesson 03:** Combine into tandem debugging superpower
4. **Lesson 04+:** Build features with full visibility

**Result:** Rapid development with continuous hardware feedback

### Agentic Learning

**Traditional tutorials:** "Follow these steps, type this code"

**Our approach:**
- Claude Code guides discovery (Socratic method)
- Students find registers themselves (with AI help)
- AI learns from conversations (`/improve-command`)
- Commit-based progression (self-paced)

**Result:** Understanding over memorization

---

## Hardware Requirements

- ESP32-C6 DevKit (recommended: ESP32-C6-DevKitC-1)
- USB cable for programming/debugging
- Optional: FTDI adapter for UART lessons
- Optional: Sensors (MPU9250, etc.) for advanced lessons

**All lessons work with just the dev board** - external hardware optional.

---

## Current Status

| Component | Status | Notes |
|-----------|--------|-------|
| Lesson 01 | ✅ Complete | Compilation tested, ready for hardware |
| Register discovery tool | ✅ Working | Tested on 70 peripherals |
| GDB automation scripts | ✅ Working | Syntax validated |
| Claude Code commands | ✅ Ready | /gdb-blinky, /find-registers |
| Documentation | ✅ Complete | README, QUICKSTART, test report |
| Lesson 02 | 📝 Planned | UART + DMA + /improve-command |
| Lesson 03 | 📝 Planned | GDB + UART tandem debugging |
| Hardware testing | ⏸️ Pending | Awaiting ESP32-C6 board |

---

## Next Steps

**For contributors:**
1. Test Lesson 01 on hardware
2. Create 8-step commit breakdown (see LESSON_01_COMMIT_PLAN.md)
3. Start Lesson 02 (UART + DMA)

**For students:**
1. Clone repo: `git clone https://github.com/shanemmattner/esp32-c6-agentic-firmware`
2. Checkout lesson: `git checkout lesson-01`
3. Follow README: `lessons/01-gdb-blinky/README.md`

---

## License

MIT OR Apache-2.0

---

## Acknowledgments

- [esp-rs Team](https://github.com/esp-rs) - esp-hal development
- [Espressif](https://www.espressif.com/) - ESP32-C6 hardware and tooling
- [Rust Embedded](https://github.com/rust-embedded) - embedded-hal standards
- Claude Code community - Agentic development methodology

---

**This is embedded development, reimagined for the AI era.**

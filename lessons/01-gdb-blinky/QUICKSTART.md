# Lesson 01: Quick Start Guide

**Goal:** Make an LED blink using ONLY GDB commands (no firmware LED code)

**Time:** 15-30 minutes

---

## Prerequisites

- ESP32-C6 DevKit (with LED on GPIO12)
- USB cable
- Rust toolchain installed
- `riscv32-esp-elf-gdb` (for GDB debugging)
- `probe-rs` or `openocd` (debug server)

---

## 5-Minute Quick Start

### 1. Build and Flash (2 min)

```bash
cd lessons/01-gdb-blinky
cargo build --release
espflash flash --port /dev/cu.usbmodem* target/riscv32imac-unknown-none-elf/release/main
```

### 2. Start Debug Server (Terminal 1)

```bash
probe-rs attach --chip esp32c6 --protocol jtag target/riscv32imac-unknown-none-elf/release/main
```

### 3. Load Automated Script (Terminal 2)

```bash
riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main
(gdb) source gdb_scripts/blinky.gdb
(gdb) continue
```

**LED should blink every 500ms!**

---

## Learning Paths

### Path A: Automated (Fastest)
- Just run `blinky.gdb`
- See it work
- Read code to understand

### Path B: Interactive (Recommended)
- Use `manual_control.gdb`
- Step-by-step discovery
- Learn by doing

### Path C: Guided with Claude Code (Best)
- Run `/gdb-blinky` command
- Claude guides you through discovery
- Learn concepts with AI assistance

---

## Troubleshooting

**LED doesn't blink?**
- Verify LED is connected to GPIO12
- Check debug server is running
- Try manual toggle: `(gdb) toggle_led`

**Can't connect GDB?**
- Ensure debug server started first
- Check port 3333 is available
- Restart both debug server and GDB

**Compilation error?**
- Verify esp-hal has `unstable` feature
- Check `rustup target list | grep riscv32imac`

---

## What You're Learning

1. **Register Discovery** - Finding hardware addresses from PAC crates
2. **Memory-Mapped I/O** - Hardware is just memory addresses
3. **GDB Power** - Debugging tool as hardware controller
4. **Agentic Development** - AI-assisted learning and discovery

---

## Next Lesson

**Lesson 02: High-Speed UART with DMA**

Use the GDB skills from this lesson to develop a UART peripheral rapidly by watching registers in real-time.

---

**Need help?** See full README.md for detailed explanations.

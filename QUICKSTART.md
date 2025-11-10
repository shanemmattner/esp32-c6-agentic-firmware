# Quick Start Guide

Get your first ESP32-C6 firmware running in 5 minutes.

## Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add RISC-V target for ESP32-C6
rustup target add riscv32imac-unknown-none-elf

# Install probe-rs for debugging
cargo install probe-rs --locked

# Install espflash for flashing
cargo install espflash --locked

# Install esp-generate for project templates
cargo install esp-generate --locked
```

## Create Your First Project

```bash
# Generate a new ESP32-C6 project with probe-rs support
esp-generate --chip esp32c6 my-project -o probe-rs

cd my-project
```

## Build & Flash

```bash
# Build release firmware
cargo build --release

# Flash to ESP32-C6
cargo run --release
```

The binary will be compiled and automatically flashed to your connected ESP32-C6.

## Use Lesson 01 as Template

Or clone and use the working example:

```bash
cd esp32-c6-agentic-firmware/lessons/01-blinky
cargo build --release
cargo run --release
```

## Monitor Serial Output

```bash
# View real-time logs
espflash monitor /dev/cu.usbserial-10

# Or use Python monitor script
python3 scripts/monitor.py --port /dev/cu.usbserial-10 --baud 115200
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `riscv32imac-unknown-none-elf not found` | Run `rustup target add riscv32imac-unknown-none-elf` |
| `espflash: command not found` | Run `cargo install espflash --locked` |
| `No port found` | Check USB cable, run `ls /dev/cu.*` to find port |
| `Permission denied on port` | May need elevated privileges or different terminal |

## Next Steps

- **Lesson 01**: [GPIO Output & Input](./lessons/01-blinky/) ✅
- **Lesson 02**: GPIO input and interrupts
- **Lesson 03**: Async/await with Embassy
- Full roadmap: [docs/LESSON_PLAN.md](./docs/LESSON_PLAN.md)

---

For detailed documentation, see [README.md](./README.md)

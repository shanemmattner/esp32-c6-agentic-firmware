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
cd esp32-c6-agentic-firmware/lessons/01-button-neopixel
cargo build --release
cargo run --release
```

## Monitor Serial Output

```bash
# View real-time logs via USB CDC (built into espflash)
espflash monitor

# Or for UART debugging (lessons 06-08), use the Python helper script
python3 .claude/templates/read_uart.py /dev/cu.usbserial-FT58PFX4 5
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `riscv32imac-unknown-none-elf not found` | Run `rustup target add riscv32imac-unknown-none-elf` |
| `espflash: command not found` | Run `cargo install espflash --locked` |
| `No port found` | Check USB cable, run `ls /dev/cu.*` to find port |
| `Permission denied on port` | May need elevated privileges or different terminal |

## Next Steps

- **Lesson 01**: [Button + NeoPixel](./lessons/01-button-neopixel/) ✅
- **Lesson 02**: [Task Scheduler](./lessons/02-task-scheduler/)
- **Lesson 03**: [I2C MPU9250 Sensor](./lessons/03-mpu9250/)
- **Lesson 07**: [GDB Debugging](./lessons/07-gdb-debugging/) ✅
- **Lesson 08**: [UART + GDB Tandem](./lessons/08-uart-gdb-tandem/) ✅
- Full roadmap: [docs/LESSON_PLAN.md](./docs/LESSON_PLAN.md)

---

For detailed documentation, see [README.md](./README.md)

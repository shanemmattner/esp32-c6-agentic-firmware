# ESP32-C6 Agentic Firmware Development

![ESP32-C6](https://img.shields.io/badge/ESP32--C6-Rust-orange)
![esp-hal](https://img.shields.io/badge/esp--hal-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## 🚀 Overview

Modern embedded Rust development for ESP32-C6 using **esp-hal 1.0.0** (pure Rust, bare-metal HAL) with practical, lesson-based tutorials.

**Why esp-hal 1.0.0?**
- ✨ **Pure Rust** - No C dependencies, no ESP-IDF required
- 🎯 **Official Support** - Backed by Espressif
- 🔥 **Bare Metal** - Direct hardware access, smaller binaries
- ⚡ **Modern** - Implements embedded-hal 1.0 traits

## 📚 Lessons

Progressive tutorials from basic GPIO to advanced features:

- **[01-blinky](./lessons/01-blinky/)** ✅ - GPIO output & input with logging
- **02-button-input** - GPIO input and interrupts
- **03-state-machine** - Async state machines with Embassy
- **04-i2c-sensor** - I2C driver implementation
- **05-spi-display** - SPI with display driver
- **06-uart-shell** - UART communication and CLI

See [docs/LESSON_PLAN.md](./docs/LESSON_PLAN.md) for the full curriculum.

## 🛠️ Quick Start

### Prerequisites

```bash
# Install Rust and RISC-V target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add riscv32imac-unknown-none-elf

# Install tools
cargo install espflash esp-generate --locked
```

### Build & Flash

```bash
cd lessons/01-blinky
cargo build --release
cargo run --release  # Flash to ESP32-C6
```

See [QUICKSTART.md](./QUICKSTART.md) for detailed instructions.

## 📖 Documentation

- **[docs/LESSON_PLAN.md](./docs/LESSON_PLAN.md)** - Full curriculum overview
- **[docs/REMOTE_DEVELOPMENT.md](./docs/REMOTE_DEVELOPMENT.md)** - Remote build setup
- **[Official esp-hal Docs](https://docs.esp-rs.org/esp-hal/)** - HAL reference
- **[esp-hal Examples](https://github.com/esp-rs/esp-hal/tree/main/examples)** - Code examples

## 🔥 esp-hal 1.0.0 Highlights

**Core Features:**
- Stable API with backward compatibility guarantees
- embedded-hal 1.0 standard traits
- Embassy async/await support
- DMA support for all peripherals
- Type-safe GPIO validation

**Example:**
```rust
#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());
    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(500);
    }
}
```

No ESP-IDF. No C code. Pure Rust! 🦀

## 📂 Project Structure

```
lessons/
├── 01-blinky/          # Lesson 1: GPIO output & input
│   ├── src/bin/main.rs
│   ├── Cargo.toml
│   ├── .cargo/config.toml
│   └── README.md
├── 02-button/          # Lesson 2: GPIO input & interrupts
└── ...

scripts/
├── monitor.py          # Serial monitor tool
└── remote-build-flash.sh

docs/
├── LESSON_PLAN.md      # Full curriculum
├── REMOTE_DEVELOPMENT.md
└── ...
```

## 🤝 Contributing

Contributions welcome! Focus areas:
- New esp-hal 1.0.0 examples
- Lesson improvements
- Documentation
- Bug fixes

## 📄 License

MIT OR Apache-2.0

## 🙏 Acknowledgments

- **[esp-rs Team](https://github.com/esp-rs)** - esp-hal 1.0.0
- **[Espressif](https://www.espressif.com/)** - Official Rust support
- **[Rust Embedded](https://github.com/rust-embedded)** - embedded-hal standards

---

**Built with esp-hal 1.0.0 🦀 | Modern Embedded Rust Development**

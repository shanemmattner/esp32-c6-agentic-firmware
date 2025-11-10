# ESP32-C6 Agentic Firmware Development
### Exploring esp-hal 1.0.0 + Claude Code Workflows

![ESP32-C6](https://img.shields.io/badge/ESP32--C6-Rust-orange)
![esp-hal](https://img.shields.io/badge/esp--hal-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## 🚀 Mission

This repository demonstrates **modern embedded Rust development** using the **new esp-hal 1.0.0** (released October 2024) combined with **Claude Code agentic workflows**. This is the first officially supported, bare-metal Rust HAL for ESP32 chips backed by Espressif.

### Why esp-hal 1.0.0?

**esp-hal 1.0.0** is a game-changer for ESP32 Rust development:
- ✨ **Pure Rust** - No C dependencies, no ESP-IDF required
- 🎯 **Official Support** - Backed by Espressif with paid developer time
- 🔥 **Modern Patterns** - Implements embedded-hal 1.0 traits properly
- ⚡ **Bare Metal** - Direct hardware access, smaller binaries, faster execution
- 🆕 **Latest Features** - Built for Rust's newest embedded ecosystem

**vs. Old Approach (esp-idf-hal):**
| Feature | esp-hal 1.0.0 (NEW) | esp-idf-hal (OLD) |
|---------|---------------------|-------------------|
| Architecture | Pure Rust, bare-metal | C wrapper (ESP-IDF) |
| Official Support | ✅ Espressif-backed | Community-maintained |
| Dependencies | Minimal | Requires ESP-IDF install |
| Binary Size | Smaller | Larger |
| Learning Curve | Modern Rust patterns | ESP-IDF + Rust hybrid |
| Future | Active development | Legacy path |

## 🎯 Goals

1. **Explore esp-hal 1.0.0** - Document new features, patterns, and capabilities
2. **Claude Code Workflows** - Build agentic templates for firmware generation
3. **Tutorial Series** - Progressive lessons from blinky to complex systems
4. **YouTube Content** - Video series on LLM-driven embedded development

## 📚 Tutorial Structure

Each lesson demonstrates **esp-hal 1.0.0** features with comprehensive Claude Code integration:

### Foundations (esp-hal 1.0.0 Basics)
- **[✅ 01-blinky](./lessons/01-blinky/)** - GPIO output, `Delay`, logging patterns
- **02-button-input** - GPIO input, interrupts, `Io` peripheral
- **03-state-machine** - Async state machines with `embassy-executor`

### Peripherals (esp-hal 1.0.0 Drivers)
- **04-i2c-sensor** - I2C with embedded-hal 1.0 traits
- **05-spi-display** - SPI with DMA support
- **06-uart-shell** - UART communication, CLI interface

### Advanced (esp-hal 1.0.0 Features)
- **07-async-embassy** - Embassy async runtime integration
- **08-wifi-basics** - WiFi with esp-wifi 1.0 (bare-metal stack!)
- **09-dma-advanced** - Direct Memory Access patterns
- **10-low-power** - LP core and sleep modes

## 🤖 Claude Code Integration

### Slash Commands (Coming Soon)
```bash
/hal-driver <peripheral>    # Generate esp-hal 1.0.0 driver template
/add-logging <granularity>  # Add comprehensive logging
/create-test <component>    # Generate embedded test scaffold
/async-task <description>   # Create embassy async task
```

### Agentic Templates
- **Driver Templates** - I2C, SPI, UART with esp-hal 1.0.0 patterns
- **State Machines** - Embassy async state patterns
- **Error Handling** - Result types, panic handlers, logging
- **Test Scaffolds** - Unit, integration, hardware-in-loop

### Claude Code Workflows
1. **PRD-Driven** - Generate Product Requirements Documents
2. **Test-First** - Write tests, implement to pass
3. **Feedback Loops** - Use logging for agentic verification
4. **Template-Based** - Reusable patterns for common tasks

## 🛠️ Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add RISC-V target (ESP32-C6)
rustup target add riscv32imac-unknown-none-elf

# Install flashing tools
cargo install espflash
```

### Quick Start

```bash
git clone https://github.com/shanemmattner/esp32-c6-agentic-firmware.git
cd esp32-c6-agentic-firmware/lessons/01-blinky
cargo build --release
cargo run --release  # Flash to ESP32-C6
```

See [QUICKSTART.md](./QUICKSTART.md) for detailed instructions.

## 🔥 esp-hal 1.0.0 Highlights

### What's New in 1.0.0?

**Core Features:**
- ✅ **Stable API** - Breaking changes only in major versions
- ✅ **embedded-hal 1.0** - Standard traits for portability
- ✅ **Embassy Integration** - First-class async/await support
- ✅ **DMA Support** - Direct memory access for all peripherals
- ✅ **Type-Safe GPIO** - Compile-time pin validation
- ✅ **RMT Peripheral** - For addressable LEDs, IR, etc.

**Example: Modern GPIO Pattern**
```rust
#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(1000);
    }
}
```

**No ESP-IDF, no C code, pure Rust!** 🦀

## 📖 Documentation

- **[QUICKSTART.md](./QUICKSTART.md)** - Build and flash guide
- **[esp-hal 1.0.0 Docs](https://docs.esp-rs.org/esp-hal/)** - Official HAL docs
- **[Examples](./submodules/esp-hal/examples/)** - Reference implementations
- **[Rust ESP Book](https://docs.esp-rs.org/book/)** - Comprehensive guide

## 🎓 Learning Resources

### esp-hal 1.0.0 Specific
- [esp-hal 1.0.0 Release Announcement](https://developer.espressif.com/blog/2025/10/esp-hal-1/)
- [Migration from esp-idf-hal](https://docs.esp-rs.org/book/writing-your-own-application/nostd.html)
- [embedded-hal 1.0 Traits](https://docs.rs/embedded-hal/1.0.0/)

### Claude Code + Embedded
- Agentic firmware development patterns
- Template-based code generation
- Logging-driven verification
- Test-driven embedded development

## 🧪 Development Workflow

### 1. Plan with PRD
Use Claude Code to generate Product Requirements Documents:
```
What: Add BME280 temperature sensor driver
Why: Environmental monitoring
How: I2C with embedded-hal 1.0 traits
Success: Reads temp every 1s, logs values
```

### 2. Test-First Development
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_bme280_read() {
        // Write test first
        assert_eq!(sensor.read_temperature(), Ok(25.0));
    }
}
```

### 3. Implement with esp-hal 1.0.0
```rust
let i2c = I2c::new(peripherals.I2C0, sda, scl, Config::default());
let mut sensor = Bme280::new(i2c);
let temp = sensor.read_temperature()?;
```

### 4. Verify with Logging
```rust
info!("BME280 initialized successfully");
info!("Temperature: {:.2}°C", temp);
```

## 🤝 Contributing

This is a learning and demonstration repository. Contributions welcome!

Focus areas:
- New esp-hal 1.0.0 examples
- Claude Code slash commands
- Template improvements
- Documentation

## 📄 License

MIT OR Apache-2.0

## 🙏 Acknowledgments

- **[esp-rs Team](https://github.com/esp-rs)** - esp-hal 1.0.0 development
- **[Espressif](https://www.espressif.com/)** - Official support for Rust
- **[Anthropic](https://www.anthropic.com/)** - Claude Code AI assistant
- **[Rust Embedded Working Group](https://github.com/rust-embedded)** - embedded-hal 1.0

## 📺 YouTube Series (Planned)

Topics:
1. **Why esp-hal 1.0.0?** - Modern vs legacy approaches
2. **Blinky Deep Dive** - Understanding the new patterns
3. **Async with Embassy** - Modern concurrency
4. **WiFi Bare-Metal** - esp-wifi without ESP-IDF
5. **Claude Code Workflows** - Agentic firmware development

---

**Built with esp-hal 1.0.0 🦀 | Powered by Claude Code 🤖**

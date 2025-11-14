# Contributing to ESP32-C6 Agentic Firmware

Thank you for your interest in contributing! This project aims to provide high-quality, educational embedded Rust firmware examples for the ESP32-C6 using esp-hal 1.0.0.

## 🚀 Quick Start for Contributors

### Prerequisites

1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add riscv32imac-unknown-none-elf
   ```

2. **ESP32 Tools**
   ```bash
   cargo install espflash esp-generate --locked
   ```

3. **Hardware**
   - ESP32-C6-DevKitC-1 development board
   - USB-C cable
   - (Optional) FTDI UART adapter for debugging lessons 06-08

### Building Lessons

```bash
cd lessons/01-button-neopixel
cargo build --release
cargo run --release  # Flash to hardware
```

---

## 📋 Contribution Areas

We welcome contributions in these areas:

### 1. New Lessons
- Additional peripheral examples (SPI, ADC, PWM, timers)
- Advanced topics (DMA, sleep modes, watchdog)
- Communication protocols (WiFi, BLE)
- Real-world project examples

### 2. Documentation
- Improving existing lesson READMEs
- Adding diagrams and schematics
- Troubleshooting guides
- Video tutorials or blog posts

### 3. Bug Fixes
- Build errors
- Hardware compatibility issues
- Documentation errors
- Code quality improvements

### 4. Testing
- Hardware validation on different ESP32-C6 boards
- Cross-platform testing (Windows, Linux, macOS)
- Edge case testing

---

## 🎯 Code Style Guidelines

### General Principles

- **Keep it simple**: Educational code should be clear, not clever
- **Follow CLAUDE.md**: Adhere to project conventions in `CLAUDE.md`
- **Code length**: Aim for 100-150 lines per simple lesson, up to 300-400 for complex topics
- **Comment wisely**: Explain "why", not "what"
- **Hardware first**: Always test on real hardware before submitting

### Rust Style

- Follow standard Rust formatting: `cargo fmt`
- Run Clippy: `cargo clippy --release`
- Use `esp-hal 1.0.0` patterns (see existing lessons for examples)
- Prefer explicit types over `auto`/inference where it aids clarity
- Use `#[main]` entry point macro (not `#[entry]`)

### esp-hal 1.0.0 Patterns

```rust
// ✅ Good: esp-hal 1.0.0 style
let peripherals = esp_hal::init(esp_hal::Config::default());
let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));

// ❌ Bad: old pre-1.0 style
let peripherals = Peripherals::take();
let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
let button = io.pins.gpio9.into_pull_up_input();
```

---

## 📝 Lesson Structure

When creating a new lesson, follow this structure:

```
lessons/XX-lesson-name/
├── src/
│   ├── bin/
│   │   └── main.rs          # Main firmware code
│   └── lib.rs               # (Optional) Library code
├── .cargo/
│   └── config.toml          # Build and flash configuration
├── Cargo.toml               # Dependencies and binary config
├── rust-toolchain.toml      # Rust version (nightly)
├── build.rs                 # Build script (if needed)
├── README.md                # Lesson documentation
└── TEST.md                  # (Optional) Hardware test procedures
```

### README.md Template

```markdown
# Lesson XX: Title

Brief description of what this lesson teaches.

## What You'll Learn

- Bullet point 1
- Bullet point 2

## Hardware Requirements

- ESP32-C6-DevKitC-1
- Additional components (if any)

## Wiring

(Include GPIO pin mappings and wiring diagram if needed)

## Building and Flashing

\```bash
cargo build --release
cargo run --release
\```

## Expected Behavior

Describe what should happen when firmware runs.

## Troubleshooting

Common issues and solutions.

## Key Takeaways

- Learning point 1
- Learning point 2
```

---

## 🧪 Testing Requirements

Before submitting a PR:

1. **Build successfully**
   ```bash
   cargo build --release
   cargo clippy --release
   cargo fmt --check
   ```

2. **Test on hardware**
   - Flash to ESP32-C6
   - Verify expected behavior
   - Test edge cases
   - Document any hardware-specific issues

3. **Update documentation**
   - Ensure README is accurate
   - Update main `README.md` if adding new lesson
   - Add troubleshooting section for common issues

---

## 🔄 Pull Request Process

### 1. Fork and Clone

```bash
git clone https://github.com/YOUR-USERNAME/esp32-c6-agentic-firmware.git
cd esp32-c6-agentic-firmware
```

### 2. Create a Branch

```bash
git checkout -b feature/lesson-XX-new-peripheral
# or
git checkout -b fix/lesson-03-i2c-timing
```

### 3. Make Changes

- Follow code style guidelines
- Test on hardware
- Update documentation

### 4. Commit

```bash
git add .
git commit -m "feat(lesson-XX): Add SPI peripheral example

- Implement SPI master mode
- Add Nokia 5110 LCD driver
- Include wiring diagram
- Test on ESP32-C6-DevKitC-1"
```

**Commit Message Format:**
- `feat(scope): Description` - New feature
- `fix(scope): Description` - Bug fix
- `docs(scope): Description` - Documentation only
- `refactor(scope): Description` - Code refactoring
- `test(scope): Description` - Adding tests

### 5. Push and Create PR

```bash
git push origin feature/lesson-XX-new-peripheral
```

Then create a Pull Request on GitHub with:
- Clear title and description
- Hardware tested confirmation
- Screenshots or output logs (if applicable)
- Any breaking changes noted

---

## 🐛 Reporting Issues

When reporting bugs, please include:

1. **Lesson number and name**
2. **Hardware:** ESP32-C6 board variant
3. **Environment:** OS, Rust version, esp-hal version
4. **Expected behavior**
5. **Actual behavior**
6. **Steps to reproduce**
7. **Build output** (if build error)
8. **Serial output** (if runtime error)

---

## 💡 Feature Requests

We're always looking for new lesson ideas! When suggesting a new lesson:

1. **Check existing lessons** to avoid duplication
2. **Describe the peripheral/feature** clearly
3. **Explain educational value** - what will learners gain?
4. **List hardware requirements**
5. **Note complexity level** (beginner, intermediate, advanced)

---

## 📜 License

By contributing, you agree that your contributions will be licensed under both:
- MIT License
- Apache License 2.0

The same dual-license as the project.

---

## 🙏 Code of Conduct

- **Be respectful** and inclusive
- **Provide constructive feedback**
- **Help newcomers** - we were all beginners once
- **Focus on education** - clarity over cleverness
- **Test on hardware** - embedded code must work in reality

---

## 🔗 Resources

- **esp-hal Documentation:** https://docs.esp-rs.org/esp-hal/
- **esp-hal Examples:** https://github.com/esp-rs/esp-hal/tree/main/examples
- **ESP32-C6 Datasheet:** https://www.espressif.com/sites/default/files/documentation/esp32-c6_datasheet_en.pdf
- **Rust Embedded Book:** https://doc.rust-lang.org/embedded-book/

---

## ❓ Questions?

- Open an issue for general questions
- Tag with `question` label
- Check existing issues first

Thank you for contributing! 🦀🚀

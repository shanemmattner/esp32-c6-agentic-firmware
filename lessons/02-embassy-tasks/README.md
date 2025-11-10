# Lesson 01: Button + NeoPixel Control

Welcome to the first lesson in the ESP32-C6 firmware series! In this lesson, you'll learn how to control an onboard NeoPixel (WS2812 RGB LED) using a button press.

## What You'll Learn

- Reading digital input (button)
- Controlling WS2812/NeoPixel LEDs via RMT peripheral
- Edge detection for clean button handling
- Simple debouncing techniques
- Basic event-driven programming

## Hardware Requirements

- **ESP32-C6-DevKitC-1** development board
- USB-C cable for programming and power

The dev board has:
- **Onboard NeoPixel (WS2812)** on GPIO8
- **BOOT button** on GPIO9

No external components needed!

---

## Prerequisites

### Software Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add RISC-V target for ESP32-C6
rustup target add riscv32imac-unknown-none-elf

# Install esp-generate (project template generator)
cargo install esp-generate --locked

# Install espflash (flashing tool)
cargo install espflash --locked
```

## Creating Your First ESP32-C6 Project

### Step 1: Generate Project with esp-generate

```bash
# Generate a new ESP32-C6 project
esp-generate --chip esp32c6 lesson-01-button-neopixel

cd lesson-01-button-neopixel
```

This creates a properly configured project with:
- `.cargo/config.toml` with espflash runner configuration
- `build.rs` for linker configuration
- `rust-toolchain.toml` with nightly Rust (required for RMT peripheral in esp-hal 1.0.0)
- `Cargo.toml` with project metadata and binary configuration
- Skeleton code in `src/bin/main.rs` and `src/lib.rs`

**Why nightly?** The RMT (Remote Control Transceiver) peripheral used for NeoPixel control requires nightly Rust features in esp-hal 1.0.0.

### Project Structure Explained

```
lesson-01-button-neopixel/
├── src/
│   ├── bin/
│   │   └── main.rs          ← Main firmware code
│   └── lib.rs               ← Library code (empty by default)
├── .cargo/
│   └── config.toml          ← Build configuration
├── build.rs                 ← Build script (linker config)
├── Cargo.toml               ← Project manifest
└── rust-toolchain.toml      ← Rust version & targets
```

**Why `src/bin/main.rs`?** This is the default structure from `esp-generate`. The `[[bin]]` section in `Cargo.toml` explicitly points to this binary.

### Step 2: Update Cargo.toml

The generated `Cargo.toml` already has the right structure. We just need to add NeoPixel dependencies to the `[dependencies]` section:

```toml
[dependencies]
# Hardware abstraction layer
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }

# Panic handler
esp-backtrace = { version = "0.15", features = ["esp32c6", "panic-handler", "println"] }

# Serial printing and logging
esp-println = { version = "0.13", features = ["esp32c6", "log"] }
log = "0.4"

# Bootloader app descriptor
esp-bootloader-esp-idf = { version = "0.4.0", features = ["esp32c6"] }

# Critical sections
critical-section = "1.2.0"

# SmartLED driver for NeoPixel
esp-hal-smartled2 = { version = "0.26", features = ["esp32c6"] }
smart-leds = "0.4"
```

**Key dependencies added**:
- `esp-hal-smartled2` - WS2812/NeoPixel driver using RMT
- `smart-leds` - Common LED traits and RGB types

### Step 3: Write the Code

Replace the skeleton code in `src/bin/main.rs` with the button + NeoPixel firmware code

---

## Code Walkthrough

Let's break down `main.rs` section by section.

### 1. Setup and Imports

```rust
#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Pull},
    main,
    rmt::Rmt,
    time::Rate,
    Blocking,
};
use esp_hal_smartled::{buffer_size, color_order, SmartLedsAdapter, Ws2812Timing};
use log::info;
use smart_leds::{SmartLedsWrite, RGB8};
```

**What's happening:**
- `#![no_std]` - We're not using Rust's standard library (embedded systems don't have malloc, threads, etc.)
- `#![no_main]` - We define our own entry point (not `fn main()`)
- Import hardware peripherals: GPIO, RMT, Delay
- Import NeoPixel driver and RGB color types

### 2. Panic Handler

```rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n\n*** PANIC: {} ***\n", info);
    loop {}
}
```

**Why?** Without `std`, we need to define what happens when code panics. This prints the error and hangs.

### 3. Main Function

```rust
#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);
```

**What's happening:**
- `#[main]` - ESP32 entry point macro
- Initialize serial logging so we can see `info!()` messages
- Return type `-> !` means "never returns" (infinite loop)

### 4. Initialize Hardware

```rust
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
```

**What's happening:**
- Take ownership of all ESP32 peripherals
- Create a delay helper for `delay_millis()`

### 5. Configure Button Input

```rust
    // Configure button (GPIO9) as input with pull-up
    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));
    info!("Button configured on GPIO9");
```

**What's happening:**
- GPIO9 is the BOOT button on the dev board
- Configure as input with internal pull-up resistor
- Button is **active LOW**: pressed = LOW (0), released = HIGH (1)

**Why pull-up?** Without it, the pin would float (undefined state). Pull-up ensures it reads HIGH when not pressed.

### 6. Initialize NeoPixel via RMT

```rust
    // Initialize RMT for NeoPixel control
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("Failed to init RMT");
    let mut led = SmartLedsAdapter::<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>::new_with_memsize(
        rmt.channel0,
        peripherals.GPIO8,
        2,
    ).expect("Failed to create SmartLedsAdapter");
```

**What's happening:**
- RMT (Remote Control Transceiver) peripheral generates precise timing for WS2812
- Create adapter for 1 LED (buffer_size(1))
- Use RGB color order and WS2812 timing (800kHz)
- GPIO8 is the NeoPixel data pin

**Why RMT?** WS2812 LEDs need very precise microsecond-level timing. RMT hardware handles this automatically.

### 7. Main Loop with Edge Detection

```rust
    let mut led_on = false;
    let mut button_was_pressed = false;

    loop {
        let button_pressed = button.is_low();

        // Detect button press (rising edge)
        if button_pressed && !button_was_pressed {
            // Toggle LED state
            led_on = !led_on;

            if led_on {
                // Turn on (blue, dimmed)
                led.write([RGB8::new(0, 0, 30)].into_iter()).unwrap();
                info!("LED ON (blue)");
            } else {
                // Turn off
                led.write([RGB8::new(0, 0, 0)].into_iter()).unwrap();
                info!("LED OFF");
            }

            // Simple debounce delay
            delay.delay_millis(200);
        }

        button_was_pressed = button_pressed;

        // Small delay to avoid busy-waiting
        delay.delay_millis(10);
    }
```

**What's happening:**

1. **Edge Detection**: We only trigger on button press, not while held
   - `button_pressed && !button_was_pressed` - detects transition from released to pressed
   - Prevents rapid toggling while button is held down

2. **Toggle State**: Flip `led_on` between true/false

3. **Update NeoPixel**:
   - ON: `RGB8::new(0, 0, 30)` - Blue at low brightness (30/255)
   - OFF: `RGB8::new(0, 0, 0)` - All LEDs off (black)

4. **Debouncing**: 200ms delay after detection prevents mechanical bounce

5. **Polling Loop**: Check button state every 10ms

---

## How It Works

### WS2812 NeoPixel Protocol

NeoPixel LEDs use a one-wire protocol with precise timing:
- Each color (R, G, B) is 8 bits = 0-255 brightness
- Data is sent as timed HIGH/LOW pulses:
  - **0 bit**: 0.4μs HIGH, 0.85μs LOW
  - **1 bit**: 0.8μs HIGH, 0.45μs LOW

The RMT peripheral handles this timing automatically!

### Button Debouncing

Mechanical buttons "bounce" - they make/break contact multiple times when pressed. Our simple approach:
1. Detect button press
2. Wait 200ms before checking again
3. This ignores bounce noise

### RGB Color Values

`RGB8::new(red, green, blue)` takes 0-255 for each channel:
- `RGB8::new(255, 0, 0)` - Full red
- `RGB8::new(0, 255, 0)` - Full green
- `RGB8::new(0, 0, 255)` - Full blue
- `RGB8::new(0, 0, 30)` - Dim blue (our choice)
- `RGB8::new(255, 255, 255)` - White (all colors max)
- `RGB8::new(0, 0, 0)` - Off (black)

---

## Building and Flashing

### Build the project

```bash
cargo build --release
```

This compiles for RISC-V and creates an optimized binary.

### Flash to ESP32-C6

```bash
cargo run --release
```

Or manually:

```bash
espflash flash --chip esp32c6 target/riscv32imac-unknown-none-elf/release/lesson-01-button-neopixel
```

### Expected Behavior

1. Flash completes
2. Device boots
3. Press BOOT button → Blue LED turns ON
4. Press BOOT button again → LED turns OFF
5. Serial output shows "LED ON (blue)" and "LED OFF"

---

## Troubleshooting

**LED doesn't light up:**
- Check GPIO8 is correct for your board
- Verify NeoPixel is soldered and working
- Try increasing brightness: `RGB8::new(0, 0, 255)` (full blue)

**Button doesn't respond:**
- GPIO9 should be BOOT button
- Try pressing firmly
- Check serial output - you should see log messages

**Build fails:**
- Ensure `riscv32imac-unknown-none-elf` target installed: `rustup target add riscv32imac-unknown-none-elf`
- Use nightly toolchain: `rustup default nightly`

---

## Next Steps

**Lesson 02** will introduce:
- Task scheduler for running multiple periodic tasks
- Separating concerns into modules (`lib.rs`, `scheduler.rs`, `tasks.rs`)
- Better code organization for larger projects

---

## Key Takeaways

✅ **no_std** embedded programming has no allocator, no threads, no filesystem
✅ **RMT peripheral** handles precise timing for WS2812 protocol
✅ **Edge detection** prevents button bounce and held-button spam
✅ **Pull-up resistors** ensure stable digital input readings
✅ **RGB color mixing** lets you create any color (16.7 million combinations!)

Great job completing Lesson 01! 🎉

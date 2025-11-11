//! # Lesson 06: UART Terminal
//!
//! Interactive serial terminal for debugging and controlling firmware.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - MPU9250 IMU module (I2C)
//! - WS2812 NeoPixel LED
//! - Push button (active LOW)
//! - USB-to-serial adapter on GPIO15 (TX) and GPIO23 (RX)
//!
//! **Pins:**
//! - GPIO9: Button input (active LOW with pull-up)
//! - GPIO8: NeoPixel data (RMT)
//! - GPIO2: I2C SDA (MPU9250)
//! - GPIO11: I2C SCL (MPU9250)
//! - GPIO15: UART TX (transmit to PC)
//! - GPIO23: UART RX (receive from PC)
//!
//! **What You'll Learn:**
//! - UART communication for terminal I/O
//! - Command parsing and dispatching
//! - Integrating multiple peripherals (Button, LED, IMU, UART)
//! - Streaming sensor data over serial
//!
//! **Interaction:**
//! - Connect via serial terminal (115200 baud)
//! - Type 'help' to see available commands
//! - Commands: imu_read, imu_stream <hz>, led_on, led_off, led_color, etc.

#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Pull},
    i2c::master::{Config as I2cConfig, I2c},
    main,
    rmt::Rmt,
    time::Rate,
    uart::{Config as UartConfig, Uart},
    Blocking,
};
use esp_hal_smartled::{buffer_size, color_order, SmartLedsAdapter, Ws2812Timing};
use log::info;
use smart_leds::{SmartLedsWrite, RGB8};

use lesson_06_uart_terminal::{
    button, cli, mpu9250, uart, uwriteln,
    BUTTON_GPIO, I2C_SCL_GPIO, I2C_SDA_GPIO, NEOPIXEL_GPIO, RMT_CLOCK_MHZ,
    UART_RX_GPIO, UART_TX_GPIO,
};

// ============================================================================
// PANIC HANDLER
// ============================================================================

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n\n*** PANIC: {} ***\n", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// [SECTION 1/2: COPY-PASTE - Peripheral initialization]
// ============================================================================
// Keep this section, copy from starter code

// Global state for LED control
static LED_ON: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
static LED_COLOR: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0x00_00_1E); // Blue, dimmed

// IMU streaming state
static IMU_STREAM_ENABLED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
static IMU_STREAM_RATE_HZ: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);

/// Get LED color from atomic state
fn get_led_color() -> (u8, u8, u8) {
    let color = LED_COLOR.load(core::sync::atomic::Ordering::Relaxed);
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    (r, g, b)
}

/// Set LED color
fn set_led_color(r: u8, g: u8, b: u8) {
    let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    LED_COLOR.store(color, core::sync::atomic::Ordering::Relaxed);
}

/// Check if LED is on
fn is_led_on() -> bool {
    LED_ON.load(core::sync::atomic::Ordering::Relaxed)
}

/// Set LED on/off state
fn set_led_on(on: bool) {
    LED_ON.store(on, core::sync::atomic::Ordering::Relaxed);
}

// [END SECTION 1/2]

// ============================================================================
// [SECTION 2/2: USER TYPES - Main application and command dispatcher]
// ============================================================================
// DELETE THIS COMMENT and type from here until [END SECTION 2/2]

#[main]
fn main() -> ! {
    // Initialize logging
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 06: UART Terminal\n");

    // Initialize hardware
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // ========================================================================
    // Initialize I2C for MPU9250
    // ========================================================================

    let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .expect("I2C init failed")
        .with_sda(peripherals.GPIO2)
        .with_scl(peripherals.GPIO11);

    info!("✓ I2C initialized (GPIO{}=SDA, GPIO{}=SCL)", I2C_SDA_GPIO, I2C_SCL_GPIO);

    // Wake up MPU9250
    if mpu9250::wake_sensor(&mut i2c).is_ok() {
        info!("✓ MPU9250 awake");
    }

    delay.delay_millis(100);

    // ========================================================================
    // Initialize Button (GPIO9, active LOW with pull-up)
    // ========================================================================

    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));
    info!("✓ Button configured (GPIO{}, active LOW)", BUTTON_GPIO);

    // ========================================================================
    // Initialize NeoPixel (GPIO8, RMT)
    // ========================================================================

    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(RMT_CLOCK_MHZ))
        .expect("Failed to init RMT");
    let mut led = SmartLedsAdapter::<
        { buffer_size(1) },
        Blocking,
        color_order::Rgb,
        Ws2812Timing,
    >::new_with_memsize(rmt.channel0, peripherals.GPIO8, 2)
    .expect("Failed to create SmartLedsAdapter");

    info!("✓ NeoPixel initialized (GPIO{})", NEOPIXEL_GPIO);

    // ========================================================================
    // Initialize UART (GPIO15=TX, GPIO23=RX, 115200 baud)
    // ========================================================================

    let mut uart = Uart::new(peripherals.UART1, UartConfig::default())
        .expect("Failed to init UART")
        .with_tx(peripherals.GPIO15)
        .with_rx(peripherals.GPIO23);

    info!("✓ UART initialized (GPIO{}=TX, GPIO{}=RX, 115200 baud)", UART_TX_GPIO, UART_RX_GPIO);

    let mut terminal = uart::Terminal::new();

    // ========================================================================
    // Startup Message
    // ========================================================================

    info!("✓ All peripherals initialized\n");
    info!("🔄 Starting interactive terminal...\n");

    let _ = terminal.write_str(&mut uart, "\r\n");
    let _ = terminal.write_str(&mut uart, "==============================================\r\n");
    let _ = terminal.write_str(&mut uart, "  ESP32-C6 Interactive Terminal\r\n");
    let _ = terminal.write_str(&mut uart, "  Lesson 06: UART Terminal\r\n");
    let _ = terminal.write_str(&mut uart, "==============================================\r\n");
    let _ = terminal.write_str(&mut uart, "\r\n");
    let _ = terminal.write_str(&mut uart, "Type 'help' for available commands.\r\n");
    let _ = terminal.write_str(&mut uart, "\r\n");
    terminal.prompt(&mut uart);

    // ========================================================================
    // Main Loop
    // ========================================================================

    let mut current_time_ms: u64 = 0;
    let mut button_next_run_ms: u64 = 0;
    let mut led_next_run_ms: u64 = 0;
    let mut imu_next_run_ms: u64 = 0;

    const TICK_MS: u64 = 10;
    const BUTTON_PERIOD_MS: u64 = 10;
    const LED_PERIOD_MS: u64 = 50;

    loop {
        // Tick
        delay.delay_millis(TICK_MS as u32);
        current_time_ms += TICK_MS;

        // Check for UART commands
        if let Some(line) = terminal.read_line(&mut uart) {
            // Parse command
            if let Ok(line_str) = uart::bytes_to_str(&line) {
                if let Some(cmd) = cli::parse_command(line_str) {
                    // Dispatch command
                    handle_command(&mut terminal, &mut uart, &mut i2c, cmd);
                }
            }
            terminal.prompt(&mut uart);
        }

        // Button task
        if current_time_ms >= button_next_run_ms {
            if button::button_task(&button) {
                // Toggle LED
                let new_state = !is_led_on();
                set_led_on(new_state);
                let status_msg = if new_state { "ON" } else { "OFF" };
                let _ = terminal.write_str(&mut uart, "🔘 Button: LED ");
                let _ = terminal.write_str(&mut uart, status_msg);
                let _ = terminal.write_str(&mut uart, "\r\n");
            }
            button_next_run_ms = current_time_ms + BUTTON_PERIOD_MS;
        }

        // LED task
        if current_time_ms >= led_next_run_ms {
            if is_led_on() {
                let (r, g, b) = get_led_color();
                let _ = led.write([RGB8::new(r, g, b)].into_iter());
            } else {
                let _ = led.write([RGB8::new(0, 0, 0)].into_iter());
            }
            led_next_run_ms = current_time_ms + LED_PERIOD_MS;
        }

        // IMU streaming task
        if IMU_STREAM_ENABLED.load(core::sync::atomic::Ordering::Relaxed) {
            if current_time_ms >= imu_next_run_ms {
                let rate_hz = IMU_STREAM_RATE_HZ.load(core::sync::atomic::Ordering::Relaxed);
                if rate_hz > 0 {
                    if let Ok(accel) = mpu9250::read_accel(&mut i2c) {
                        let _ = uwriteln!(&mut uart, "📊 {},{},{}", accel.x, accel.y, accel.z);
                    }
                    let period_ms = 1000 / rate_hz as u64;
                    imu_next_run_ms = current_time_ms + period_ms;
                }
            }
        }
    }
}

/// Handle a CLI command
fn handle_command(
    terminal: &mut uart::Terminal,
    uart: &mut Uart<Blocking>,
    i2c: &mut I2c<Blocking>,
    cmd: cli::Command,
) {
    use cli::CliCommand;

    match cli::identify_command(cmd.name) {
        CliCommand::Help => {
            let _ = terminal.write_str(uart, cli::HELP_TEXT);
        }

        CliCommand::Status => {
            let _ = uwriteln!(uart, "System Status:");
            let _ = uwriteln!(uart, "  LED: {}", if is_led_on() { "ON" } else { "OFF" });
            let (r, g, b) = get_led_color();
            let _ = uwriteln!(uart, "  LED Color: R={} G={} B={}", r, g, b);
            let streaming = IMU_STREAM_ENABLED.load(core::sync::atomic::Ordering::Relaxed);
            let _ = uwriteln!(uart, "  IMU Streaming: {}", if streaming { "ENABLED" } else { "DISABLED" });
            if streaming {
                let rate = IMU_STREAM_RATE_HZ.load(core::sync::atomic::Ordering::Relaxed);
                let _ = uwriteln!(uart, "  IMU Rate: {} Hz", rate);
            }
        }

        CliCommand::Reset => {
            let _ = terminal.write_str(uart, "⚠ Reset not implemented (use hardware reset button)\r\n");
        }

        CliCommand::ImuRead => {
            match mpu9250::read_accel(i2c) {
                Ok(accel) => {
                    let _ = uwriteln!(uart, "📊 Accel: x={}, y={}, z={}", accel.x, accel.y, accel.z);
                }
                Err(_) => {
                    let _ = terminal.write_str(uart, "❌ Failed to read IMU\r\n");
                }
            }
        }

        CliCommand::ImuStream => {
            if cmd.args.len() == 1 {
                if let Ok(rate) = cmd.args[0].parse::<u8>() {
                    if rate == 10 || rate == 50 || rate == 100 {
                        IMU_STREAM_RATE_HZ.store(rate, core::sync::atomic::Ordering::Relaxed);
                        IMU_STREAM_ENABLED.store(true, core::sync::atomic::Ordering::Relaxed);
                        let _ = uwriteln!(uart, "✓ IMU streaming at {} Hz", rate);
                    } else {
                        let _ = terminal.write_str(uart, "❌ Invalid rate. Use 10, 50, or 100 Hz\r\n");
                    }
                } else {
                    let _ = terminal.write_str(uart, "❌ Invalid rate\r\n");
                }
            } else {
                let _ = terminal.write_str(uart, "Usage: imu_stream <hz>\r\n");
            }
        }

        CliCommand::ImuStreamStop => {
            IMU_STREAM_ENABLED.store(false, core::sync::atomic::Ordering::Relaxed);
            let _ = terminal.write_str(uart, "✓ IMU streaming stopped\r\n");
        }

        CliCommand::ImuRange => {
            let _ = terminal.write_str(uart, "⚠ IMU range configuration not implemented\r\n");
        }

        CliCommand::ImuFilter => {
            let _ = terminal.write_str(uart, "⚠ IMU filter configuration not implemented\r\n");
        }

        CliCommand::ImuStatus => {
            match mpu9250::read_who_am_i(i2c) {
                Ok(id) => {
                    let _ = uwriteln!(uart, "IMU Status:");
                    let _ = uwriteln!(uart, "  WHO_AM_I: 0x{:02x}", id);
                    let _ = uwriteln!(uart, "  Expected: 0x71");
                    let _ = uwriteln!(uart, "  Status: {}", if id == 0x71 { "✓ OK" } else { "❌ ERROR" });
                }
                Err(_) => {
                    let _ = terminal.write_str(uart, "❌ Failed to read IMU\r\n");
                }
            }
        }

        CliCommand::LedOn => {
            set_led_on(true);
            let _ = terminal.write_str(uart, "✓ LED ON\r\n");
        }

        CliCommand::LedOff => {
            set_led_on(false);
            let _ = terminal.write_str(uart, "✓ LED OFF\r\n");
        }

        CliCommand::LedColor => {
            if cmd.args.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    cmd.args[0].parse::<u8>(),
                    cmd.args[1].parse::<u8>(),
                    cmd.args[2].parse::<u8>(),
                ) {
                    set_led_color(r, g, b);
                    let _ = uwriteln!(uart, "✓ LED color set to R={} G={} B={}", r, g, b);
                } else {
                    let _ = terminal.write_str(uart, "❌ Invalid color values\r\n");
                }
            } else {
                let _ = terminal.write_str(uart, "Usage: led_color <r> <g> <b>\r\n");
            }
        }

        CliCommand::Unknown => {
            let _ = uwriteln!(uart, "❌ Unknown command: '{}'", cmd.name);
            let _ = terminal.write_str(uart, "Type 'help' for available commands.\r\n");
        }
    }
}

// [END SECTION 2/2]

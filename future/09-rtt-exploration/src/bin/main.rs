//! Lesson 09: RTT Variable Streaming Infrastructure
//!
//! Stream arbitrary hardware variables via RTT for data-driven debugging.
//! Demonstrates the maximum observability philosophy: log 50-500+ variables
//! @ 100 Hz to catch bugs instantly via pattern detection.

#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use esp_hal::{delay::Delay, main};
use core::sync::atomic::{AtomicU32, Ordering};

// Import telemetry infrastructure
use lesson_08_defmt_rtt_logging::telemetry::{Telemetry, SystemState};

// defmt timestamp
defmt::timestamp!("{=u32:ms}", {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let n = COUNTER.load(Ordering::Relaxed);
    COUNTER.store(n + 1, Ordering::Relaxed);
    n
});

// Panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", defmt::Debug2Format(info));
    loop {}
}

// ============================================================================
// MAIN
// ============================================================================

#[main]
fn main() -> ! {
    info!("Starting Lesson 09: RTT Variable Streaming");

    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    info!("ESP32-C6 booted, RTT ready for telemetry");

    // Create telemetry instance
    let mut telemetry = Telemetry::new();

    // === Phase 1: Initialization ===
    info!("=== Phase 1: Hardware Initialization ===");
    telemetry.state.transition(SystemState::Initializing);

    // Simulate I2C initialization and config write
    telemetry.i2c.record_write(0x48, 0x8483);
    telemetry.i2c.record_write_success();
    telemetry.config.written = 0x8483;
    telemetry.config.mux = 0;
    telemetry.config.pga = 1;
    telemetry.config.mode = 0;
    telemetry.config.dr = 7;
    telemetry.state.transition(SystemState::ConfigWritten);
    telemetry.log_all();

    // Verify config by reading back
    delay.delay_millis(10);
    telemetry.i2c.record_read(0x48);
    telemetry.i2c.record_read_success();
    telemetry.config.readback = 0x8483;
    telemetry.state.transition(SystemState::ConfigVerified);
    telemetry.log_all();

    // === Phase 2: Steady-state operation ===
    info!("=== Phase 2: Steady-state Operation (streaming every 100ms) ===");
    telemetry.state.transition(SystemState::Idle);

    let mut iteration: u32 = 0;

    loop {
        // Simulate ADC reading cycle
        telemetry.state.transition(SystemState::ConversionInProgress);
        delay.delay_millis(5);

        // Simulate conversion complete
        let raw_adc = 0x0ABC + (iteration as u16 % 64);
        telemetry.adc.raw = raw_adc;
        telemetry.adc.volts = ((raw_adc as f32 - 2048.0) / 2048.0) * 4.096;
        telemetry.adc.ready = true;
        telemetry.adc.busy = false;

        // Record I2C transaction
        telemetry.i2c.record_read(0x48);
        telemetry.i2c.record_read_success();

        // Update data quality
        telemetry.data_quality.update(raw_adc);

        // Track state timing
        telemetry.state.state = SystemState::ResultReady;
        telemetry.state.update_time(100);

        // Log all telemetry every 100ms
        if iteration % 10 == 0 {
            telemetry.log_all();
        }

        // Also log critical state more frequently
        if iteration % 5 == 0 {
            telemetry.log_critical();
        }

        iteration += 1;
        delay.delay_millis(10);

        // Occasionally simulate an error to see error handling in logs
        if iteration % 100 == 50 {
            telemetry.i2c.record_error();
            info!("Simulated I2C error for demonstration");
        }
    }
}

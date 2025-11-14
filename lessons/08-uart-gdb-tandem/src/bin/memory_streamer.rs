//! # Arbitrary Runtime Variable Streaming
//!
//! Generic memory streamer that accepts commands to read arbitrary memory addresses
//! at specified rates. Designed for dynamic debugging without compile-time registration.
//!
//! **Protocol:**
//! - Host → ESP32: `STREAM <addr> <size> <rate_hz>\n`
//! - ESP32 → Host: `DATA <addr> <hex_bytes>\n`
//! - Host → ESP32: `STOP <addr>\n`
//! - Host → ESP32: `PING\n` → ESP32: `PONG\n`

#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{delay::Delay, main, uart::Uart};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

/// Maximum number of concurrent streams
const MAX_STREAMS: usize = 16;

/// Stream configuration
#[derive(Clone, Copy, Debug)]
struct StreamConfig {
    addr: u32,
    size: usize,
    rate_hz: u32,
    last_sample_ms: u64,
    enabled: bool,
}

impl StreamConfig {
    fn new() -> Self {
        Self {
            addr: 0,
            size: 0,
            rate_hz: 0,
            last_sample_ms: 0,
            enabled: false,
        }
    }

    fn should_sample(&mut self, now_ms: u64) -> bool {
        if !self.enabled || self.rate_hz == 0 {
            return false;
        }
        let interval_ms = 1000 / self.rate_hz as u64;
        if now_ms - self.last_sample_ms >= interval_ms {
            self.last_sample_ms = now_ms;
            true
        } else {
            false
        }
    }
}

/// Global stream list
static mut STREAMS: [StreamConfig; MAX_STREAMS] = [StreamConfig {
    addr: 0,
    size: 0,
    rate_hz: 0,
    last_sample_ms: 0,
    enabled: false,
}; MAX_STREAMS];

/// Command buffer for incoming commands
static mut CMD_BUFFER: [u8; 256] = [0u8; 256];
static mut CMD_LEN: usize = 0;

#[main]
fn main() -> ! {
    println!("BOOT|version=1.0.0|chip=ESP32-C6|mode=memory_streamer");

    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!("STATUS|msg=Memory streamer ready");
    println!("STATUS|msg=Max streams: {}|rate_limit=10000Hz", MAX_STREAMS);

    let mut timestamp_ms: u64 = 0;
    let mut heartbeat_counter: u32 = 0;

    loop {
        // Process incoming commands (non-blocking read)
        // Note: esp-println uses USB CDC but doesn't provide read capability
        // We'll need to add UART reading in a future iteration
        // For now, commands can be added via GDB or compile-time initialization

        // Sample all active streams
        unsafe {
            for stream in STREAMS.iter_mut() {
                if stream.should_sample(timestamp_ms) {
                    sample_and_print(stream);
                }
            }
        }

        // Heartbeat every second
        if timestamp_ms % 1000 == 0 {
            println!("HEARTBEAT|count={}|ts={}|active={}",
                heartbeat_counter, timestamp_ms, count_active_streams());
            heartbeat_counter += 1;
        }

        timestamp_ms += 10;
        delay.delay_millis(10); // 100 Hz base loop
    }
}

/// Sample memory and print as hex
fn sample_and_print(stream: &StreamConfig) {
    // Safety: This is inherently unsafe - reading arbitrary memory
    // Risks: torn reads, MMIO side effects, invalid addresses
    unsafe {
        let ptr = stream.addr as *const u8;

        // Basic validation: check if address is in valid RAM range
        // ESP32-C6 SRAM: 0x4080_0000 - 0x4088_0000 (512 KB)
        if stream.addr < 0x4080_0000 || stream.addr >= 0x4088_0000 {
            println!("ERROR|addr=0x{:08x}|msg=Out of SRAM range", stream.addr);
            return;
        }

        // Read bytes
        let mut hex_buf = [0u8; 128]; // Max 64 bytes * 2 hex chars
        let mut hex_len = 0;

        for i in 0..stream.size.min(64) {
            let byte = ptr.add(i).read_volatile();
            hex_buf[hex_len] = HEX_CHARS[(byte >> 4) as usize];
            hex_buf[hex_len + 1] = HEX_CHARS[(byte & 0x0F) as usize];
            hex_len += 2;
        }

        // Print as string (avoid allocations)
        let hex_str = core::str::from_utf8_unchecked(&hex_buf[..hex_len]);
        println!("DATA|addr=0x{:08x}|hex={}", stream.addr, hex_str);
    }
}

/// Count active streams
fn count_active_streams() -> usize {
    unsafe {
        STREAMS.iter().filter(|s| s.enabled).count()
    }
}

/// Hex character lookup table
const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

/// Add a new stream (would be called from command parser)
#[allow(dead_code)]
fn add_stream(addr: u32, size: usize, rate_hz: u32) -> Result<(), &'static str> {
    unsafe {
        // Find empty slot
        for stream in STREAMS.iter_mut() {
            if !stream.enabled {
                stream.addr = addr;
                stream.size = size;
                stream.rate_hz = rate_hz;
                stream.last_sample_ms = 0;
                stream.enabled = true;
                println!("STATUS|msg=Stream added|addr=0x{:08x}|size={}|rate={}",
                    addr, size, rate_hz);
                return Ok(());
            }
        }
        Err("Max streams reached")
    }
}

/// Remove a stream
#[allow(dead_code)]
fn remove_stream(addr: u32) -> Result<(), &'static str> {
    unsafe {
        for stream in STREAMS.iter_mut() {
            if stream.enabled && stream.addr == addr {
                stream.enabled = false;
                println!("STATUS|msg=Stream removed|addr=0x{:08x}", addr);
                return Ok(());
            }
        }
        Err("Stream not found")
    }
}

// Example: Add some test streams at boot (will be replaced by command parser)
#[allow(dead_code)]
fn init_test_streams() {
    // Stream timestamp variable itself (meta!)
    // We'd need to know its address from the symbol table
    // For now, this is a placeholder
    let _ = add_stream(0x4080_1000, 4, 10); // Example: 4 bytes at 10 Hz
}

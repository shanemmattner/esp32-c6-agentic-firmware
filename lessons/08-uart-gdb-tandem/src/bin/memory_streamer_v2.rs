//! # Arbitrary Runtime Variable Streaming v2
//!
//! Bidirectional variable streaming using:
//! - USB CDC (esp-println) for OUTPUT (high-speed data streaming)
//! - UART1 for INPUT (command reception)
//!
//! This avoids esp-println's read limitation while maintaining high-speed output.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    main,
    uart::{Config, Uart},
    Blocking,
};
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
    const fn new() -> Self {
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
static mut STREAMS: [StreamConfig; MAX_STREAMS] = [StreamConfig::new(); MAX_STREAMS];

/// Test variables that we can stream
static mut TEST_COUNTER: u32 = 0;
static mut TEST_SENSOR_TEMP: i32 = 2530; // 25.30°C in centi-celsius
static mut TEST_SENSOR_ACCEL_X: i16 = 0;
static mut TEST_SENSOR_ACCEL_Y: i16 = 0;
static mut TEST_SENSOR_ACCEL_Z: i16 = 1000; // 1g
static mut TEST_STATE_MACHINE: u8 = 0;
static mut TEST_TIMESTAMP: u64 = 0;

/// Command buffer
static mut CMD_BUFFER: [u8; 256] = [0u8; 256];
static mut CMD_LEN: usize = 0;

#[main]
fn main() -> ! {
    println!("BOOT|version=2.0.0|chip=ESP32-C6|mode=bidirectional_streaming");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Configure UART1 for command input
    // ESP32-C6: Use GPIO15=TX, GPIO23=RX (known working pins from previous lessons)
    let mut uart = Uart::new(peripherals.UART1, Config::default())
        .expect("Failed to initialize UART1")
        .with_rx(peripherals.GPIO23)
        .with_tx(peripherals.GPIO15);

    println!("STATUS|msg=UART1 configured (GPIO15=TX, GPIO23=RX)");
    println!("STATUS|msg=Max streams: {}|rate_limit=10000Hz", MAX_STREAMS);
    println!("STATUS|msg=Test variables: counter, sensor_temp, accel_x/y/z, state, timestamp");

    // Print test variable addresses for easy reference
    unsafe {
        println!("VARS|counter=0x{:08x}|sensor_temp=0x{:08x}|accel_x=0x{:08x}|accel_y=0x{:08x}|accel_z=0x{:08x}|state=0x{:08x}|timestamp=0x{:08x}",
            &TEST_COUNTER as *const u32 as u32,
            &TEST_SENSOR_TEMP as *const i32 as u32,
            &TEST_SENSOR_ACCEL_X as *const i16 as u32,
            &TEST_SENSOR_ACCEL_Y as *const i16 as u32,
            &TEST_SENSOR_ACCEL_Z as *const i16 as u32,
            &TEST_STATE_MACHINE as *const u8 as u32,
            &TEST_TIMESTAMP as *const u64 as u32,
        );
    }

    println!("READY");

    let mut timestamp_ms: u64 = 0;

    loop {
        // Update test variables (simulate sensors)
        unsafe {
            TEST_COUNTER = TEST_COUNTER.wrapping_add(1);
            TEST_TIMESTAMP = timestamp_ms;
            TEST_SENSOR_TEMP = 2500 + ((timestamp_ms / 100) % 100) as i32; // 25.00-25.99°C
            TEST_SENSOR_ACCEL_X = ((timestamp_ms / 10) % 200) as i16 - 100; // -100 to +100
            TEST_STATE_MACHINE = ((timestamp_ms / 1000) % 5) as u8; // 0-4
        }

        // Process incoming UART commands (non-blocking)
        process_uart_commands(&mut uart);

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
            println!(
                "HEARTBEAT|ts={}|active={}|counter={}",
                timestamp_ms,
                count_active_streams(),
                unsafe { TEST_COUNTER }
            );
        }

        timestamp_ms += 10;
        delay.delay_millis(10); // 100 Hz base loop
    }
}

/// Process incoming UART commands
fn process_uart_commands(uart: &mut Uart<Blocking>) {
    let mut buffer = [0u8; 1];

    // Read available bytes (non-blocking)
    while uart.read(&mut buffer).is_ok() {
        let byte = buffer[0];

        unsafe {
            // Add to buffer
            if CMD_LEN < CMD_BUFFER.len() {
                CMD_BUFFER[CMD_LEN] = byte;
                CMD_LEN += 1;

                // Check for newline
                if byte == b'\n' {
                    // Process command
                    let cmd_slice = &CMD_BUFFER[..CMD_LEN - 1]; // Exclude newline
                    process_command(cmd_slice);
                    CMD_LEN = 0; // Reset buffer
                }
            } else {
                // Buffer overflow - reset
                println!("ERROR|msg=Command buffer overflow");
                CMD_LEN = 0;
            }
        }
    }
}

/// Process a complete command
fn process_command(cmd: &[u8]) {
    // Try to convert to string
    if let Ok(cmd_str) = core::str::from_utf8(cmd) {
        let parts: heapless::Vec<&str, 8> = cmd_str.split_whitespace().collect();

        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "PING" => {
                println!("PONG");
            }
            "STREAM" if parts.len() >= 4 => {
                // STREAM <addr> <size> <rate_hz>
                if let (Some(addr_str), Some(size_str), Some(rate_str)) =
                    (parts.get(1), parts.get(2), parts.get(3))
                {
                    // Parse hex address
                    let addr = if addr_str.starts_with("0x") {
                        u32::from_str_radix(&addr_str[2..], 16)
                    } else {
                        addr_str.parse()
                    };

                    let size = size_str.parse();
                    let rate_hz = rate_str.parse();

                    if let (Ok(addr), Ok(size), Ok(rate_hz)) = (addr, size, rate_hz) {
                        match add_stream(addr, size, rate_hz) {
                            Ok(_) => println!("OK|cmd=STREAM|addr=0x{:08x}", addr),
                            Err(e) => println!("ERROR|cmd=STREAM|msg={}", e),
                        }
                    } else {
                        println!("ERROR|cmd=STREAM|msg=Invalid parameters");
                    }
                }
            }
            "STOP" if parts.len() >= 2 => {
                // STOP <addr>
                if let Some(addr_str) = parts.get(1) {
                    let addr = if addr_str.starts_with("0x") {
                        u32::from_str_radix(&addr_str[2..], 16)
                    } else {
                        addr_str.parse()
                    };

                    if let Ok(addr) = addr {
                        match remove_stream(addr) {
                            Ok(_) => println!("OK|cmd=STOP|addr=0x{:08x}", addr),
                            Err(e) => println!("ERROR|cmd=STOP|msg={}", e),
                        }
                    }
                }
            }
            "HELP" => {
                println!("HELP|commands=PING,STREAM,STOP,LIST,HELP");
            }
            "LIST" => {
                unsafe {
                    println!("VARS|counter=0x{:08x}|sensor_temp=0x{:08x}|accel_x=0x{:08x}",
                        &TEST_COUNTER as *const u32 as u32,
                        &TEST_SENSOR_TEMP as *const i32 as u32,
                        &TEST_SENSOR_ACCEL_X as *const i16 as u32,
                    );
                }
            }
            _ => {
                println!("ERROR|msg=Unknown command: {}", parts[0]);
            }
        }
    }
}

/// Sample memory and print as hex
fn sample_and_print(stream: &StreamConfig) {
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

        // Print as string
        let hex_str = core::str::from_utf8_unchecked(&hex_buf[..hex_len]);
        println!("DATA|addr=0x{:08x}|hex={}", stream.addr, hex_str);
    }
}

/// Count active streams
fn count_active_streams() -> usize {
    unsafe { STREAMS.iter().filter(|s| s.enabled).count() }
}

/// Hex character lookup
const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

/// Add a new stream
fn add_stream(addr: u32, size: usize, rate_hz: u32) -> Result<(), &'static str> {
    unsafe {
        for stream in STREAMS.iter_mut() {
            if !stream.enabled {
                stream.addr = addr;
                stream.size = size;
                stream.rate_hz = rate_hz;
                stream.last_sample_ms = 0;
                stream.enabled = true;
                return Ok(());
            }
        }
        Err("Max streams reached")
    }
}

/// Remove a stream
fn remove_stream(addr: u32) -> Result<(), &'static str> {
    unsafe {
        for stream in STREAMS.iter_mut() {
            if stream.enabled && stream.addr == addr {
                stream.enabled = false;
                return Ok(());
            }
        }
        Err("Stream not found")
    }
}

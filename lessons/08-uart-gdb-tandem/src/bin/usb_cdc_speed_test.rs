//! # USB CDC Speed Test
//!
//! Tests maximum USB CDC throughput using esp-println.
//! Streams test data continuously to measure actual throughput.
//!
//! Uses built-in USB CDC (no external UART needed)

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, main};
use esp_println::{print, println};

esp_bootloader_esp_idf::esp_app_desc!();

/// Test packet: 64 bytes of structured data
#[repr(C, packed)]
struct TestPacket {
    magic: u32,        // 0xDEADBEEF
    seq: u32,          // Sequence number
    timestamp_ms: u64, // Timestamp
    counter: u32,      // Counter value
    sensor_temp: i32,  // Simulated temp (centi-celsius)
    accel_x: i16,      // Simulated accel X
    accel_y: i16,      // Simulated accel Y
    accel_z: i16,      // Simulated accel Z
    state: u8,         // FSM state
    padding: [u8; 29], // Pad to 64 bytes
    checksum: u32,     // Simple checksum
}

impl TestPacket {
    fn new(seq: u32, timestamp_ms: u64, counter: u32) -> Self {
        let mut pkt = Self {
            magic: 0xDEADBEEF,
            seq,
            timestamp_ms,
            counter,
            sensor_temp: 2500 + ((timestamp_ms / 100) % 100) as i32,
            accel_x: ((timestamp_ms / 10) % 200) as i16 - 100,
            accel_y: ((timestamp_ms / 15) % 200) as i16 - 100,
            accel_z: 1000 + ((timestamp_ms / 20) % 100) as i16,
            state: ((timestamp_ms / 1000) % 5) as u8,
            padding: [0; 29],
            checksum: 0,
        };

        // Simple checksum: sum of all u32 words
        pkt.checksum = pkt.magic
            .wrapping_add(pkt.seq)
            .wrapping_add((pkt.timestamp_ms >> 32) as u32)
            .wrapping_add(pkt.timestamp_ms as u32)
            .wrapping_add(pkt.counter)
            .wrapping_add(pkt.sensor_temp as u32);

        pkt
    }

    fn as_bytes(&self) -> &[u8; 64] {
        unsafe { &*(self as *const Self as *const [u8; 64]) }
    }
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // LED on GPIO8 - blink to show running
    use esp_hal::gpio::{Level, Output, OutputConfig};
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    println!("BOOT|version=usb_cdc_speed_test_1.0.0|chip=ESP32-C6");
    println!("STATUS|interface=USB_CDC|packet_size=64");
    println!("READY");

    let mut timestamp_ms: u64 = 0;
    let mut seq: u32 = 0;
    let mut counter: u32 = 0;
    let mut packets_sent: u32 = 0;

    loop {
        counter = counter.wrapping_add(1);

        // Blink LED every 500ms
        if timestamp_ms % 500 == 0 {
            led.toggle();
        }

        // Send packet every 10ms (100 Hz)
        if timestamp_ms % 10 == 0 {
            let packet = TestPacket::new(seq, timestamp_ms, counter);
            let bytes = packet.as_bytes();

            // Print as hex string for USB CDC
            print!("DATA|seq={}|hex=", seq);
            for &byte in bytes {
                print!("{:02x}", byte);
            }
            println!();

            seq = seq.wrapping_add(1);
            packets_sent = packets_sent.wrapping_add(1);
        }

        // Stats every second
        if timestamp_ms % 1000 == 0 {
            println!(
                "STATS|ts={}|seq={}|packets={}|throughput={} bytes/s",
                timestamp_ms,
                seq,
                packets_sent,
                packets_sent * 64
            );
            packets_sent = 0;
        }

        timestamp_ms += 10;
        delay.delay_millis(10);
    }
}

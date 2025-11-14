//! # Lesson 02: UART with DMA (UHCI) - High-Speed Data Streaming
//!
//! **Goal:** Learn UART communication with hardware-accelerated DMA
//!
//! **Hardware:**
//! - ESP32-C6 DevKit
//! - FTDI USB-to-serial adapter
//!
//! **Pins:**
//! - GPIO23: UART TX (ESP32 transmit → FTDI RX)
//! - GPIO15: UART RX (ESP32 receive ← FTDI TX)
//!
//! **What You'll Learn:**
//! - UART peripheral configuration
//! - Finding UART registers using PAC crate (like Lesson 01)
//! - **DMA via UHCI (Universal Host Controller Interface)**
//! - Hardware-accelerated data transfer (CPU-free streaming!)
//! - Baud rate tuning experiments (115200 → 921600 → 2000000)
//! - GDB register inspection during development
//! - Structured logging for debugging with Claude Code
//!
//! **What is UHCI DMA?**
//! - UHCI = Universal Host Controller Interface
//! - Provides DMA (Direct Memory Access) for UART on ESP32-C6
//! - Hardware transfers data between memory and UART peripheral
//! - CPU is completely free during transfers (no busy-waiting!)
//!
//! **Debugging Workflow:**
//! 1. Use `/find-registers UART1` and `/find-registers UHCI0` to discover registers
//! 2. Build and flash firmware
//! 3. Use GDB to inspect registers during streaming:
//!    ```
//!    (gdb) x/1xw 0x60001000  # UART1 FIFO
//!    (gdb) x/1xw 0x6000101C  # UART1 STATUS
//!    (gdb) x/16xw 0x60006000 # UHCI0 DMA registers
//!    ```
//! 4. Monitor output: `python3 .claude/templates/read_uart.py /dev/cu.usbserial* 10`
//! 5. Experiment with BAUD_RATE constant and re-flash

#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    dma::{DmaRxBuf, DmaTxBuf},
    dma_buffers,
    main,
    uart::{self, Config as UartConfig, RxConfig, Uart, uhci::Uhci},
};
use esp_println::println;
use heapless::String;

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// Configuration Constants
// ============================================================================

/// UART baud rate - **EXPERIMENT WITH THIS!**
/// Try these values and observe throughput:
/// - 115200   (standard, ~11 KB/s)
/// - 460800   (4x faster, ~46 KB/s)
/// - 921600   (8x faster, ~92 KB/s)
/// - 2000000  (very fast, ~200 KB/s)
const BAUD_RATE: u32 = 921_600;

/// How often to send data (milliseconds)
const STREAM_INTERVAL_MS: u32 = 100;

/// DMA buffer size (bytes)
const DMA_BUFFER_SIZE: usize = 4092;

// ============================================================================
// Test Data - Variables to Stream
// ============================================================================

/// Counter that increments each iteration
static mut COUNTER: u32 = 0;

/// Simulated sensor value (increments by 10 each time)
static mut SENSOR_VALUE: i32 = 1000;

/// Random-looking value (for testing data integrity)
static mut CHECKSUM: u16 = 0xABCD;

// ============================================================================
// Main Application
// ============================================================================

#[main]
fn main() -> ! {
    println!("🚀 Lesson 02: UART + DMA (UHCI) Streaming\n");

    // Initialize hardware
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // ========================================================================
    // Initialize UART
    // ========================================================================
    //
    // First, create a basic UART instance
    // Then wrap it with UHCI for DMA support

    let config = UartConfig::default()
        .with_rx(RxConfig::default().with_fifo_full_threshold(64))
        .with_baudrate(BAUD_RATE);

    let uart = Uart::new(peripherals.UART1, config)
        .expect("Failed to initialize UART")
        .with_tx(peripherals.GPIO23)
        .with_rx(peripherals.GPIO15);

    println!("✓ UART initialized:");
    println!("  - Baud rate: {} bps ({} KB/s)", BAUD_RATE, BAUD_RATE / 8000);
    println!("  - TX: GPIO23");
    println!("  - RX: GPIO15");

    // ========================================================================
    // Initialize UHCI DMA
    // ========================================================================
    //
    // **What is UHCI?**
    // - Universal Host Controller Interface
    // - Provides DMA functionality for UART
    // - Hardware moves data between memory buffers and UART FIFO
    //
    // **How it works:**
    // 1. Firmware writes data to DMA TX buffer
    // 2. Call uhci_tx.write(dma_tx) to start transfer
    // 3. UHCI hardware streams bytes from buffer to UART
    // 4. CPU is FREE to do other work!
    // 5. transfer.wait() blocks until DMA completes

    // Create DMA buffers (separate for TX and RX)
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(DMA_BUFFER_SIZE);
    let _dma_rx = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let mut dma_tx = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    // Wrap UART with UHCI for DMA support
    let mut uhci = Uhci::new(uart, peripherals.UHCI0, peripherals.DMA_CH0);

    // Configure RX and TX
    uhci.apply_rx_config(&uart::uhci::RxConfig::default().with_chunk_limit(DMA_BUFFER_SIZE as u16))
        .expect("Failed to apply RX config");
    uhci.apply_tx_config(&uart::uhci::TxConfig::default())
        .expect("Failed to apply TX config");

    // Split into separate TX and RX halves
    let (_uhci_rx, mut uhci_tx) = uhci.split();

    println!("✓ UHCI DMA initialized:");
    println!("  - Buffer size: {} bytes", DMA_BUFFER_SIZE);
    println!("  - DMA channel: CH0");
    println!("  - 🎯 Hardware-accelerated transfers!");
    println!();

    // ========================================================================
    // Print Register Discovery Guide
    // ========================================================================

    println!("📖 GDB Debugging Guide:");
    println!("  1. Find UART1 and UHCI0 registers:");
    println!("     python3 scripts/find-registers.py UART1");
    println!("     python3 scripts/find-registers.py UHCI0");
    println!();
    println!("  2. Start GDB session:");
    println!("     riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main");
    println!("     (gdb) target remote :3333");
    println!();
    println!("  3. Inspect during streaming:");
    println!("     (gdb) x/1xw 0x60001000  # UART1 FIFO");
    println!("     (gdb) x/1xw 0x6000101C  # UART1 STATUS");
    println!("     (gdb) x/16xw 0x60006000 # UHCI0 DMA registers");
    println!();
    println!("  4. Modify variables live:");
    println!("     (gdb) set SENSOR_VALUE = 9999");
    println!("     (gdb) continue");
    println!();
    println!("🌊 Starting DMA streaming...\n");

    // ========================================================================
    // Main Loop - Stream Variables via UART DMA
    // ========================================================================

    let mut iteration: u64 = 0;

    loop {
        delay.delay_millis(STREAM_INTERVAL_MS);

        // Update test variables
        unsafe {
            COUNTER = COUNTER.wrapping_add(1);
            SENSOR_VALUE = SENSOR_VALUE.wrapping_add(10);
            if SENSOR_VALUE > 5000 {
                SENSOR_VALUE = 1000;
            }
            // Simple checksum: XOR of counter and sensor
            CHECKSUM = ((COUNTER ^ SENSOR_VALUE as u32) & 0xFFFF) as u16;
        }

        // Create formatted output
        let mut buffer = String::<128>::new();
        let _ = write!(
            buffer,
            "stream: iter={} counter={} sensor={} checksum=0x{:04X}\n",
            iteration,
            unsafe { COUNTER },
            unsafe { SENSOR_VALUE },
            unsafe { CHECKSUM }
        );

        // Copy to DMA buffer
        let bytes = buffer.as_bytes();
        let len = bytes.len().min(DMA_BUFFER_SIZE);
        dma_tx.as_mut_slice()[0..len].copy_from_slice(&bytes[0..len]);
        dma_tx.set_length(len);

        // **Start DMA transfer!**
        // This hands the buffer to UHCI hardware and returns a Transfer object
        // The hardware will stream bytes to UART while CPU continues
        let transfer = uhci_tx.write(dma_tx)
            .unwrap_or_else(|err| panic!("Failed to start DMA: {:?}", err.0));

        // Wait for DMA to complete
        // (In a real application, you could do other work here!)
        let (result, uhci, dma) = transfer.wait();
        result.unwrap();

        // Reclaim the TX channel and buffer for next iteration
        uhci_tx = uhci;
        dma_tx = dma;

        // Also print to USB CDC for debugging without FTDI
        println!("{}", buffer.trim_end());

        iteration += 1;

        // Every 10 iterations, print stats
        if iteration % 10 == 0 {
            let bytes_per_sec = len * (1000 / STREAM_INTERVAL_MS as usize);
            println!("\n📊 Stats after {} iterations:", iteration);
            println!("  Baud rate: {} bps ({} KB/s theoretical)",
                     BAUD_RATE, BAUD_RATE / 8000);
            println!("  Actual throughput: ~{} bytes/sec ({} KB/s)",
                     bytes_per_sec, bytes_per_sec / 1000);
            println!("  Message size: {} bytes", len);
            println!("  🎯 DMA hardware does all the work!");
            println!();
        }
    }
}

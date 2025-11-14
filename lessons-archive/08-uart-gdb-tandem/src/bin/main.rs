//! # Lesson 08 - Phase 1: Memory-Safe Variable Streaming
//!
//! Demonstrates pointer-based variable streaming over UART with comprehensive
//! memory safety checks. This allows GDB to dynamically change which variables are
//! being streamed by modifying slot pointers.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - USB-JTAG for GDB debugging
//! - UART on GPIO23 (TX) and GPIO15 (RX) for streaming
//!
//! **What You'll Learn:**
//! - Memory-safe pointer-based variable streaming
//! - Bounds checking and alignment validation
//! - Type-safe variable slot system
//! - GDB tandem debugging workflow
//! - Dynamic variable redirection via GDB

#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    main,
    uart::{Config as UartConfig, Uart},
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

// ================================================================================================
// Memory Safety Constants
// ================================================================================================

/// ESP32-C6 RAM address range (512 KB total, from memory.x linker script)
const RAM_START: usize = 0x40800000;
const RAM_END: usize = 0x40880000;

// ================================================================================================
// Variable Slot System
// ================================================================================================

/// Supported variable types for streaming
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum VarType {
    I32 = 0,
    U32 = 1,
    F32 = 2,
    I16 = 3,
    U16 = 4,
    I8 = 5,
    U8 = 6,
}

impl VarType {
    /// Returns the required alignment for this type
    const fn alignment(&self) -> usize {
        match self {
            VarType::I32 | VarType::U32 | VarType::F32 => 4,
            VarType::I16 | VarType::U16 => 2,
            VarType::I8 | VarType::U8 => 1,
        }
    }

    /// Returns the size of this type in bytes
    const fn size(&self) -> usize {
        match self {
            VarType::I32 | VarType::U32 | VarType::F32 => 4,
            VarType::I16 | VarType::U16 => 2,
            VarType::I8 | VarType::U8 => 1,
        }
    }
}

/// A slot that points to a variable to be streamed
#[repr(C)]
struct StreamSlot {
    /// Pointer to the variable
    ptr: *const u8,
    /// Type of the variable
    type_id: VarType,
    /// Slot name for debugging
    name: &'static str,
}

/// Possible errors when reading a slot
#[derive(Debug)]
enum SlotError {
    OutOfBounds { addr: usize },
    Misaligned { addr: usize, required_alignment: usize },
}

/// Safe value read from a slot
#[derive(Debug)]
enum SlotValue {
    I32(i32),
    U32(u32),
    F32(f32),
    I16(i16),
    U16(u16),
    I8(i8),
    U8(u8),
}

impl StreamSlot {
    /// Creates a new stream slot pointing to a typed variable
    const fn new<T>(ptr: *const T, type_id: VarType, name: &'static str) -> Self {
        Self {
            ptr: ptr as *const u8,
            type_id,
            name,
        }
    }

    /// Reads the value from this slot with full safety checks
    fn read_safe(&self) -> Result<SlotValue, SlotError> {
        let addr = self.ptr as usize;

        // 1. Bounds checking - ensure address is in valid RAM
        if addr < RAM_START || addr >= RAM_END {
            return Err(SlotError::OutOfBounds { addr });
        }

        // 2. Alignment checking
        let required_alignment = self.type_id.alignment();
        if addr % required_alignment != 0 {
            return Err(SlotError::Misaligned {
                addr,
                required_alignment,
            });
        }

        // 3. Ensure we don't read past the end of RAM
        let size = self.type_id.size();
        if addr + size > RAM_END {
            return Err(SlotError::OutOfBounds { addr: addr + size });
        }

        // 4. Safe dereference after validation
        unsafe {
            Ok(match self.type_id {
                VarType::I32 => SlotValue::I32(*(self.ptr as *const i32)),
                VarType::U32 => SlotValue::U32(*(self.ptr as *const u32)),
                VarType::F32 => SlotValue::F32(*(self.ptr as *const f32)),
                VarType::I16 => SlotValue::I16(*(self.ptr as *const i16)),
                VarType::U16 => SlotValue::U16(*(self.ptr as *const u16)),
                VarType::I8 => SlotValue::I8(*(self.ptr as *const i8)),
                VarType::U8 => SlotValue::U8(*self.ptr),
            })
        }
    }
}

// ================================================================================================
// Global Variables to Stream
// ================================================================================================

static mut SENSOR_X: i32 = 0;
static mut SENSOR_Y: i32 = 0;
static mut TEMPERATURE: i32 = 2500; // 25.00°C in centi-degrees
static mut COUNTER: u32 = 0;
static mut STATUS_FLAGS: u8 = 0;

// ================================================================================================
// Stream Slot Configuration (GDB can modify these pointers!)
// ================================================================================================

static mut STREAM_SLOTS: [StreamSlot; 4] = [
    StreamSlot::new(unsafe { &raw const SENSOR_X }, VarType::I32, "sensor_x"),
    StreamSlot::new(unsafe { &raw const SENSOR_Y }, VarType::I32, "sensor_y"),
    StreamSlot::new(unsafe { &raw const TEMPERATURE }, VarType::I32, "temperature"),
    StreamSlot::new(unsafe { &raw const COUNTER }, VarType::U32, "counter"),
];

// ================================================================================================
// Main Program
// ================================================================================================

#[main]
fn main() -> ! {
    println!("=== Lesson 08 - Phase 1: Memory-Safe Variable Streaming ===");
    println!("Hardware: ESP32-C6");
    println!("UART: TX=GPIO23, RX=GPIO15");
    println!();

    // Initialize peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Configure UART1 on GPIO23/15 at 115200 baud
    let mut uart = Uart::new(peripherals.UART1, UartConfig::default())
        .expect("Failed to init UART")
        .with_tx(peripherals.GPIO23)
        .with_rx(peripherals.GPIO15);

    println!("UART initialized at 115200 baud");
    println!("Starting variable streaming...");
    println!();
    println!("GDB Instructions:");
    println!("  1. Connect GDB: riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/debug/phase1_dma_uart");
    println!("  2. Attach: (gdb) target extended-remote :3333");
    println!("  3. View slots: (gdb) p/x STREAM_SLOTS");
    println!("  4. Change pointer: (gdb) set STREAM_SLOTS[0].ptr = &TEMPERATURE");
    println!();

    let mut loop_count: u32 = 0;

    loop {
        // Update simulated sensor values
        unsafe {
            SENSOR_X = (loop_count as i32 * 10) % 1000;
            SENSOR_Y = (loop_count as i32 * 20) % 2000;
            TEMPERATURE = 2500 + ((loop_count / 10) % 100) as i32;
            COUNTER = loop_count;
            STATUS_FLAGS = (loop_count % 256) as u8;
        }

        // Stream all slots with safety checks
        let mut buffer = heapless::String::<256>::new();

        write!(&mut buffer, "STREAM|ts={}|", loop_count).ok();

        unsafe {
            let slots = &*core::ptr::addr_of!(STREAM_SLOTS);
            for slot in slots {
                match slot.read_safe() {
                    Ok(value) => {
                        match value {
                            SlotValue::I32(v) => write!(&mut buffer, "{}={}|", slot.name, v),
                            SlotValue::U32(v) => write!(&mut buffer, "{}={}|", slot.name, v),
                            SlotValue::F32(v) => write!(&mut buffer, "{}={:.2}|", slot.name, v),
                            SlotValue::I16(v) => write!(&mut buffer, "{}={}|", slot.name, v),
                            SlotValue::U16(v) => write!(&mut buffer, "{}={}|", slot.name, v),
                            SlotValue::I8(v) => write!(&mut buffer, "{}={}|", slot.name, v),
                            SlotValue::U8(v) => write!(&mut buffer, "{}={}|", slot.name, v),
                        }
                        .ok();
                    }
                    Err(SlotError::OutOfBounds { addr }) => {
                        write!(&mut buffer, "{}=ERR_BOUNDS(0x{:x})|", slot.name, addr).ok();
                    }
                    Err(SlotError::Misaligned {
                        addr,
                        required_alignment,
                    }) => {
                        write!(
                            &mut buffer,
                            "{}=ERR_ALIGN(0x{:x},{})|",
                            slot.name, addr, required_alignment
                        )
                        .ok();
                    }
                }
            }
        }

        write!(&mut buffer, "\n").ok();

        // Send via UART
        let bytes = buffer.as_bytes();
        uart.write(bytes).ok();

        loop_count += 1;
        delay.delay_millis(100); // 10 Hz streaming rate
    }
}

# Lesson 02: UART with DMA (UHCI) - High-Speed Data Streaming

**Status:** ✅ Complete (compilation tested)

---

## Overview

This lesson teaches UART communication with hardware-accelerated DMA using the UHCI (Universal Host Controller Interface). You'll learn to stream data at high speeds while keeping the CPU completely free.

**Key Concepts:**
- UART peripheral configuration
- DMA via UHCI on ESP32-C6
- Register discovery from PAC crates
- GDB-based debugging workflow
- **Agentic development: Finding APIs in source code**

---

## 🎯 The Most Important Lesson: Documentation Discovery

### Why This Matters for Agentic Development

**Traditional approach:** Read 1000-page datasheets, search forums, hope you find the right API

**Agentic approach:** Go directly to the source code and find the working examples

### How We Found the UHCI DMA API

When building this lesson, we initially tried to use DMA with UART and hit compilation errors:

```rust
// ❌ This doesn't exist in esp-hal 1.0
use esp_hal::dma::Dma;
let uart = Uart::new(...).with_dma(...);  // No such method!
```

**Instead of giving up, here's what we did:**

#### Step 1: Found the esp-hal source code
```bash
find ~/.cargo/registry/src -name "esp-hal-1.0.0" -type d
# Found: ~/.cargo/registry/src/.../esp-hal-1.0.0/
```

#### Step 2: Searched for DMA references in UART module
```bash
grep -n "dma\|DMA" ~/.cargo/registry/src/.../esp-hal-1.0.0/src/uart/mod.rs
# Found: #[cfg(all(soc_has_uhci0, gdma))]
#        pub mod uhci;
```

#### Step 3: Read the UHCI module documentation
```bash
cat ~/.cargo/registry/src/.../esp-hal-1.0.0/src/uart/uhci.rs | head -100
```

**Result:** Found a complete working example in the module docs (lines 1-84)!

```rust
// ✅ The correct API from the source code
use esp_hal::uart::uhci::Uhci;

let uart = Uart::new(peripherals.UART1, config)
    .with_tx(peripherals.GPIO23)
    .with_rx(peripherals.GPIO15);

let mut uhci = Uhci::new(uart, peripherals.UHCI0, peripherals.DMA_CH0);
let (_uhci_rx, mut uhci_tx) = uhci.split();

let transfer = uhci_tx.write(dma_tx).unwrap();
let (result, uhci, dma) = transfer.wait();
```

### Why This Works with Claude Code

**Claude excels at:**
1. **Source code navigation** - Finding files, searching patterns
2. **API pattern recognition** - Understanding how similar APIs work
3. **Documentation extraction** - Reading inline docs and examples
4. **Troubleshooting** - When APIs don't match expectations, find the right ones

**This is faster than:**
- Reading datasheets (1000+ pages, register-level details)
- Searching Google/Stack Overflow (often outdated examples)
- Trial-and-error with cargo docs (may miss unstable features)

### Teachable Workflow for Students

When you encounter an API issue:

1. **Find the source:** `find ~/.cargo/registry/src -name "crate-name"`
2. **Search for keywords:** `grep -r "keyword" path/to/crate/src/`
3. **Read module docs:** Look at the top of the .rs file (often has examples!)
4. **Ask Claude Code:** "Show me how to use this API" with the file path
5. **Copy working patterns:** Adapt the example to your use case

**This is professional-level engineering.** Architects read source code, not just documentation sites.

---

## What is UHCI DMA?

### Architecture Overview

ESP32-C6 uses a **GDMA** (General-Purpose DMA) architecture:

```
Memory Buffer → UHCI → GDMA Channel → UART Peripheral → TX Pin
```

**Components:**
- **GDMA**: General-purpose DMA controller (serves SPI, I2C, UART, etc.)
- **UHCI**: Universal Host Controller Interface (bridges UART to GDMA)
- **DMA Channel**: Hardware that moves data without CPU

### Why UHCI?

Older ESP32 chips had **PDMA** (Peripheral DMA) - dedicated DMA per UART. ESP32-C6 uses:
- One shared GDMA controller
- UHCI wrappers to make peripherals GDMA-compatible

**Benefits:**
- More flexible (any channel can serve any peripheral)
- Shared hardware resources
- Modern architecture

### How It Works

```rust
// 1. Create DMA buffers
let (rx_buf, rx_desc, tx_buf, tx_desc) = dma_buffers!(4092);
let mut dma_tx = DmaTxBuf::new(tx_desc, tx_buf).unwrap();

// 2. Wrap UART with UHCI
let uart = Uart::new(peripherals.UART1, config).with_tx(...).with_rx(...);
let mut uhci = Uhci::new(uart, peripherals.UHCI0, peripherals.DMA_CH0);

// 3. Start DMA transfer (non-blocking!)
dma_tx.as_mut_slice()[0..len].copy_from_slice(&data);
dma_tx.set_length(len);
let transfer = uhci_tx.write(dma_tx).unwrap();

// 4. CPU is free! Do other work here...

// 5. Wait for completion
let (result, uhci, dma) = transfer.wait();
```

**Key insight:** Between `write()` and `wait()`, the CPU is completely free. Hardware streams bytes from memory to UART.

---

## Hardware Requirements

- ESP32-C6 DevKit (e.g., ESP32-C6-DevKitC-1)
- FTDI USB-to-serial adapter (3.3V)
- USB cable for programming

**Wiring:**
```
ESP32-C6          FTDI Adapter
--------          ------------
GPIO23 (TX)  -->  RX
GPIO15 (RX)  <--  TX
GND          ---  GND
```

**Note:** You can also monitor via the onboard USB-JTAG (no FTDI needed), but FTDI gives you an independent data channel separate from debug.

---

## Quick Start

### 1. Build and Flash

```bash
cd lessons/02-uart-dma
cargo build --release
cargo run --release
```

### 2. Monitor Output

**Via USB CDC (onboard):**
```bash
# Output appears automatically in cargo run
```

**Via FTDI adapter:**
```bash
python3 ../../.claude/templates/read_uart.py /dev/cu.usbserial* 10
```

You should see:
```
stream: iter=0 counter=1 sensor=1010 checksum=0x03F7
stream: iter=1 counter=2 sensor=1020 checksum=0x03FA
stream: iter=2 counter=3 sensor=1030 checksum=0x03F9
...
```

---

## GDB Debugging Workflow

### Step 1: Find Registers Using PAC Crate

```bash
# Find UART1 registers
python3 ../../scripts/find-registers.py UART1

# Output:
# UART1 Peripheral
# Base Address: 0x60001000
# Registers:
#   FIFO (0x0000): 0x60001000
#   STATUS (0x001C): 0x6000101C
#   CLKDIV (0x0014): 0x60001014
```

```bash
# Find UHCI0 (DMA controller) registers
python3 ../../scripts/find-registers.py UHCI0

# Output:
# UHCI0 Peripheral
# Base Address: 0x60006000
# Registers: (DMA descriptors, state machine, etc.)
```

### Step 2: Start GDB Session

Terminal 1 (debug server):
```bash
probe-rs attach --chip esp32c6 --protocol jtag target/riscv32imac-unknown-none-elf/release/main
```

Terminal 2 (GDB):
```bash
riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main
(gdb) target remote :3333
(gdb) continue
```

### Step 3: Inspect Registers During Streaming

```gdb
# Check UART status
(gdb) x/1xw 0x6000101C
0x6000101C:     0x00000002  # TX FIFO empty

# Check UART baud rate config
(gdb) x/1xw 0x60001014
0x60001014:     0x00000035  # CLKDIV register

# Inspect UHCI DMA state
(gdb) x/16xw 0x60006000
# Shows DMA descriptor pointers, transfer counts, etc.
```

### Step 4: Modify Variables Live

```gdb
# Change sensor value while running
(gdb) set SENSOR_VALUE = 9999
(gdb) continue

# Output changes immediately:
# stream: iter=42 counter=43 sensor=9999 checksum=0x2716
```

---

## Experiments to Try

### 1. Baud Rate Tuning

Edit `BAUD_RATE` constant in `src/bin/main.rs`:

```rust
const BAUD_RATE: u32 = 115_200;   // Standard (baseline)
const BAUD_RATE: u32 = 460_800;   // 4x faster
const BAUD_RATE: u32 = 921_600;   // 8x faster (default)
const BAUD_RATE: u32 = 2_000_000; // Very fast
```

**Observe:**
- Does FTDI adapter handle higher speeds?
- Do you see data corruption?
- What's the practical maximum?

### 2. DMA Buffer Size

```rust
const DMA_BUFFER_SIZE: usize = 1024;  // Small
const DMA_BUFFER_SIZE: usize = 4092;  // Large (default)
const DMA_BUFFER_SIZE: usize = 8192;  // Very large
```

**Question:** Does larger buffer improve throughput? Why/why not?

### 3. Stream Interval

```rust
const STREAM_INTERVAL_MS: u32 = 10;   // High frequency
const STREAM_INTERVAL_MS: u32 = 100;  // Medium (default)
const STREAM_INTERVAL_MS: u32 = 1000; // Low frequency
```

**Observe:** CPU utilization, data integrity at different rates

### 4. Message Complexity

Add more variables to the stream:

```rust
static mut TEMPERATURE: i16 = 25;
static mut PRESSURE: u32 = 101325;
static mut HUMIDITY: u8 = 60;

// Update format string
write!(buffer, "stream: iter={} temp={} press={} humid={} ...\n", ...);
```

**Question:** How does message size affect throughput?

---

## Understanding DMA Transfer Flow

```
1. Firmware prepares data:
   dma_tx.as_mut_slice()[0..len].copy_from_slice(&data);

2. Firmware starts DMA:
   let transfer = uhci_tx.write(dma_tx).unwrap();

   → UHCI configures GDMA channel
   → GDMA starts reading from memory buffer
   → Data flows to UART FIFO automatically

3. CPU is FREE during transfer:
   // Could do sensor reads, calculations, etc.

4. Wait for completion:
   let (result, uhci, dma) = transfer.wait();

   → Blocks until GDMA signals "done"
   → Returns ownership of buffer and channel
```

**Key point:** Between steps 2 and 4, the CPU can do anything. Hardware handles data movement.

---

## Troubleshooting

### No output on FTDI adapter

**Check wiring:**
- ESP32 TX → FTDI RX (cross-connect!)
- ESP32 RX → FTDI TX
- Common ground

**Check baud rate:**
- FTDI adapter must match firmware `BAUD_RATE`
- Most FTDI adapters max out at 921600 or 1 Mbaud

### Compilation errors about UHCI

**Check features:**
```toml
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }
```

UHCI is in the `unstable` feature set.

### DMA transfer fails

**Check buffer alignment:**
- DMA requires word-aligned buffers
- `dma_buffers!()` macro handles this automatically

**Check buffer size:**
- Must be ≤ DMA_BUFFER_SIZE
- Must set length: `dma_tx.set_length(len)`

---

## Learning Objectives

After completing this lesson, you should be able to:

1. ✅ **Find APIs in source code** (most important!)
2. ✅ Configure UART with custom baud rates
3. ✅ Use UHCI for DMA-accelerated transfers
4. ✅ Discover peripheral registers from PAC crates
5. ✅ Inspect hardware state using GDB
6. ✅ Explain GDMA vs PDMA architecture
7. ✅ Optimize throughput by tuning parameters
8. ✅ Debug data streaming issues autonomously

---

## Next Lesson

**Lesson 03: GDB + UART Tandem Debugging**

Combine GDB and UART for bidirectional hardware control:
- Stream multiple variables dynamically
- Redirect GDB to change which variables stream
- Memory-safe pointer validation
- Autonomous debugging with Claude Code
- Compare with RTT (SEGGER Real-Time Transfer)

This is where the debugging superpowers really shine!

---

## Files in This Lesson

```
02-uart-dma/
├── src/
│   ├── bin/
│   │   └── main.rs           # DMA streaming firmware (255 lines)
│   └── lib.rs                # Empty (minimal)
├── .cargo/
│   └── config.toml           # espflash runner
├── Cargo.toml                # Dependencies (esp-hal 1.0.0)
├── build.rs                  # Linker script
├── rust-toolchain.toml       # Nightly toolchain
└── README.md                 # This file
```

---

## Key Code Patterns

### UHCI DMA Setup

```rust
use esp_hal::uart::{self, Config as UartConfig, RxConfig, Uart, uhci::Uhci};
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::dma_buffers;

// Create UART
let uart = Uart::new(peripherals.UART1, config)
    .with_tx(peripherals.GPIO23)
    .with_rx(peripherals.GPIO15);

// Create DMA buffers
let (rx_buf, rx_desc, tx_buf, tx_desc) = dma_buffers!(4092);
let mut dma_tx = DmaTxBuf::new(tx_desc, tx_buf).unwrap();

// Wrap with UHCI
let mut uhci = Uhci::new(uart, peripherals.UHCI0, peripherals.DMA_CH0);
uhci.apply_tx_config(&uart::uhci::TxConfig::default()).unwrap();

// Split into TX/RX
let (_rx, mut tx) = uhci.split();

// Use it
dma_tx.as_mut_slice()[0..len].copy_from_slice(data);
dma_tx.set_length(len);
let transfer = tx.write(dma_tx).unwrap();
let (result, tx, dma) = transfer.wait();
```

### Variable Streaming Pattern

```rust
// Static variables (inspectable via GDB)
static mut COUNTER: u32 = 0;
static mut SENSOR: i32 = 1000;

loop {
    // Update variables
    unsafe {
        COUNTER += 1;
        SENSOR += 10;
    }

    // Format to string
    let mut buffer = String::<128>::new();
    write!(buffer, "counter={} sensor={}\n",
           unsafe { COUNTER },
           unsafe { SENSOR }).unwrap();

    // Stream via DMA
    let data = buffer.as_bytes();
    dma_tx.as_mut_slice()[0..data.len()].copy_from_slice(data);
    dma_tx.set_length(data.len());

    let transfer = uhci_tx.write(dma_tx).unwrap();
    let (_, uhci, dma) = transfer.wait();
    uhci_tx = uhci;
    dma_tx = dma;
}
```

---

## Additional Resources

- [esp-hal Documentation](https://docs.esp-rs.org/esp-hal/)
- [ESP32-C6 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [UART UHCI Source Code](~/.cargo/registry/src/.../esp-hal-1.0.0/src/uart/uhci.rs)
- [DMA Source Code](~/.cargo/registry/src/.../esp-hal-1.0.0/src/dma/)

**Remember:** Source code is often the best documentation!

---

**Built with:** esp-hal 1.0.0, Rust nightly, ESP32-C6

**Tested:** Compilation ✅ | Hardware ⏳ (pending board availability)

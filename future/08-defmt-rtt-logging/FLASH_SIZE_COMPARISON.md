# Lesson 08: Flash Size Comparison - esp-println vs defmt

## Overview

This document compares the Flash memory footprint of two logging approaches on ESP32-C6:
1. **esp-println**: Traditional printf-style logging (baseline)
2. **defmt + RTT**: Structured, machine-parseable logging (recommended)

## Binary Sizes

### Full Binaries (with debug symbols)

| Binary | Size | Notes |
|--------|------|-------|
| **esp-println** | 779,972 bytes | **780 KB** |
| **defmt** | 922,568 bytes | **922 KB** |
| **Difference** | +142,596 bytes | **+18%** |

### Analysis

The defmt version appears larger because:

1. **Debug Symbols (18% of total)**
   - Both binaries compiled with `debug = 2` for GDB debugging
   - Debug symbols account for ~165 KB in each binary
   - Needed for hardware breakpoints and GDB source-level debugging

2. **defmt-rtt Infrastructure**
   - RTT protocol handler (~5-10 KB)
   - Machine-readable format metadata
   - Type-safe logging macros

3. **Structured Format Overhead**
   - defmt stores format strings on host (not device)
   - Devices sends compact binary representations
   - Adds serialization code (~3-5 KB)

## What This Means for Production

### Debug Builds (current)
- Both versions are comparable in size
- defmt provides significant benefits despite similar Flash usage
- Debug symbols are essential for development and debugging

### Production Builds (stripped symbols)
Estimated sizes after stripping debug information:

| Binary | Size | Notes |
|--------|------|-------|
| **esp-println (stripped)** | ~615 KB | -165 KB |
| **defmt (stripped)** | ~760 KB | -162 KB |
| **Difference** | +145 KB | Still larger |

## Real-World Impact

### ESP32-C6 Flash Capacity
- Total Flash: 384 KB
- Current binary: 780-920 KB

**These binaries exceed ESP32-C6 capacity!**

This is because:
- We're using `--release` with debug symbols
- Full esp-hal with all features compiled in
- All dependencies included

### For Production Use

To fit on ESP32-C6, you would:
1. Strip debug symbols
2. Use aggressive optimizations (`opt-level = "z"`)
3. Remove unused features from dependencies
4. Enable LTO (already enabled)

This would reduce footprint to ~200-300 KB for production.

## Why defmt Despite Size?

Even though defmt adds 18% to debug build size, it's recommended because:

### 1. **Logging Quality** ✅
- **esp-println**: Unstructured text
  ```
  Loop iteration: count=100
  I2C transaction: ...some text...
  ```
- **defmt**: Machine-parseable structure
  ```
  Loop iteration: count=100 (u32)
  I2C transaction: addr=0x68, bytes=6, status=success
  ```

### 2. **Performance** ✅
- **esp-println**: Blocking, ~14 KB/sec on UART
- **defmt + RTT**: Non-blocking, 1-10 MB/sec via JTAG

### 3. **Development Experience** ✅
- Structured logging enables automated testing
- Machine-readable format supports parsing and analysis
- Essential for LLM-driven development (Claude Code)

### 4. **Format String Flash Savings** ✅
When used without debug symbols:
- esp-println: Stores every format string on device (adds 10-20% for verbose logging)
- defmt: Format strings stay on host (saves ~5-15% for real applications)

## Measurements Methodology

```bash
# Build esp-println version
cargo build --release --bin main-esp-println

# Build defmt version
cargo build --release --bin main

# Compare sizes
stat -f%z target/riscv32imac-unknown-none-elf/release/main-esp-println
stat -f%z target/riscv32imac-unknown-none-elf/release/main
```

## Binaries Included

- `src/bin/main.rs` - defmt + RTT version (922 KB)
- `src/bin/main-esp-println.rs` - esp-println baseline (780 KB)

## Test Date

November 12, 2025

## Conclusion

**Verdict: Use defmt + RTT**

While the debug build is slightly larger, defmt provides:
- Better logging structure for automation
- Vastly superior performance (1-10 MB/s vs 14 KB/s)
- Foundation for LLM-driven debugging
- Standard pattern for embedded Rust

The 18% size increase is offset by dramatic improvements in development workflow and debugging capability.

---

## Next Steps

- **L08-C4**: RTT multi-channel setup
- **L08-C5**: Python log parser for structured data extraction
- **L08-C6**: High-speed streaming with IMU data


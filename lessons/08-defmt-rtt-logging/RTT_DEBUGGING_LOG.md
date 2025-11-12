# RTT Debugging Case Study: ESP32-C6 with probe-rs

**Date:** 2025-11-12
**Issue:** RTT (Real-Time Transfer) logging with defmt-rtt not producing output on ESP32-C6
**Tools:** probe-rs 0.30.0, cargo-embed, ESP32-C6 built-in USB-JTAG (303a:1001)

## Problem Statement

Lesson 08 firmware (defmt + RTT logging) builds and flashes successfully, but RTT logs are not captured by probe-rs. The user reported "this was working yesterday" suggesting a regression or configuration drift.

## Environment

```
Hardware: ESP32-C6 DevKit
JTAG: Built-in USB-JTAG (ESP JTAG 303a:1001:F0:F5:BD:01:88:2C)
USB Port: /dev/cu.usbmodem101
probe-rs: 0.30.0 (git commit: crates.io)
cargo-embed: (part of probe-rs)
Firmware: target/riscv32imac-unknown-none-elf/release/main (922KB)
```

## Systematic Investigation

### Step 1: Verify Hardware Detection ✓ PASS

```bash
probe-rs list
# Result: ESP JTAG 303a:1001:F0:F5:BD:01:88:2C detected
```

**Finding:** Hardware detection works correctly. JTAG probe is recognized.

### Step 2: Verify probe-rs Connection ✓ PASS

```bash
probe-rs info --chip esp32c6 --probe 303a:1001:F0:F5:BD:01:88:2C
# Result: Successfully connected, chip identified
```

**Finding:** probe-rs can establish JTAG connection and identify ESP32-C6.

### Step 3: Verify RTT Control Block in Binary ✓ PASS

```bash
nm target/riscv32imac-unknown-none-elf/release/main | grep -i "segger"
# Result: 40800dbc D _SEGGER_RTT

nm target/riscv32imac-unknown-none-elf/release/main | grep -i "rtt"
# Results:
# 40800dbc D _SEGGER_RTT
# 40801040 b _ZN9defmt_rtt6BUFFER17h1ce2f60d0796dc07E
# 42005b30 t _ZN9defmt_rtt7channel7Channel14blocking_write17h08dc4c6b82fb0f76E
```

**Finding:** RTT control block exists at address 0x40800dbc. defmt-rtt symbols present in binary.

### Step 4: Verify Cargo.toml Configuration ✓ PASS (with note)

```toml
[dependencies]
defmt = "0.3"
defmt-rtt = "0.4"
esp-println = { version = "0.11", features = ["esp32c6"] }  # ⚠ Potential conflict
```

**Finding:** defmt and defmt-rtt correctly configured. Note: esp-println is also present, which may conflict with defmt-rtt.

### Step 5: Verify Firmware Source Code ✓ PASS

```rust
// src/bin/main.rs
use defmt::{info, warn, error, debug};
use defmt_rtt as _;  // ✓ Correctly initializes RTT

defmt::timestamp!("{=u64:us}", { 0 });  // ✓ Timestamp defined

#[main]
fn main() -> ! {
    info!("🚀 Starting Lesson 08: defmt + RTT Structured Logging");
    // ... infinite loop with periodic logging
}
```

**Finding:** Firmware correctly imports and initializes defmt-rtt. Infinite loop ensures firmware runs indefinitely.

### Step 6: Initial probe-rs run Test ❌ FAIL (Silent)

```bash
probe-rs run --chip esp32c6 --probe 303a:1001 target/.../main
# Result:
#      Finished in 1.01s
# Exited by user request
```

**Finding:** probe-rs runs firmware but produces NO RTT output. "Finished in 1.01s" suggests early exit. No error messages shown.

### Step 7: Test with --rtt-scan-memory Flag ❌ FAIL (Silent)

```bash
probe-rs run --chip esp32c6 --probe 303a:1001 --rtt-scan-memory target/.../main
# Result:
#      Finished in 1.00s
# Exited by user request
```

**Finding:** --rtt-scan-memory flag doesn't help. Same silent failure.

### Step 8: Switch to cargo-embed (More Verbose) ❌ FAIL (With Error!)

```bash
cargo embed --release --probe 303a:1001 --bin main
# Result:
#      Finished in 0.83s
#        Error Failed to attach to RTT: Timeout
```

**BREAKTHROUGH:** cargo-embed reveals the actual error: "Failed to attach to RTT: Timeout"

**Root Cause Identified:** probe-rs cannot locate the RTT control block in target memory within the timeout period, even though:
1. Control block exists in binary (0x40800dbc)
2. Firmware runs successfully
3. defmt-rtt is properly initialized

### Step 9: Test Different RTT Timeouts ❌ FAIL

Tested timeouts: 3000ms, 5000ms, 10000ms, 30000ms

```bash
# Embed.toml
[default.rtt]
enabled = true
timeout = 30000  # 30 seconds
```

**Result:**
- 3000ms, 5000ms: "Failed to attach to RTT: Timeout"
- 10000ms, 30000ms: No error, but also no RTT output

**Finding:** Increasing timeout doesn't resolve the issue. probe-rs fundamentally cannot find the RTT control block in ESP32-C6 memory.

## Current Hypotheses

### Hypothesis 1: RTT Control Block Memory Mapping Issue
**Theory:** The RTT control block at 0x40800dbc may not be accessible via JTAG when the firmware is running, or it's not properly mapped to RAM.

**Evidence:**
- Control block exists in binary (verified via `nm`)
- probe-rs times out when scanning memory for RTT
- Address 0x40800dbc is in high memory region (possibly ROM/Flash, not RAM)

**Status:** Needs investigation - check ESP32-C6 memory map to confirm RTT is in accessible RAM region.

### Hypothesis 2: probe-rs ESP32-C6 RTT Support Limitation
**Theory:** probe-rs 0.30.0 has incomplete or broken RTT support for ESP32-C6, particularly with built-in USB-JTAG.

**Evidence:**
- Previous web research (from earlier conversation) found GitHub issues about "Failed to attach to RTT" on ESP32
- ESP32 support in probe-rs is relatively new (2024)
- Built-in USB-JTAG has timing/initialization issues documented

**Status:** Likely cause. Recommendation: Test with OpenOCD instead.

### Hypothesis 3: defmt-rtt + esp-println Conflict
**Theory:** Having both esp-println and defmt-rtt in Cargo.toml creates a conflict in how logging is routed.

**Evidence:**
- Cargo.toml includes both backends
- Only one logging backend should be active at a time
- defmt-rtt uses global state that might conflict with esp-println

**Status:** Possible contributing factor. Recommendation: Remove esp-println and rebuild.

### Hypothesis 4: Firmware Not Actually Initializing RTT
**Theory:** Despite `use defmt_rtt as _;`, the RTT control block isn't being properly initialized at runtime.

**Evidence Against:**
- RTT symbols present in binary
- defmt-rtt is included in dependencies
- `use defmt_rtt as _;` should trigger initialization

**Status:** Unlikely - configuration looks correct.

## Next Steps

### Immediate Testing
1. **Remove esp-println from Cargo.toml** - Eliminate potential conflict
2. **Verify ESP32-C6 memory map** - Confirm 0x40800dbc is in RAM, not ROM
3. **Test with OpenOCD instead of probe-rs** - OpenOCD may have better ESP32-C6 RTT support
4. **Check probe-rs version compatibility** - Test with older/newer versions

### Long-term Solutions
1. **Use esp-println instead of defmt-rtt** - Known to work on ESP32-C6 (trade-off: blocking I/O)
2. **Report probe-rs issue** - If confirmed as probe-rs bug, contribute fix or report
3. **Document workaround** - Update lesson README with current status and alternatives

## Key Findings Summary

| Component | Status | Notes |
|-----------|--------|-------|
| Hardware detection | ✓ PASS | ESP32-C6 USB-JTAG detected correctly |
| probe-rs connection | ✓ PASS | Can connect and identify chip |
| RTT control block | ✓ PASS | Exists at 0x40800dbc in binary |
| defmt-rtt config | ✓ PASS | Properly configured in Cargo.toml |
| Firmware code | ✓ PASS | Correctly uses defmt-rtt |
| RTT attachment | ❌ FAIL | "Failed to attach to RTT: Timeout" |
| RTT log capture | ❌ FAIL | No output captured |

## Tools Comparison

| Tool | RTT Detection | Verbosity | Result |
|------|---------------|-----------|---------|
| `probe-rs run` | Silent fail | Low | No error shown, just exits |
| `probe-rs run --rtt-scan-memory` | Silent fail | Low | No improvement |
| `cargo embed` | Explicit timeout | High | ✓ Shows actual error message |

**Recommendation:** Use `cargo embed` for RTT debugging - it provides much better error reporting than `probe-rs run`.

## Lessons Learned (for next engineer/agent)

1. **cargo-embed gives better errors than probe-rs run** - Always try cargo-embed if probe-rs run fails silently
2. **RTT timeout doesn't mean increase the timeout** - It means the control block isn't found at all
3. **Symbol presence != runtime availability** - Just because `nm` shows _SEGGER_RTT doesn't mean it's accessible via JTAG
4. **ESP32 RTT support is still maturing** - probe-rs ESP32 support is relatively new, expect issues
5. **Check for logging backend conflicts** - Don't mix esp-println and defmt-rtt in the same project
6. **User saying "it worked yesterday" should trigger git diff check** - May have been a dependency update or config change

## Questions Still Outstanding

1. Why does probe-rs fail to find RTT control block when it's clearly in the binary?
2. Is 0x40800dbc in a JTAG-accessible memory region on ESP32-C6?
3. Did probe-rs or esp-hal have a recent breaking change?
4. Would an external JTAG probe (vs built-in USB-JTAG) work better?
5. Does OpenOCD have better ESP32-C6 RTT support than probe-rs?

## Files Referenced

```
/Users/shanemattner/Desktop/esp32-c6-agentic-firmware/lessons/08-defmt-rtt-logging/
├── Cargo.toml
├── src/bin/main.rs
├── target/riscv32imac-unknown-none-elf/release/main
└── Embed.toml (generated during testing)
```

## Temporary Test Scripts Created

- `/tmp/check_symbols.sh` - Check for RTT symbols in binary
- `/tmp/detailed_rtt_test.sh` - Extended RTT capture test
- `/tmp/test_cargo_embed.sh` - cargo-embed configuration tests
- `/tmp/test_rtt_timeout.sh` - RTT timeout variation tests
- `/tmp/test_explicit_rtt_scan.sh` - Explicit control block address tests
- `/tmp/check_rtt_address.sh` - Memory map inspection

---

**Status:** Investigation ongoing. Next: Test with OpenOCD.

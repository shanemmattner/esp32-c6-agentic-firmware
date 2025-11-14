# Lesson 07 Test Specification

**Lesson:** 07 - GDB Debugging with probe-rs and OpenOCD
**Hardware:** ESP32-C6 + MPU9250 IMU + Button + NeoPixel + UART
**Test Duration:** ~3 minutes (quick), ~15 minutes (full)

---

## Hardware Setup

### Required Connections

```
ESP32-C6 Pin      →  Peripheral
─────────────────────────────────
GPIO2             →  MPU9250 SDA
GPIO11            →  MPU9250 SCL
GPIO4             →  JTAG TMS
GPIO5             →  JTAG TDI
GPIO6             →  JTAG TDO
GPIO7             →  JTAG TCK
GPIO9             →  Button (pull-up)
GPIO8             →  NeoPixel Data In
GPIO15            →  UART TX
GPIO23            →  UART RX
GND               →  GND (shared)
3V3               →  MPU9250 VCC
```

### Verification Checklist

- [ ] ESP32-C6 USB-C cable connected
- [ ] JTAG debugger connected (ESP-Prog or compatible)
- [ ] MPU9250 I2C connections verified
- [ ] Button connected with pull-up
- [ ] NeoPixel powered and connected
- [ ] UART adapter connected

---

## Test Modes

### Quick Mode (default)
- Build and flash firmware
- Infrastructure validation (debug symbols, source structure)
- Configuration file verification
- ~3 minutes

### Full Mode
- All quick mode tests
- Interactive debugging tests (probe-rs)
- Register inspection verification
- Memory debugging validation
- ~15 minutes

---

## Automated Tests

These tests run automatically via `/test-lesson 07`:

### Test 1: Build Verification
**Purpose:** Verify firmware compiles with debug symbols
**Command:** `cargo build --release`
**Success Criteria:**
- Exit code 0
- Binary size ~62KB
- No compilation errors (warnings acceptable)

### Test 2: Debug Symbols
**Purpose:** Verify debugging is possible
**Command:** `file target/riscv32imac-unknown-none-elf/release/main | grep "not stripped"`
**Success Criteria:** Binary contains debug information

### Test 3: Source Code Structure
**Purpose:** Verify all required files present
**Command:** Check for required source files
**Success Criteria:**
- `src/bin/main.rs` exists (~386 lines)
- `src/lib.rs` exists (~64 lines)
- `src/mpu9250.rs` exists (~63 lines)
- `src/cli.rs` exists (~145 lines)

### Test 4: GDB Configuration Files
**Purpose:** Verify both probe-rs and GDB workflows supported
**Command:** Check for configuration files
**Success Criteria:**
- `.gdbinit` exists (~74 lines)
- `gdb_helpers.py` exists (~236 lines)
- `openocd.cfg` exists (~17 lines)

### Test 5: Python Helper Script Syntax
**Purpose:** Verify GDB Python helpers are valid
**Command:** `python3 -m py_compile gdb_helpers.py`
**Success Criteria:** No syntax errors

### Test 6: Cargo.toml Debug Configuration
**Purpose:** Verify proper build configuration
**Command:** Check Cargo.toml for [[bin]] and debug settings
**Success Criteria:**
- `[[bin]]` section present (name = "main")
- `debug = 2` in release profile
- No conflicting configurations

### Test 7: Flash Firmware
**Purpose:** Verify firmware can be flashed to hardware
**Command:** `espflash flash --port $USB_CDC_PORT target/.../main`
**Success Criteria:**
- Flash succeeds
- Firmware size 62,016 bytes
- No "exclusive access" errors

### Test 8: Boot Verification
**Purpose:** Verify firmware boots and initializes CLI
**Method:** Read serial output for 2 seconds after reset
**Success Criteria:**
- Boot messages appear
- CLI prompt visible
- No panic/crash messages

**Expected Boot Output:**
```
ESP32-C6 GDB Debugging Demo
===========================

Initializing peripherals...
✓ I2C initialized
✓ MPU9250 found (WHO_AM_I: 0x71)
✓ GPIO initialized
✓ NeoPixel initialized
✓ UART initialized

Type 'help' for available commands.
>
```

---

## Interactive Tests (Manual)

⚠️ **These tests require manual execution in an interactive terminal**

### Interactive Test 1: probe-rs Attach
**Purpose:** Verify probe-rs can attach to running firmware

**Setup:**
```bash
source /tmp/test_env.sh
cd "$LESSON_DIR"
```

**Commands:**
```bash
# Start probe-rs interactive session
probe-rs attach --chip esp32c6 --probe 303a:1001:F0:F5:BD:01:88:2C target/riscv32imac-unknown-none-elf/release/main
```

**Expected Output:**
```
      probe-rs v0.X.Y
Attaching to chip...
✓ Connected to ESP32-C6
>
```

**Success Criteria:**
- Attaches without errors
- Interactive prompt appears
- Can enter commands

### Interactive Test 2: Peripheral Register Inspection
**Purpose:** Verify register reads work

**Commands:**
```
> read32 0x60013004   # I2C STATUS register
> read32 0x6000403C   # GPIO IN register
```

**Expected Output:**
```
> read32 0x60013004
0x60013004: 0x00000000

> read32 0x6000403C
0x6000403C: 0x00000200
```

**Success Criteria:**
- Commands execute without errors
- Returns valid 32-bit hex values
- I2C and GPIO registers accessible

### Interactive Test 3: Memory Inspection
**Purpose:** Verify memory reads work

**Commands:**
```
> read 0x3FC88000 64  # Read 64 bytes from RAM
```

**Expected Output:**
```
> read 0x3FC88000 64
0x3FC88000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f
0x3FC88010: 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f
...
```

**Success Criteria:**
- Memory dumps in hex format
- 64 bytes displayed
- Addresses are correct

### Interactive Test 4: Breakpoint at main
**Purpose:** Verify breakpoints work

**Commands:**
```
> break main
> reset
> continue
```

**Expected Output:**
```
> break main
✓ Breakpoint set at main (0x42000XXX)

> reset
✓ Device reset

> continue
✓ Halted at breakpoint: main (src/bin/main.rs:XX)
```

**Success Criteria:**
- Breakpoint sets successfully
- Reset triggers breakpoint
- Execution halts at main()

### Interactive Test 5: Function Breakpoint
**Purpose:** Verify function breakpoints work

**Commands:**
```
> break handle_command
> continue
# In serial console, type: help
```

**Expected Output:**
```
> break handle_command
✓ Breakpoint set at handle_command

> continue
Running...
✓ Halted at breakpoint: handle_command (src/cli.rs:XX)
```

**Success Criteria:**
- Breakpoint sets on function name
- Triggered by CLI command
- Shows correct source location

### Interactive Test 6: Call Stack Analysis
**Purpose:** Verify backtrace works

**Commands:**
```
> backtrace
```

**Expected Output:**
```
> backtrace
#0  handle_command at src/cli.rs:45
#1  main at src/bin/main.rs:123
#2  0x42000xxx in _start
```

**Success Criteria:**
- Shows function call hierarchy
- Includes source file and line numbers
- Stack is reasonable depth

---

## Expected Outputs

### Serial Console Output

After flashing, the serial console should show the CLI prompt. Test commands:

```
> help
Available commands:
  read <addr>      - Read I2C register (hex)
  write <addr> <val> - Write I2C register (hex)
  accel            - Read accelerometer
  gyro             - Read gyroscope
  temp             - Read temperature
  button           - Read button state
  led <r> <g> <b>  - Set NeoPixel color
  help             - Show this help

> accel
Accel: X=-0.05g, Y=0.02g, Z=1.00g

> led 255 0 0
✓ NeoPixel set to red
```

### LED/Visual Indicators

- NeoPixel should change color with `led` command
- No color on boot (default: off)

---

## Troubleshooting

### Issue: Firmware won't flash
**Symptoms:** espflash fails with "exclusive access" error
**Cause:** probe-rs or openocd still attached
**Solution:**
```bash
pkill -f probe-rs
pkill -f openocd
# Retry flash
```

### Issue: No serial output after boot
**Symptoms:** Blank serial console
**Cause:** Wrong baud rate or USB cable
**Solution:**
- Verify baud rate is 115200
- Try different USB cable (must support data)
- Check USB port: `ls /dev/cu.usbmodem*`

### Issue: probe-rs attach fails
**Symptoms:** "Could not find probe" or "exclusive access"
**Cause:** JTAG not connected or another tool using probe
**Solution:**
```bash
# Check probe is detected
probe-rs list

# Kill conflicting processes
pkill -f openocd

# Retry attach
```

### Issue: Breakpoint not hit
**Symptoms:** Firmware runs past breakpoint
**Cause:** Debug symbols missing or wrong function name
**Solution:**
- Verify debug symbols: `file target/.../main | grep "not stripped"`
- Try simpler breakpoint: `break main`
- Rebuild with `cargo clean && cargo build --release`

### Issue: MPU9250 WHO_AM_I fails
**Symptoms:** Boot shows "MPU9250 not found"
**Cause:** I2C wiring or address wrong
**Solution:**
- Verify SDA on GPIO2, SCL on GPIO11
- Check I2C address (0x68 or 0x69)
- Verify MPU9250 powered (3.3V)

### Issue: Device stuck in download mode
**Symptoms:** Boot messages show "waiting for download"
**Cause:** Incorrect reset sequence or hardware issue
**Solution:**
- Unplug/replug USB cable
- Hold BOOT button while resetting
- Check DTR/RTS lines on USB adapter

---

## Performance Benchmarks (Full Mode)

### Timing Measurements
- Boot time: ~50ms
- I2C initialization: ~10ms
- CLI command latency: <5ms

### Memory Usage
- Stack usage: ~4KB
- Static RAM: ~12KB
- Total binary: ~62KB

### I2C Performance
- I2C read latency: ~2ms
- MPU9250 sample rate: up to 500Hz

---

## Notes

- This lesson demonstrates both probe-rs and GDB+OpenOCD workflows
- probe-rs is recommended (faster, easier setup)
- GDB workflow requires OpenOCD installation
- Python GDB helpers provide custom commands for register inspection
- Expected completion time: 3 minutes for basic validation, 15 minutes for full interactive testing
- Known issues: Compiler warning about lifetime elision in cli.rs (cosmetic only)

---

**Last Updated:** 2025-11-12
**Verified On:** probe-rs 0.24+, ESP32-C6 rev 0.1, macOS

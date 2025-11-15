# Hardware Testing Command

**Usage:** `/hardware-test [lesson-path]`

**Description:** Test ESP32-C6 firmware on actual hardware using probe-rs (no TTY required)

## What This Command Does

1. Detects ESP32-C6 via probe-rs (USB-JTAG)
2. Builds the firmware
3. Flashes using probe-rs
4. Tests GPIO control via direct register writes

## Critical Information

### Port Detection

**The ESP32-C6 appears as `/dev/cu.usbmodem1101` (or similar).**

**IMPORTANT:** If you don't see this port, DON'T waste time debugging USB:
- Check if `probe-rs list` shows the device
- If probe-rs sees it, the hardware is working
- espflash requires TTY for interactive port selection (won't work in automation)

### Testing Strategy

**Use probe-rs for everything** - it works without TTY and directly controls hardware.

#### Option 1: Flash and Run (Recommended)
```bash
probe-rs run --chip esp32c6 target/riscv32imac-unknown-none-elf/release/main
```

#### Option 2: Direct GPIO Control (No Firmware Needed!)
```bash
# Enable GPIO12 as output
probe-rs write b32 --chip esp32c6 0x60091024 0x1000

# LED ON
probe-rs write b32 --chip esp32c6 0x60091008 0x1000

# LED OFF
probe-rs write b32 --chip esp32c6 0x6009100C 0x1000
```

## Implementation

When executing this command:

1. **Detect device:**
```bash
probe-rs list  # Should show "ESP JTAG -- 303a:1001..."
```

2. **Build firmware:**
```bash
cd [lesson-path]
cargo build --release
```

3. **Flash and test:**
```bash
# Flash firmware
probe-rs run --chip esp32c6 target/riscv32imac-unknown-none-elf/release/main

# OR test GPIO directly without firmware
probe-rs write b32 --chip esp32c6 0x60091024 0x1000  # Enable GPIO12
probe-rs write b32 --chip esp32c6 0x60091008 0x1000  # LED ON
sleep 1
probe-rs write b32 --chip esp32c6 0x6009100C 0x1000  # LED OFF
```

4. **Report results:**
- Device detected: ✅/❌
- Firmware built: ✅/❌
- Flashed successfully: ✅/❌
- GPIO control verified: ✅/❌

## DO NOT

❌ Try to automate espflash port selection (requires TTY)
❌ Waste time with `ls /dev/cu.*` if probe-rs works
❌ Use UART adapters for flashing (use built-in USB-JTAG)
❌ Assume USB CDC serial output will work (device may stay in bootloader mode)

## If Testing Fails

1. Run `probe-rs list` - if this works, hardware is fine
2. Have the user run `cargo run --release` manually in their terminal
3. Use probe-rs GPIO commands to verify LED works
4. Check git history - branch `lesson-01` has proven working GDB-only firmware

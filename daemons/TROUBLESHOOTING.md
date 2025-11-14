# ESP32-C6 Daemon System Troubleshooting

## Current Hardware Issue: Firmware Not Running After JTAG Flash

### Symptoms
- ✅ Firmware compiles successfully
- ✅ probe-rs flashes without errors
- ❌ No UART output on any of the 3 UART ports
- ❌ LED on GPIO8 not blinking (500ms interval expected)
- ❌ Board likely stuck in download mode (boot:0x75)

### Tested Configurations

**UART Ports Tested:**
- `/dev/cu.usbserial-111300` - No data
- `/dev/cu.usbserial-111301` - No data
- `/dev/cu.usbserial-FT58PFX4` - No data

**Expected Output:**
```
BOOT|version=speed_test_1.0.0|chip=ESP32-C6
STATUS|baudrate=115200|packet_size=64
READY
STATS|ts=1000|seq=100|packets=100|throughput=6400 bytes/s
```

**Firmware Configuration:**
- GPIO8: LED (should blink every 500ms)
- GPIO15: UART TX
- GPIO23: UART RX
- Baudrate: 115200

### Diagnostic Steps

#### 1. Check LED Blink
**Expected:** GPIO8 LED blinks every 500ms
**Actual:** (User to observe)
**Meaning:** If LED is blinking, firmware IS running but UART isn't working. If no blink, firmware isn't executing.

#### 2. Check Boot Mode
After flashing via JTAG, the ESP32-C6 may need a proper reset sequence to exit download mode and run the application.

**Try this reset sequence:**
```bash
# Flash firmware
probe-rs run --chip esp32c6 --probe 303a:1001 target/.../uart_speed_test

# Power cycle the board (disconnect and reconnect power)
# OR
# Press the RST button on the board
```

#### 3. Check JTAG Flash Output
The probe-rs output should show successful flash:
```
     Erasing ✔ [00:00:01] [########################################] 64.00 KiB/64.00 KiB @ 32.00 KiB/s (eta 0s)
 Programming ✔ [00:00:01] [########################################] 64.00 KiB/64.00 KiB @ 32.00 KiB/s (eta 0s)
    Finished in 2.0s
```

If flash fails or shows errors, the probe connection may be unstable (loose jumper wires).

#### 4. Alternative: Flash via USB Bootloader
If JTAG flash isn't working, try flashing directly via the ESP32-C6's USB port:

```bash
# Put ESP32-C6 into bootloader mode:
# 1. Hold BOOT button
# 2. Press RST button
# 3. Release RST button
# 4. Release BOOT button

# Flash using espflash
espflash flash --port /dev/cu.usbmodem2101 --monitor target/riscv32imac-unknown-none-elf/release/uart_speed_test

# Should see boot output and firmware start running
```

### Known Working Configurations

From previous lessons, we know:
- ✅ GPIO15/23 UART worked in Lesson 07 (defmt-rtt-logging)
- ✅ JTAG worked for flashing in Lesson 07
- ✅ Same hardware, same wiring

**This suggests the issue is boot configuration, not hardware failure.**

### Possible Root Causes

1. **Boot Strapping Pins:** ESP32-C6 boot mode is determined by GPIO0/GPIO9 strapping at reset
   - Download mode: GPIO0=LOW, GPIO9=HIGH (boot:0x75)
   - Application mode: GPIO0=HIGH, GPIO9=LOW (boot:0x13)

2. **JTAG Flash Doesn't Reset Properly:** probe-rs may flash successfully but leave the chip in a state where it doesn't auto-boot

3. **Loose Connections:** If jumper wire on GPIO15 or GPIO23 came loose, UART won't work

### Verification Checklist

- [ ] LED on GPIO8 is blinking (confirms firmware running)
- [ ] UART TX (GPIO15) connected to FTDI RX
- [ ] UART RX (GPIO23) connected to FTDI TX
- [ ] UART GND connected to FTDI GND
- [ ] FTDI shows up as `/dev/cu.usbserial-*` device
- [ ] Board power LED is on
- [ ] Tried manual RST button press after flash
- [ ] Tried power cycle after flash

### Next Steps

**If LED is blinking:**
1. Issue is UART wiring or configuration
2. Check GPIO15/23 connections
3. Try different FTDI adapter
4. Verify baudrate (115200)

**If LED is NOT blinking:**
1. Firmware not executing
2. Try USB bootloader flash instead of JTAG
3. Check boot strapping pins (GPIO0/GPIO9)
4. Try probe-rs with explicit reset: `probe-rs reset --chip esp32c6`

### Manual Test Without Daemon

```bash
# Simple serial monitor to see raw output
screen /dev/cu.usbserial-111300 115200

# OR
python3 -m serial.tools.miniterm /dev/cu.usbserial-111300 115200

# Should see:
# BOOT|version=speed_test_1.0.0|chip=ESP32-C6
# STATUS|baudrate=115200|packet_size=64
# READY
# STATS|ts=1000|...
```

Press Ctrl+A then K to exit screen.

### Related Issues

This is the same boot mode problem documented in the conversation history:
- Board boots into download mode after JTAG flash
- USB CDC not accessible
- Power cycling doesn't fix
- Only occurs with this specific board/setup

**Recommendation:** Use USB bootloader flash method (espflash) as primary method until JTAG boot issue is resolved.

---

**Last Updated:** 2025-11-12
**Status:** Investigating - firmware compiles but doesn't execute after JTAG flash

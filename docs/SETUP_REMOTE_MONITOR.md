# Remote Development Setup
## Mac → SSH → RPi → ESP32-C6

Simple step-by-step guide to get your remote development working.

---

## Step 1: Find the Serial Port on RPi

**On your Mac**, SSH into the RPi and check for connected devices:

```bash
ssh pi@raspberrypi.local
# or ssh pi@<your-rpi-ip>
```

Once logged in on the RPi, check for the ESP32:

```bash
# List all USB devices
ls -la /dev/tty* /dev/cu* 2>/dev/null | grep -E "(USB|ACM|ttyUSB)"

# Or more detailed
lsusb
```

**Look for something like:**
- `/dev/ttyUSB0` (most common)
- `/dev/ttyACM0` (also common)
- `/dev/cu.usbserial-*` (if using macOS-like naming)

**Note this port** - you'll use it in the next steps.

---

## Step 2: Create Python Monitor on RPi

Create this file on your **Mac**, then copy it to the RPi:

**File: `scripts/monitor.py`**

```python
#!/usr/bin/env python3
"""
Simple serial monitor that works on RPi and streams to stdout.
Usage: python3 monitor.py --port /dev/ttyUSB0 --baud 115200
"""

import serial
import sys
import argparse
from datetime import datetime

def main():
    parser = argparse.ArgumentParser(description='Serial port monitor')
    parser.add_argument('--port', default='/dev/ttyUSB0', help='Serial port')
    parser.add_argument('--baud', type=int, default=115200, help='Baud rate')
    args = parser.parse_args()

    try:
        ser = serial.Serial(args.port, args.baud, timeout=1)
        print(f"✓ Connected to {args.port} at {args.baud} baud")
        print("=" * 60)

        while True:
            if ser.in_waiting:
                line = ser.readline().decode('utf-8', errors='ignore').rstrip()
                if line:
                    print(line)
                    sys.stdout.flush()

    except KeyboardInterrupt:
        print("\n" + "=" * 60)
        print("✓ Monitor closed")
    except Exception as e:
        print(f"✗ Error: {e}")
        sys.exit(1)

if __name__ == '__main__':
    main()
```

**Copy to RPi:**
```bash
scp scripts/monitor.py pi@raspberrypi.local:/home/pi/monitor.py
chmod +x /home/pi/monitor.py  # Make executable
```

---

## Step 3: Create SSH Helper on Mac

Create this helper script on your **Mac**:

**File: `scripts/remote-build-flash.sh`**

```bash
#!/bin/bash
# Build locally, flash via RPi, monitor output

set -e

# Configuration
RPI_HOST="${1:-pi@raspberrypi.local}"
LESSON="${2:-01-blinky}"
RPI_PORT="${3:-/dev/ttyUSB0}"
RPI_BAUD="${4:-115200}"

echo "📍 Remote Development Setup"
echo "  RPi Host:   $RPI_HOST"
echo "  Lesson:     $LESSON"
echo "  Serial Port: $RPI_PORT"
echo ""

# Step 1: Build locally
echo "🔨 Building lesson: $LESSON"
cd lessons/$LESSON
cargo build --release || {
    echo "✗ Build failed"
    exit 1
}
cd ../..

BINARY="lessons/$LESSON/target/riscv32imac-unknown-none-elf/release/$LESSON"

# Step 2: Copy binary to RPi
echo "📦 Copying binary to RPi..."
scp "$BINARY" "$RPI_HOST:/tmp/$LESSON"

# Step 3: Flash on RPi
echo "⚡ Flashing on RPi..."
ssh "$RPI_HOST" "espflash flash /tmp/$LESSON --port $RPI_PORT" || {
    echo "✗ Flash failed"
    exit 1
}

# Step 4: Monitor output
echo "👀 Monitoring serial output (Ctrl+C to stop)..."
echo "=" * 60
ssh "$RPI_HOST" "python3 /home/pi/monitor.py --port $RPI_PORT --baud $RPI_BAUD"
```

**Install on Mac:**
```bash
chmod +x scripts/remote-build-flash.sh
```

---

## Step 4: Test It!

### First time setup - find the serial port:
```bash
ssh pi@raspberrypi.local
ls -la /dev/tty* | grep USB
# Note the port, exit SSH
exit
```

### Build, flash, and monitor:
```bash
./scripts/remote-build-flash.sh pi@raspberrypi.local 01-blinky /dev/ttyUSB0
```

**Expected output:**
```
📍 Remote Development Setup
  RPi Host:   pi@raspberrypi.local
  Lesson:     01-blinky
  Serial Port: /dev/ttyUSB0

🔨 Building lesson: 01-blinky
   Compiling blinky v0.1.0
    Finished release [optimized + debuginfo] target(s) in 1.36s

📦 Copying binary to RPi...
blinky                          100%  35KB    2.1MB/s   00:00

⚡ Flashing on RPi...
[INFO] Connecting...
[INFO] Flashing has completed!

👀 Monitoring serial output (Ctrl+C to stop)
============================================================
🚀 Starting Blinky (Lesson 01)
✓ HAL initialized
✓ GPIO13 configured as output
💡 Entering blink loop...
```

---

## Step 5: Make It Even Easier (Optional)

Add to your Mac's `~/.bash_profile` or `~/.zshrc`:

```bash
# ESP32-C6 helpers
alias esp-build="cd ~/Desktop/esp32-c6-agentic-firmware"
alias esp-flash='./scripts/remote-build-flash.sh pi@raspberrypi.local'
```

Then you can just do:
```bash
esp-flash 01-blinky /dev/ttyUSB0
```

---

## 🐛 Troubleshooting

### "Connection refused"
```bash
# Make sure RPi is on and accessible
ping raspberrypi.local
# or ping <rpi-ip>
```

### "Permission denied (publickey)"
```bash
# Set up SSH key if not already done
ssh-copy-id pi@raspberrypi.local
```

### "Serial port not found"
```bash
# SSH to RPi and check
ssh pi@raspberrypi.local
lsusb
ls -la /dev/tty* | grep -i usb
```

### "No serial output"
Check:
1. ESP32 is powered (check LED on board)
2. Port matches what's on RPi (not your Mac port!)
3. Try setting `ESP_LOG_LEVEL=INFO` on RPi before flashing

```bash
ssh pi@raspberrypi.local "export ESP_LOG_LEVEL=INFO && espflash flash ..."
```

---

## 📋 Quick Reference

| Task | Command |
|------|---------|
| Find serial port | `ssh pi@raspberrypi.local && ls -la /dev/tty*` |
| Build + Flash + Monitor | `./scripts/remote-build-flash.sh pi@raspberrypi.local 01-blinky /dev/ttyUSB0` |
| Just monitor | `ssh pi@raspberrypi.local "python3 monitor.py --port /dev/ttyUSB0"` |
| SSH into RPi | `ssh pi@raspberrypi.local` |

---

## 🎯 Next Steps

1. **Find your serial port** on the RPi
2. **Create `scripts/monitor.py`** on your Mac
3. **Copy it to RPi**
4. **Create `scripts/remote-build-flash.sh`** on your Mac
5. **Run**: `./scripts/remote-build-flash.sh pi@raspberrypi.local 01-blinky /dev/ttyUSB0`
6. **Watch your LED blink!** 🎉

---

## 💡 Why This Works

```
Mac (your laptop)
  ├─ Build (fast, local)
  ├─ Copy binary (small, ~35KB)
  └─ SSH commands
       │
       └──> RPi
             ├─ Flash (has the hardware)
             ├─ Monitor serial (USB is local to RPi)
             └─ Stream output back to Mac
```

No USB port needed on Mac = perfect for remote dev!

---

**Once this is working, you're ready to:**
- ✅ Create Lesson 02 (Button Input)
- ✅ Add more peripherals
- ✅ Build the UART command channel later

Let me know what serial port you find and we'll get it fully working! 🚀

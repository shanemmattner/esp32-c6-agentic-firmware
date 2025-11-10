# Quick Start: Remote Development (3 Steps)

Get your ESP32-C6 + RPi setup working in ~5 minutes.

---

## Step 1️⃣: Find Serial Port on RPi (2 min)

**On your Mac:**
```bash
ssh pi@raspberrypi.local
```

**Once logged into RPi:**
```bash
ls -la /dev/tty* | grep -E "(USB|ACM)"
```

**Look for output like:**
```
crw-rw---- 1 root dialout 188,   0 Nov 10 02:20 /dev/ttyUSB0
```

**Note down:** `__________` (save this!)

**Exit:**
```bash
exit
```

---

## Step 2️⃣: Copy Monitor Script to RPi (1 min)

**On your Mac:**
```bash
scp scripts/monitor.py pi@raspberrypi.local:/home/pi/monitor.py
```

**Verify it worked:**
```bash
ssh pi@raspberrypi.local "ls -lah /home/pi/monitor.py"
```

---

## Step 3️⃣: Build, Flash, Monitor (2 min)

**On your Mac:**
```bash
./scripts/remote-build-flash.sh pi@raspberrypi.local 01-blinky /dev/ttyUSB0
```

**Replace `/dev/ttyUSB0` with your actual port from Step 1**

**Expected output:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Remote Build → Flash → Monitor
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
RPi Host:      pi@raspberrypi.local
Lesson:        01-blinky
Serial Port:   /dev/ttyUSB0
Baud Rate:     115200

🔨 Building lesson locally...
  Binary size: 35.2 KB
📦 Copying binary to RPi...
  Copied to /tmp/01-blinky
⚡ Flashing on RPi...
  ✓ Flash successful
👀 Monitoring serial output...
  (Press Ctrl+C to stop)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚀 Starting Blinky (Lesson 01)
✓ HAL initialized
✓ GPIO13 configured as output
💡 Entering blink loop...
```

**✨ If you see that output and your GPIO13 LED is blinking, you're done!**

Press `Ctrl+C` to stop monitoring.

---

## 🎯 What Comes Next

Once this works:

1. **Lesson 02**: Add button input on GPIO9
2. **Lesson 03**: Build a simple state machine
3. **Lesson 04**: Add async/await with Embassy
4. **Later**: Add UART for commands

---

## 🐛 If It Doesn't Work

### Can't SSH to RPi?
```bash
# Check if RPi is online
ping raspberrypi.local
# or use IP: ping 192.168.x.x
```

### Serial port not found on RPi?
```bash
ssh pi@raspberrypi.local
# Check all serial devices
ls -la /dev/tty* /dev/cu*
# Check USB specifically
lsusb
```

### "Connection refused" on build/flash?
```bash
# Make sure SSH key is set up
ssh-copy-id pi@raspberrypi.local
# Test it works
ssh pi@raspberrypi.local "echo hello"
```

### Flash failed?
```bash
# Check port on RPi (must be /dev/ttyUSB0 or similar, not your Mac port)
ssh pi@raspberrypi.local "ls -la /dev/tty* | grep USB"
# Then retry with correct port
./scripts/remote-build-flash.sh pi@raspberrypi.local 01-blinky /dev/ttyUSB0
```

---

## 💡 Tips

**Save typing with shell aliases** (optional):

Add to `~/.zshrc` or `~/.bash_profile`:
```bash
alias esp-dev='cd ~/Desktop/esp32-c6-agentic-firmware'
alias esp-flash='./scripts/remote-build-flash.sh pi@raspberrypi.local'
```

Then you can do:
```bash
esp-dev
esp-flash 01-blinky /dev/ttyUSB0
```

---

## 📋 Summary

```bash
# One-time setup
ssh pi@raspberrypi.local "ls -la /dev/tty*"  # Find port
scp scripts/monitor.py pi@raspberrypi.local:/home/pi/

# Every time you want to test
./scripts/remote-build-flash.sh pi@raspberrypi.local 01-blinky /dev/ttyUSB0
```

That's it! 🚀

---

**Next:** Reply with what port you found (e.g., `/dev/ttyUSB0`), and let me know if you hit any issues!

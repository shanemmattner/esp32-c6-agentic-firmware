# Remote Development Workflow
## ESP32-C6 via Raspberry Pi

This document describes the remote embedded development setup used in this repository.

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Development Environment                      │
│                                                                   │
│  ┌──────────────┐         ┌──────────────┐         ┌──────────┐│
│  │   Laptop     │  SSH/   │ Raspberry Pi │   USB   │ ESP32-C6 ││
│  │              │  Network│              │  Serial │          ││
│  │ - IDE/Editor │ ──────> │ - espflash   │ ──────> │ - GPIO8  ││
│  │ - Claude Code│         │ - Rust       │         │ - I2C    ││
│  │ - Git        │         │ - Build tools│         │ - SPI    ││
│  │              │         │              │         │ - UART   ││
│  └──────────────┘         └──────────────┘         └──────────┘│
│         │                         │                      │       │
│         │                         │                      │       │
│         └─────────Code────────────┘                      │       │
│                                                           │       │
│                                   ┌───────────────────────┘       │
│                                   │                               │
│                                   ▼                               │
│                          ┌─────────────────┐                      │
│                          │  Peripherals    │                      │
│                          │  - I2C Sensors  │                      │
│                          │  - SPI Display  │                      │
│                          │  - UART Module  │                      │
│                          │  - GPIO Devices │                      │
│                          └─────────────────┘                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🎯 Why Remote Development?

### Benefits

**1. Persistent Hardware Setup**
- ESP32-C6 stays connected to all peripherals
- No need to disconnect/reconnect for each test
- Cables and wiring remain stable
- Production-like environment

**2. Rapid Iteration**
- Code on laptop with full dev tools
- Build and flash without physical access
- Monitor serial output remotely
- Quick feedback loop

**3. Multiple Peripheral Testing**
- Connect many sensors/devices at once
- Test driver interactions
- Real-world system integration
- No workspace clutter on laptop

**4. Scalable Development**
- Can manage multiple boards
- Team members can access same hardware
- Remote debugging capabilities
- Continuous integration potential

**5. Real-World Simulation**
- Mimics production deployment
- Network-based monitoring
- Remote firmware updates
- Cloud integration testing

---

## 🛠️ Setup Guide

### Raspberry Pi Setup

**1. Install Rust Toolchain**
```bash
# On Raspberry Pi
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add RISC-V target
rustup target add riscv32imac-unknown-none-elf
```

**2. Install ESP Tools**
```bash
# Install espflash
cargo install espflash

# Install cargo-espflash for convenience
cargo install cargo-espflash
```

**3. USB Permissions**
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER

# Create udev rule for ESP32
echo 'SUBSYSTEM=="tty", ATTRS{idVendor}=="303a", ATTRS{idProduct}=="1001", MODE="0666"' | \
  sudo tee /etc/udev/rules.d/99-esp32.rules

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Log out and back in for group changes to take effect
```

**4. Verify ESP32-C6 Connection**
```bash
# List USB serial devices
ls -la /dev/ttyUSB* /dev/ttyACM*

# Should see something like /dev/ttyUSB0 or /dev/ttyACM0

# Test with espflash
espflash board-info
```

### Laptop Setup

**1. Configure SSH to Raspberry Pi**
```bash
# Add to ~/.ssh/config
Host rpi-esp32
    HostName <raspberry-pi-ip>
    User pi
    ForwardAgent yes
    Compression yes
```

**2. Test Connection**
```bash
ssh rpi-esp32

# On RPi, verify:
ls /dev/ttyUSB* /dev/ttyACM*
espflash board-info
```

---

## 🚀 Development Workflow

### Method 1: Direct SSH + Build on Pi

**Step 1: SSH to Raspberry Pi**
```bash
ssh rpi-esp32
cd ~/esp32-c6-agentic-firmware/lessons/01-blinky
```

**Step 2: Build and Flash**
```bash
cargo build --release
cargo run --release  # Builds and flashes
```

**Step 3: Monitor Output**
```bash
espflash monitor /dev/ttyUSB0
```

### Method 2: Remote Build from Laptop

**Using `rsync` and SSH**

```bash
#!/bin/bash
# deploy.sh - Run from laptop

# Sync code to Pi
rsync -av --exclude target --exclude .git \
  ~/Desktop/esp32-c6-agentic-firmware/ \
  rpi-esp32:~/esp32-c6-agentic-firmware/

# Build and flash on Pi
ssh rpi-esp32 "cd ~/esp32-c6-agentic-firmware/lessons/01-blinky && \
  cargo build --release && \
  cargo run --release"

# Monitor serial output
ssh rpi-esp32 "espflash monitor /dev/ttyUSB0"
```

### Method 3: VS Code Remote Development

**Install Extensions:**
- Remote - SSH
- rust-analyzer
- CodeLLDB (for debugging)

**Setup:**
1. Connect to RPi via Remote-SSH
2. Open project folder on Pi
3. Build and flash directly from VS Code
4. Use integrated terminal for monitoring

### Method 4: Claude Code Remote

**Workflow:**
1. Use Claude Code on laptop for development
2. Commit changes to git
3. Pull on RPi or use rsync
4. Build and flash from Pi
5. Claude Code reads serial output for verification

---

## 📋 Peripheral Connection Plan

### Currently Connected
- ✅ **DHT11 Sensor** - GPIO12 (Temperature & Humidity)

### Phase 1: Basic Peripherals
- **GPIO**: LEDs, buttons, relays
- **DHT11/DHT22**: ✅ Digital temp/humidity sensor (GPIO12)
- **I2C**: Temperature sensors (BME280, SHT31)
- **SPI**: OLED display (SSD1306)
- **UART**: GPS module

### Phase 2: Advanced Peripherals
- **I2C**: IMU (MPU6050), RTC (DS3231)
- **SPI**: SD card reader, LoRa module
- **ADC**: Analog sensors
- **PWM**: Servo motors, LED dimming

### Phase 3: Communication
- **WiFi**: MQTT client, HTTP server
- **Bluetooth**: BLE peripheral
- **CAN**: Automotive sensors
- **I2S**: Audio codec

---

## 🔧 Troubleshooting

### SSH Connection Issues
```bash
# Test connectivity
ping <raspberry-pi-ip>

# Check SSH service
ssh -v rpi-esp32

# Regenerate SSH keys if needed
ssh-keygen -R <raspberry-pi-ip>
```

### USB Serial Issues
```bash
# On Raspberry Pi
# Check if device is detected
lsusb | grep -i esp

# Check dmesg for USB events
dmesg | tail -n 20

# Try different USB port
# Check cable quality (some cables are power-only)
```

### Build Issues
```bash
# Ensure Rust is up to date
rustup update stable

# Clean build
cargo clean
cargo build --release
```

### Flash Issues
```bash
# Hold BOOT button while connecting
# Reset ESP32-C6
# Try manual port specification
espflash flash --port /dev/ttyUSB0 target/riscv32imac-unknown-none-elf/release/blinky
```

---

## 📊 Performance Notes

### Build Times
- **On Pi 4 (4GB)**: ~30 seconds for release build
- **On Pi 5 (8GB)**: ~15 seconds for release build
- **Laptop (M1/M2)**: ~8 seconds (but would need to transfer)

**Conclusion**: Building on Pi is acceptable for rapid iteration

### Network Latency
- **LAN**: <5ms typical
- **SSH overhead**: Minimal
- **File sync**: ~1 second for code changes

### Flash Time
- **Constant**: ~10 seconds regardless of build location
- **Bottleneck**: USB serial communication, not network

---

## 🎓 Best Practices

### 1. Version Control
```bash
# Always commit before testing
git add .
git commit -m "Test: description"

# Pull on Pi
ssh rpi-esp32 "cd ~/esp32-c6-agentic-firmware && git pull"
```

### 2. Logging
```rust
// Use comprehensive logging for remote debugging
info!("🚀 Initializing peripheral");
debug!("Register value: 0x{:02X}", value);
error!("Failed to initialize: {:?}", err);
```

### 3. Serial Monitoring
```bash
# Use tmux for persistent monitoring
ssh rpi-esp32
tmux new -s esp32
espflash monitor /dev/ttyUSB0

# Detach with Ctrl+B, D
# Reattach with: tmux attach -t esp32
```

### 4. Automated Testing
```bash
# Create test script
#!/bin/bash
# test-peripheral.sh

cargo build --release
cargo run --release &
FLASH_PID=$!

sleep 5  # Wait for flash to complete

# Capture serial output
timeout 10 espflash monitor /dev/ttyUSB0 | tee test-output.log

# Parse results
grep "✓ All tests passed" test-output.log
```

---

## 🚀 Next Steps

### Immediate
- [ ] Connect first I2C sensor (BME280)
- [ ] Test remote flashing workflow
- [ ] Set up tmux for persistent monitoring
- [ ] Create deployment script

### Future
- [ ] Add multiple boards for parallel testing
- [ ] Set up CI/CD with GitHub Actions
- [ ] Remote debugging with probe-rs
- [ ] Web-based serial monitor

---

## 📚 Resources

- [Raspberry Pi GPIO Pinout](https://pinout.xyz/)
- [ESP32-C6 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-c6_datasheet_en.pdf)
- [Remote Development with VS Code](https://code.visualstudio.com/docs/remote/ssh)
- [tmux Cheat Sheet](https://tmuxcheatsheet.com/)

---

**This remote development setup enables rapid, iterative driver development while maintaining a stable hardware configuration.**

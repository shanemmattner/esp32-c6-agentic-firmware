# Remote Development Architecture
## With Raspberry Pi as Hardware Proxy

```
┌─────────────────────────────────────────────────────────────┐
│ LAPTOP (Your Development Machine)                            │
│                                                               │
│  • Code editor (Claude Code)                                │
│  • Python test runner                                        │
│  • Log analyzer                                              │
│  • Monitor/dashboard                                         │
└─────────────────────────────────────────────────────────────┘
                         │
                    SSH Connection
                         │
         ┌───────────────┴───────────────┐
         │                               │
         ▼                               ▼
┌──────────────────────┐        ┌──────────────────────┐
│  RASPBERRY PI        │        │  RASPBERRY PI        │
│  (hardware proxy)    │        │  (hardware proxy)    │
│                      │        │                      │
│  • espflash (flash)  │        │  • espflash (flash)  │
│  • Python monitor    │        │  • Python monitor    │
│  • UART aggregator   │        │  • UART aggregator   │
│                      │        │                      │
│  USB (debug)    ─────┼────────┤  GPIO UART  (GPIO4/5)
│  UART1 (log)    ─────┼────────┤  UART2 (GPIO8/9)
│  UART2 (cmds)   ─────┤        │
│  GPIO (future)  ─────┤        │
└─────────┬────────────┘        │
          │                      │
          └──────────────────────┤
                                 │
                    ┌────────────▼──────────────┐
                    │   ESP32-C6               │
                    │                          │
                    │  USB Debug (via RPi)     │
                    │  UART1 (Debug Logs)      │
                    │  UART2 (Commands)        │
                    │                          │
                    │  GPIO13 (LED)            │
                    │  GPIO9 (Button)          │
                    │  GPIO6/7 (I2C)           │
                    │  GPIO4/5 (UART pins)     │
                    └────────────────────────┘
```

---

## 🎯 New Recommendation: **YES to UART, and Here's Why**

With the RPi as a proxy, adding UART becomes **much more valuable**:

### Current Bottleneck
```bash
Laptop → SSH to RPi → espflash monitor /dev/ttyUSB0
```
❌ Limited bandwidth back to laptop
❌ One serial connection for everything
❌ No reliable way to send commands to device

---

### With UART Port Strategy
```
ESP32-C6 has:
  ├─ USB Debug (flash + one log stream)
  └─ UART Port on GPIO4/5 (connected to RPi GPIO UART)

RPi has:
  ├─ /dev/ttyUSB0  (USB from ESP32)
  ├─ /dev/ttyAMA0  (Onboard UART)
  └─ Python aggregator
       ├─ Reads both ports
       ├─ Multiplexes output
       └─ Sends back to laptop over SSH
```

**Benefits:**
- ✅ Two independent serial channels
- ✅ No data collision (one for logs, one for commands)
- ✅ RPi aggregates everything
- ✅ Works over slow SSH connection (smaller packets)
- ✅ Perfect for testing: send commands, get structured responses
- ✅ Teaching value: students learn UART with real RPi setup

---

## 🏗️ Proposed Three-Part Implementation

### Part 1: Current (This Week)
**Goal**: Get blinking LED working with GPIO13

**Just use USB debug port:**
```bash
./scripts/build-flash-monitor.sh 01-blinky
```

Python monitor reads `/dev/cu.usbserial-110` (USB from RPi)

---

### Part 2: Add UART Port (Lesson 05-06)
**Goal**: Second UART for commands/responses

**Firmware changes:**
```rust
// In lesson code when we reach Lesson 06

#[embassy_executor::task]
async fn command_handler() {
    let uart = Uart::new_async(
        peripherals.UART1,
        Config::default(),
        peripherals.GPIO4,  // TX
        peripherals.GPIO5,  // RX
    );

    loop {
        // Read JSON command
        match read_json_command(&mut uart).await {
            Ok(cmd) => {
                let response = handle_command(cmd).await;
                send_json_response(&mut uart, response).await;
            }
            Err(_) => continue,
        }
    }
}

// Command types:
// {"cmd": "led", "action": "blink", "count": 5}
// {"cmd": "button", "action": "simulate"}
// {"cmd": "sensor", "action": "read"}
```

**Hardware wiring (trivial):**
- ESP32 GPIO4 → RPi GPIO14 (UART0 RX)
- ESP32 GPIO5 → RPi GPIO15 (UART0 TX)
- GND → GND

---

### Part 3: Testing Framework (Lesson 07+)
**Goal**: Automated hardware testing

```python
# scripts/rpi_test_runner.py
class RemoteHardwareTester:
    def __init__(self, rpi_host='raspberrypi.local'):
        self.ssh = paramiko.SSHClient()
        self.ssh.connect(rpi_host)

    def send_command(self, cmd_json):
        """Send command via UART to ESP32"""
        stdin, stdout, stderr = self.ssh.exec_command(
            f"python3 /home/pi/monitor.py --cmd '{cmd_json}'"
        )
        return json.loads(stdout.read().decode())

    def test_led_blink(self):
        response = self.send_command({
            'cmd': 'led',
            'action': 'blink',
            'count': 5
        })
        assert response['status'] == 'OK'
        assert response['blinks_completed'] == 5

# Run from laptop
if __name__ == '__main__':
    tester = RemoteHardwareTester()
    tester.test_led_blink()
    tester.test_button_press()
    tester.test_sensor_read()
```

---

## 🎯 Timeline

| Week | Task | Benefit |
|------|------|---------|
| **This** | Python monitor on RPi, fix GPIO13 blinky | Unblock current lesson |
| **Next** | Lessons 02-05 (button, state, async) | Learn fundamentals |
| **Week 3** | Add UART port to ESP32-C6 | Add second channel |
| **Week 4** | Command protocol + testing | Automated hardware tests |

---

## 💻 Immediate Next Steps

1. **On RPi**, install Python dependencies:
   ```bash
   pip3 install pyserial paramiko
   ```

2. **Copy monitor script to RPi**:
   ```bash
   scp scripts/monitor.py pi@raspberrypi.local:/home/pi/
   ```

3. **SSH into RPi, run monitor**:
   ```bash
   ssh pi@raspberrypi.local
   python3 monitor.py --port /dev/ttyUSB0
   ```

4. **From laptop, trigger build on RPi**:
   ```bash
   ssh pi@raspberrypi.local "cd esp32-c6-agentic-firmware && cargo build --release"
   scp -r pi@raspberrypi.local:esp32-c6-agentic-firmware/target/... .
   ssh pi@raspberrypi.local "espflash flash /home/pi/blinky --port /dev/ttyUSB0"
   ```

Or simpler: run everything on RPi, just forward output to laptop.

---

## 📋 Files to Create This Week

```
scripts/
├── monitor.py                      # Python UART monitor
├── remote-build-flash.sh           # SSH to RPi for build/flash
├── rpi-monitor.py                  # Run on RPi (aggregates both UARTs)
└── test_runner.py                  # Tests via UART commands
```

---

## 🤔 Questions for You

1. **RPi Setup**: What's the current wiring? Is `/dev/ttyUSB0` available on RPi?
2. **Power**: Is RPi powered via ESP32 or separately?
3. **SSH Access**: Can you SSH to RPi from laptop reliably?
4. **Future Scale**: Plan to have multiple ESP32s? (UART becomes critical then)

---

## ✨ Why This Matters

With RPi + UART, you're building **production-grade testing infrastructure**:

- ✅ Decoupled logging and commands (two UART ports)
- ✅ Remote hardware testing (perfect for lessons)
- ✅ Scalable to multiple devices
- ✅ Students learn real embedded patterns
- ✅ CI/CD ready (test on RPi hardware)

**This is how real firmware teams do it!**

---

**Next: Should we start by setting up the Python monitor on the RPi?**

---
description: Generate esp-hal 1.0.0 + Claude Code curriculum with progressive UART infrastructure
---

# /gen-all-lessons - esp-hal 1.0.0 + Claude Code Curriculum

**Purpose**: Build practical embedded firmware with esp-hal 1.0.0 Rust, using Claude Code + GDB + UART as your AI debugging partner. Each lesson builds on a progressive UART CLI + streaming infrastructure.

**Target Audience**: Embedded developers learning esp-hal 1.0.0 with AI-assisted debugging workflows.

**Time Estimate**: 18-30 hours total (3-6 hours per lesson × 5 lessons)

**Prerequisites**:
- ESP32-C6-DevKit-C board
- Hardware: Onboard button (GPIO9), onboard Neopixel (GPIO8), MPU6050 I2C sensor, FTDI UART adapter, LED + 220Ω resistor
- On `main` branch with clean working directory
- All dependencies installed (esp-hal, espflash, probe-rs, GDB)

---

## Hardware Inventory

### Onboard (ESP32-C6-DevKit-C)
- ✅ Button (GPIO9, active LOW)
- ✅ Neopixel WS2812 (GPIO8, RMT peripheral)

### External Components
- ✅ MPU6050 IMU (I2C on GPIO2=SDA, GPIO11=SCL)
- ✅ FTDI UART adapter (GPIO23=TX, GPIO15=RX)
- ✅ LED + 220Ω resistor (GPIO12, for PWM)

---

## Curriculum Overview (5 Lessons)

```
esp-hal 1.0.0 + Claude Code Curriculum (Progressive Infrastructure)
  ├─ Lesson 01: GPIO Basics + GDB Fundamentals (⭐⭐☆☆☆, 90-120 min)
  ├─ Lesson 02: UART CLI + Streaming Infrastructure (⭐⭐⭐☆☆, 180-240 min)
  ├─ Lesson 03: PWM + Neopixel Drivers (extend CLI) (⭐⭐⭐☆☆, 180-240 min)
  ├─ Lesson 04: MPU6050 + State Machine (extend CLI) (⭐⭐⭐⭐☆, 240-300 min)
  └─ Lesson 05: Posture Monitor Device (full integration) (⭐⭐⭐⭐⭐, 300-420 min)
```

**Philosophy**:
- **Progressive infrastructure** - Each lesson extends the UART CLI built in Lesson 2
- **Learn by debugging** - Intentional bugs, Claude Code uses GDB to find and fix
- **Advanced GDB throughout** - Use techniques when needed, not in separate "advanced" lesson
- **Hardware-based unit testing** - CLI + GDB validate register state after every command
- **Build a real device** - Culminates in functional posture monitor

---

## Key Innovation: Progressive UART CLI

### Lesson 02 Foundation
```
Commands:
  gpio.init <pin>
  gpio.on <pin>
  gpio.off <pin>
  stream.start

Telemetry:
  [gpio12=1 counter=45]
```

### Lesson 03 Extension (adds PWM + Neopixel)
```
Commands:
  gpio.* (from L02)
  pwm.init <pin> <freq_hz>
  pwm.duty <pin> <percent>
  neo.init <pin>
  neo.color <r> <g> <b>
  stream.start

Telemetry:
  [gpio12=1 pwm12=50% pwm_freq=1000 neo_r=255 neo_g=0 neo_b=0]
```

### Lesson 04 Extension (adds IMU + State Machine)
```
Commands:
  gpio.*, pwm.*, neo.* (from L02-03)
  imu.init
  imu.cal
  imu.read
  state.set <state>
  stream.start

Telemetry:
  [state=Monitoring tilt=5.2° imu_x=245 imu_y=-12 imu_z=16384 neo=green]
```

### Lesson 05 Final Device (all commands + device modes)
```
Commands:
  All previous commands available for testing
  device.start posture_monitor
  device.cal_zero
  device.sleep

Telemetry:
  [device=PostureMonitor state=Normal tilt=5.2° neo=green led=off ...]
```

**The CLI becomes your hardware testing interface** - no need to reflash to test different scenarios!

---

## Repository Refactoring Workflow

**IMPORTANT:** This command performs a complete curriculum refactoring. It will:
1. Tag current main for rollback safety
2. Archive old lessons to `archive/`
3. Generate new lessons 01-05 with progressive infrastructure
4. Test all lessons on hardware
5. Create lesson branches with progressive commits

**Existing lesson branches** (will be reused):
- `lesson-01` - Can be updated with new Lesson 01 content
- `lesson-01-with-gpio9-input` - Variant branch
- `lesson-02` - Can be updated with new Lesson 02 content

---

## Step 0: Repository Refactoring Setup

### 0.1: Tag Current State

Create a snapshot tag before refactoring for rollback safety:

```bash
# Create timestamp tag
TIMESTAMP=$(date +"%Y%m%d-%H%M%S")
TAG_NAME="pre-refactor-$TIMESTAMP"

git tag -a "$TAG_NAME" -m "Snapshot before esp-hal 1.0.0 curriculum refactoring

Preserves lessons 01-08 before generating new progressive curriculum.
Old lessons will be archived to archive/lessons-old-*/

To rollback: git checkout $TAG_NAME"

git push origin "$TAG_NAME"
echo "✓ Created tag: $TAG_NAME"
echo "  Rollback: git checkout $TAG_NAME"
```

### 0.2: Archive Old Lessons

Move existing lessons to timestamped archive directory:

```bash
# Create archive with timestamp
ARCHIVE_DIR="archive/lessons-old-$(date +%Y%m%d)"
mkdir -p "$ARCHIVE_DIR"

# Move existing lessons
if [ -d "lessons" ]; then
    mv lessons/ "$ARCHIVE_DIR/"
    git add "$ARCHIVE_DIR"
    git commit -m "chore: Archive old lessons before refactoring

Moved lessons 01-08 to $ARCHIVE_DIR/ before
generating new esp-hal 1.0.0 + GDB curriculum.

Old lessons preserved for reference.
Snapshot tag: $TAG_NAME"
    echo "✓ Archived lessons to $ARCHIVE_DIR/"
else
    echo "⚠ No lessons/ directory found, skipping archive"
fi
```

### 0.3: Verify Clean State

Confirm repository is ready for new curriculum generation:

```bash
# Check git status
if [ -n "$(git status --porcelain)" ]; then
    echo "ERROR: Working directory not clean after archiving"
    git status
    exit 1
fi
echo "✓ Working directory clean"

# Verify lessons/ removed
if [ -d "lessons" ]; then
    echo "ERROR: lessons/ directory still exists"
    exit 1
fi
echo "✓ Ready for new lessons generation"

# Show archive location
echo "✓ Old lessons archived to: $ARCHIVE_DIR/"
ls -la "$ARCHIVE_DIR/lessons/" | head -10
```

### 0.4: Verify Hardware Availability

**CRITICAL:** Confirm all hardware is available before starting 18-30 hour curriculum generation.

Required hardware:
- ✅ ESP32-C6-DevKit-C (USB-C cable for USB-JTAG)
- ✅ MPU6050 I2C sensor + jumper wires
- ✅ FTDI UART adapter
- ✅ LED + 220Ω resistor
- ✅ Breadboard

**Interactive check:**
```bash
echo "Do you have all required hardware ready? (y/n)"
echo "  - ESP32-C6-DevKit-C with USB-C cable"
echo "  - MPU6050 I2C sensor + jumper wires"
echo "  - FTDI UART adapter"
echo "  - LED + 220Ω resistor + breadboard"
read -p "> " HARDWARE_READY

if [ "$HARDWARE_READY" != "y" ]; then
    echo "ERROR: Get all hardware before starting curriculum generation"
    exit 1
fi
echo "✓ Hardware confirmed ready"
```

---

## Pre-Flight Checklist

### Repository State
```bash
git branch --show-current  # Should show "main"
git status  # Should show "nothing to commit, working tree clean"
git tag | grep pre-refactor  # Should show snapshot tag
ls archive/lessons-old-*/  # Should show archived lessons
```

### Hardware Available
- [ ] ESP32-C6-DevKit-C (USB-C cable for USB-JTAG)
- [ ] MPU6050 I2C sensor + jumper wires
- [ ] FTDI UART adapter
- [ ] LED + 220Ω resistor
- [ ] Breadboard

### Software Installed
```bash
cargo --version
espflash --version
probe-rs --version  # Optional, for advanced GDB
riscv32-esp-elf-gdb --version  # Optional
cargo search esp-hal  # Should show 1.0.0+
```

---

## Lesson 01: GPIO Basics + GDB Fundamentals

**Duration**: 90-120 minutes
**Complexity**: ⭐⭐☆☆☆
**Hardware**: ESP32-C6-DevKit-C (button + LED)

### Learning Objectives

**esp-hal 1.0.0 APIs**:
- GPIO input (button with pull-up)
- GPIO output (LED control)
- Basic polling and debouncing

**Claude Code + GDB**:
- Memory inspection (`x/`, `print`)
- Variable modification (`set`)
- Function calls from GDB (`call`)
- Breakpoints and stepping

### Hardware Setup

**Wiring**:
```
ESP32-C6        LED
--------        ---
GPIO12     -->  Anode (long leg)
               Cathode (short leg) --> 220Ω resistor --> GND
GPIO9      -->  Onboard BOOT button (no wiring needed)
```

### Progressive Commits (4 commits)

**Commit 1: Broken GPIO init** (Bug: Missing GPIO enable)
- LED initialization incomplete
- Button reads but LED doesn't respond
- **Claude uses GDB to**:
  - Inspect GPIO registers (`x/16x 0x60004000`)
  - Discover GPIO peripheral not enabled
  - Call `gpio_enable()` from GDB to test
  - Fix code

**Commit 2: Working button polling** (Bug: No debounce)
- Button reads, LED toggles... but bounces
- Multiple toggles per press
- **Claude uses GDB to**:
  - Set breakpoint on button read
  - Step through and observe rapid transitions
  - Add debounce logic

**Commit 3: LED control functions**
- Add `led_on()`, `led_off()`, `led_toggle()` functions
- **Claude demonstrates**:
  - Call functions from GDB: `call led_toggle()`
  - Control LED remotely without code changes
  - Live firmware interaction

**Commit 4: GDB-based register validation**
- Add assertions for expected GPIO register state
- **Claude shows**:
  - After `led_on()`, check `GPIO_OUT_REG` has bit 12 set
  - Use GDB to validate hardware state matches expectations
  - **Hardware-based unit testing pattern**

### Claude Code + GDB Workflow

**Pattern: Debug → Inspect → Fix → Validate**
1. Bug: LED doesn't work
2. GDB inspects GPIO registers, finds missing enable bit
3. Test fix by calling function from GDB
4. Apply fix to code
5. Validate with register assertions

### Success Criteria

- [ ] All 4 commits build successfully
- [ ] Commit 1: Claude uses GDB to find missing GPIO enable
- [ ] Commit 2: Claude adds debounce logic via GDB analysis
- [ ] Commit 3: Functions callable from GDB
- [ ] Commit 4: GDB validates register state after function calls
- [ ] README documents GDB commands used for each bug

---

## Lesson 02: UART CLI + Streaming Infrastructure

**Duration**: 180-240 minutes
**Complexity**: ⭐⭐⭐☆☆
**Hardware**: ESP32-C6 + FTDI UART adapter + LED

### Learning Objectives

**esp-hal 1.0.0 APIs**:
- UART peripheral configuration
- DMA for high-throughput streaming
- Command parsing (simple CLI)

**Claude Code + GDB**:
- Mode switching via GDB (CLI ↔ streaming)
- Watchpoints on buffer overflow
- Live parameter tuning

**Real Firmware Pattern**:
- **CLI Mode**: Interactive command interface (testing/debugging)
- **Streaming Mode**: High-speed telemetry output (monitoring)
- **Mode toggle**: Via GDB or command
- **Hardware unit testing**: CLI commands → GDB validates registers

### Hardware Setup

**Wiring**:
```
ESP32-C6        FTDI UART
--------        ---------
GPIO23 (TX) --> RX
GPIO15 (RX) --> TX
GND         --> GND

GPIO12     -->  LED + 220Ω resistor (from Lesson 01)
```

### CLI Commands (This Lesson)

```
> help
Commands:
  gpio.init <pin>     - Initialize GPIO as output
  gpio.on <pin>       - Set GPIO high
  gpio.off <pin>      - Set GPIO low
  gpio.deinit <pin>   - Deinitialize GPIO
  stream.start        - Start streaming telemetry
  stream.stop         - Stop streaming (back to CLI)
  help                - Show commands

> gpio.init 12
OK [GPIO12 initialized as output]

> gpio.on 12
OK [GPIO12 = HIGH]

> gpio.off 12
OK [GPIO12 = LOW]

> stream.start
[Switching to streaming mode...]
[gpio12=0 counter=1 uptime_ms=1234]
[gpio12=0 counter=2 uptime_ms=1334]
[gpio12=1 counter=3 uptime_ms=1434]
...
```

**Key Innovation**: CLI commands call **real firmware functions** (`gpio_init()`, `gpio_set_high()`, etc.)

After each command, **GDB validates hardware state**:
- `gpio.init 12` → GDB checks `GPIO_ENABLE_REG` bit 12 = 1
- `gpio.on 12` → GDB checks `GPIO_OUT_REG` bit 12 = 1

### Progressive Commits (6 commits)

**Commit 1: Basic UART TX** (Bug: Blocking writes)
- Send "Hello World" over UART
- Blocking, low throughput
- **Claude uses GDB to**:
  - Measure time spent in `uart_write()`
  - Profile with breakpoints and cycle counters

**Commit 2: CLI parser** (Bug: Buffer overflow on long commands)
- Parse commands: `gpio.init`, `gpio.on`, `gpio.off`
- Buffer overflow with >64 char input
- **Claude uses GDB to**:
  - Set **watchpoint** on buffer boundary: `watch *(char*)(&cmd_buf[64])`
  - Catches overflow, adds bounds check

**Commit 3: GPIO control via CLI**
- Implement `gpio.init`, `gpio.on`, `gpio.off`, `gpio.deinit`
- Commands call real firmware functions
- **Hardware-based unit test pattern**:
  ```
  > gpio.init 12
  OK
  # Claude uses GDB: x/1xw 0x60004008  (GPIO enable register)
  # Validates: Bit 12 = 1 ✓

  > gpio.on 12
  OK
  # Claude uses GDB: x/1xw 0x60004004  (GPIO output register)
  # Validates: Bit 12 = 1 ✓
  ```

**Commit 4: Streaming mode** (Bug: DMA misconfiguration)
- Add `stream.start` command → switch to streaming telemetry
- Stream GPIO state, counter values at 10 Hz
- DMA doesn't transfer correctly (wrong buffer alignment)
- **Claude uses GDB to**:
  - Inspect DMA descriptor: `print &dma_desc`
  - Find buffer address not 4-byte aligned
  - Fix alignment with `#[repr(align(4))]`

**Commit 5: Mode switching via GDB**
- Add global variable `MODE: u8` (0=CLI, 1=Streaming)
- **Claude demonstrates**:
  - Firmware in streaming mode (data flood)
  - GDB: `set MODE = 0` → switches to CLI without recompile
  - GDB: `set MODE = 1` → back to streaming
  - **Live firmware reconfiguration**

**Commit 6: DMA optimization + error handling**
- Bug: DMA buffer fills faster than transmission rate
- Overflow drops data
- **Claude uses GDB to**:
  - Set **watchpoint** on buffer write pointer
  - Catches when write overtakes read (overflow condition)
  - Adds ring buffer overflow protection

### Streaming Telemetry Format

```
[gpio12=0 gpio_changes=5 counter=123 uptime_ms=12340 mode=streaming]
[gpio12=1 gpio_changes=6 counter=124 uptime_ms=12440 mode=streaming]
```

**Parseable by scripts** for automated testing and analysis.

### Claude Code + GDB Workflow

**Pattern: Hardware Unit Testing via CLI**
1. User types: `gpio.init 12`
2. Firmware calls `gpio_init(12)`
3. **Claude uses GDB**: `x/1xw 0x60004008` (GPIO enable register)
4. Expected: Bit 12 = 1
5. If wrong: GDB catches hardware state mismatch → bug found
6. Stream validates: `gpio12=0` (initialized but off)

**Pattern: Live Mode Switching**
1. Firmware in streaming mode (10 Hz data flood)
2. Want to test CLI command without reflashing
3. GDB: `set MODE = 0`
4. Firmware switches to CLI mode
5. Test command: `gpio.on 12`
6. GDB validates register
7. GDB: `set MODE = 1` → back to streaming
8. Stream shows: `gpio12=1` ✓

### Success Criteria

- [ ] All 6 commits build successfully
- [ ] CLI commands work: `gpio.init`, `gpio.on`, `gpio.off`, `gpio.deinit`
- [ ] Commands call real firmware functions (not stubs)
- [ ] **GDB validates GPIO registers after each command**
- [ ] Streaming mode outputs telemetry at 10 Hz
- [ ] Mode switchable via GDB (`set MODE = ...`)
- [ ] Watchpoint catches buffer overflow
- [ ] DMA alignment issue found and fixed with GDB
- [ ] **This CLI becomes the testing backbone for all future lessons**

---

## Lesson 03: PWM + Neopixel Drivers (Extend CLI)

**Duration**: 180-240 minutes
**Complexity**: ⭐⭐⭐☆☆
**Hardware**: ESP32-C6 + LED (PWM) + onboard Neopixel + UART adapter

### Learning Objectives

**esp-hal 1.0.0 APIs**:
- LEDC peripheral (PWM for LED brightness)
- RMT peripheral (WS2812 Neopixel timing)
- Clock configuration and dividers

**Claude Code + GDB (Advanced)**:
- **Python GDB scripting** - Automate testing 1000 color combinations
- **Disassembly** - Inspect RMT timing code for WS2812 protocol violations
- Debug timing issues (wrong PWM frequency, RMT clock divider)

**Intentional Bugs**:
1. PWM frequency wrong (forgot prescaler)
2. Neopixel shows wrong colors (RGB vs GRB byte order)
3. Neopixel flickers (RMT clock divider miscalculated)

### Hardware Setup

**Wiring**:
```
ESP32-C6        LED
--------        ---
GPIO12     -->  Anode (long leg) [same as previous lessons]
               Cathode (short leg) --> 220Ω resistor --> GND

GPIO8      -->  Onboard Neopixel (no wiring needed)

GPIO23/15  -->  FTDI UART [same as Lesson 02]
```

### Extended CLI Commands

**From Lesson 02** (still available):
```
gpio.init, gpio.on, gpio.off, stream.start, stream.stop
```

**New in Lesson 03**:
```
> pwm.init <pin> <freq_hz>
> pwm.duty <pin> <percent>
> pwm.deinit <pin>
> neo.init <pin>
> neo.color <r> <g> <b>
> neo.off <pin>
> stream.start
```

**Example session**:
```
> pwm.init 12 1000
OK [PWM initialized on GPIO12 at 1000 Hz]

> pwm.duty 12 50
OK [PWM duty cycle = 50%]

> neo.init 8
OK [Neopixel initialized on GPIO8]

> neo.color 255 0 0
OK [Neopixel = RED]

> stream.start
[pwm12=50% pwm_freq=1000 neo_r=255 neo_g=0 neo_b=0 uptime_ms=5678]
```

**Hardware validation after each command**:
- `pwm.init 12 1000` → GDB checks LEDC timer register for 1 kHz config
- `neo.color 255 0 0` → GDB checks RMT buffer has GRB sequence (not RGB)

### Progressive Commits (6 commits)

**Commit 1: PWM init** (Bug: Wrong frequency)
- Goal: 1 kHz PWM for LED brightness
- Actual: 80 kHz (forgot prescaler)
- LED barely dims, flickers at high duty cycles
- **Claude uses GDB to**:
  - Inspect LEDC timer registers: `x/8xw 0x60006000`
  - Calculate actual frequency: `APB_CLK / (prescaler * timer_max)`
  - Find prescaler = 1 (should be 80 for 1 kHz from 80 MHz APB)
  - Test fix via GDB: `set ledc.timer.div_num = 80`
  - Confirm LED brightness now smooth

**Commit 2: PWM duty cycle control + CLI**
- Add `pwm.init <pin> <freq>` and `pwm.duty <pin> <percent>` commands
- **Claude demonstrates**:
  - CLI: `pwm.duty 12 75`
  - GDB validates: LEDC duty register = 75% of max
  - LED brightness changes
  - Stream confirms: `pwm12=75%`

**Commit 3: Neopixel driver** (Bug: Wrong color order)
- Goal: `neo.color 255 0 0` → red
- Actual: Shows green
- RGB vs GRB byte order (WS2812 expects GRB)
- **Claude uses GDB to**:
  - Set breakpoint before RMT send
  - Inspect color buffer: `x/3xb &color_buf`
  - See: `[0xFF, 0x00, 0x00]` (RGB)
  - WS2812 expects: `[0x00, 0xFF, 0x00]` (GRB for red)
  - Fix byte order in driver

**Commit 4: Neopixel flickering** (Bug: RMT timing violation)
- `neo.color` works but Neopixel flickers, shows random colors
- RMT clock divider wrong → bit timing violates WS2812 spec
- **Claude uses GDB with DISASSEMBLY**:
  - `disas rmt_send_pulse`
  - Inspect RMT clock config: `x/4xw 0x60006800`
  - Calculate bit timing: `APB_CLK / divider / 32`
  - WS2812 requires: 1.25 µs per bit (800 kHz)
  - Current: `80 MHz / 1 / 32 = 2.5 MHz` (too fast!)
  - Should be: `80 MHz / 2 / 32 = 1.25 MHz` ✓
  - Fix divider: `set rmt.clock_div = 2`

**Commit 5: Python GDB script for color testing**
- **Advanced GDB**: Python script to test color accuracy
- Script sets 100 random RGB values, validates Neopixel output
- **GDB Python script**:
  ```python
  import gdb
  import random

  class TestNeopixelColors(gdb.Command):
      def invoke(self, arg, from_tty):
          for i in range(100):
              r, g, b = random.randint(0,255), random.randint(0,255), random.randint(0,255)
              gdb.execute(f"call neo_set_color({r}, {g}, {b})")
              # Read back from RMT buffer and validate
              buf = gdb.parse_and_eval("rmt_buffer")
              actual_g = int(buf[0])
              actual_r = int(buf[1])
              actual_b = int(buf[2])
              if actual_r != r or actual_g != g or actual_b != b:
                  print(f"FAIL: Expected ({r},{g},{b}), got ({actual_r},{actual_g},{actual_b})")
              else:
                  print(f"PASS: ({r},{g},{b})")

  TestNeopixelColors()
  ```
- **Claude uses**: `source test_neo.py` → `test-neopixel-colors`
- Automates testing, catches edge cases

**Commit 6: Unified streaming telemetry**
- Stream PWM + Neopixel state to UART
- Format: `[pwm12=50% pwm_freq=1000 neo_r=128 neo_g=0 neo_b=255]`
- **Claude demonstrates full workflow**:
  1. CLI: `pwm.duty 12 75`
  2. GDB validates LEDC register
  3. Stream confirms: `pwm12=75%`
  4. CLI: `neo.color 0 255 0`
  5. GDB validates RMT buffer (GRB order)
  6. Stream confirms: `neo_r=0 neo_g=255 neo_b=0`

### Claude Code + GDB Workflow

**Pattern: Timing Bug Detective with Disassembly**
1. Symptom: Neopixel flickers
2. GDB disassembles RMT code: `disas rmt_send_pulse`
3. Inspects RMT clock register: `x/1xw 0x60006804`
4. Calculates bit timing from APB clock and divider
5. Finds timing violation (too fast for WS2812)
6. Fixes clock divider via GDB, validates with oscilloscope/timing

**Pattern: Automated Testing with Python GDB**
1. Need to test 1000 color combinations (tedious manually)
2. Write Python GDB script to automate
3. Script calls `neo_set_color()` with random RGB
4. Script validates RMT buffer has correct GRB sequence
5. Catches edge cases humans would miss

### Success Criteria

- [ ] All 6 commits build successfully
- [ ] PWM frequency bug found via GDB register inspection
- [ ] RGB vs GRB byte order issue discovered and fixed
- [ ] RMT clock divider bug diagnosed with disassembly
- [ ] Python GDB script automates color testing
- [ ] CLI commands work: `pwm.*`, `neo.*`
- [ ] GDB validates registers after each CLI command
- [ ] Streaming telemetry includes PWM + Neopixel state
- [ ] README documents each bug and GDB techniques used

---

## Lesson 04: MPU6050 Driver + State Machine (Extend CLI)

**Duration**: 240-300 minutes
**Complexity**: ⭐⭐⭐⭐☆
**Hardware**: ESP32-C6 + MPU6050 IMU + Button + LED + Neopixel + UART

### Learning Objectives

**esp-hal 1.0.0 APIs**:
- I2C peripheral configuration
- MPU6050 register map (accelerometer, gyroscope)
- Interrupt handling (button)
- State machines in embedded systems

**Claude Code + GDB (Advanced)**:
- **Conditional breakpoints** - Break only on I2C errors
- **Tracepoints** - Profile I2C transaction frequency without stopping
- I2C bus debugging (clock stretching, ACK/NAK)
- State machine visualization and forced transitions

**Real Firmware Pattern**:
- Button-controlled state machine: Sleep → Monitoring → Calibrating
- State-dependent behavior (different peripherals active per state)
- Telemetry includes state + sensor data

### Hardware Setup

**Wiring**:
```
ESP32-C6        MPU6050
--------        -------
GPIO2 (SDA) --> SDA
GPIO11 (SCL)--> SCL
3.3V        --> VCC
GND         --> GND

GPIO9       --> Onboard BOOT button
GPIO12      --> LED (PWM, from previous lessons)
GPIO8       --> Neopixel (from previous lessons)
GPIO23/15   --> FTDI UART (from previous lessons)
```

### Extended CLI Commands

**From Lessons 02-03** (still available):
```
gpio.*, pwm.*, neo.*, stream.*
```

**New in Lesson 04**:
```
> imu.init
> imu.whoami
> imu.read
> imu.cal
> state.get
> state.set <sleep|monitoring|calibrating>
> stream.start
```

**Example session**:
```
> imu.init
OK [MPU6050 initialized at 0x68]

> imu.whoami
WHO_AM_I = 0x68 ✓

> imu.read
accel: x=245 y=-12 z=16384  gyro: x=3 y=-8 z=1

> imu.cal
Calibrating... [samples=100/100] Offsets: ax=2 ay=1 az=-4
OK [Calibration complete]

> state.set monitoring
OK [State = Monitoring]

> stream.start
[state=Monitoring imu_ax=245 imu_ay=-12 imu_az=16380 neo=green]
```

**Hardware validation**:
- `imu.init` → GDB checks I2C peripheral registers (clock config, enable bit)
- `imu.read` → GDB validates I2C buffer has 14 bytes (6 accel + 6 gyro + 2 temp)

### State Machine Design

```
      ┌─────────┐
      │  Sleep  │ (Button press)
      └────┬────┘        ↓
           │       ┌──────────┐
           │       │Monitoring│ (Button press)
           │       └────┬─────┘       ↓
           │            │       ┌────────────┐
           └────────────┘       │Calibrating │
                                └──────┬─────┘
                                       │ (Auto after 5s or 100 samples)
                                       ↓
                                  [Back to Monitoring]
```

**State-dependent behavior**:
- **Sleep**: I2C off, Neopixel off, LED off
- **Monitoring**: I2C active (10 Hz reads), Neopixel shows status
- **Calibrating**: I2C active (100 Hz reads), Neopixel pulses, collect offset samples

### Progressive Commits (7 commits)

**Commit 1: I2C init + device scan** (Bug: Wrong I2C speed)
- Goal: Detect MPU6050 at 0x68
- Actual: No ACK (device not found)
- I2C speed set to 1 MHz, MPU6050 supports max 400 kHz
- **Claude uses GDB to**:
  - Inspect I2C clock config: `x/8xw 0x60013000`
  - Calculate actual I2C freq from prescaler/divider
  - Find: `APB_CLK / prescaler = 1 MHz` (too fast!)
  - Fix: Set I2C to 100 kHz (safe default)
  - GDB validates: `imu.init` → WHO_AM_I reads 0x68 ✓

**Commit 2: Read WHO_AM_I register**
- Add `imu.whoami` command
- **Claude uses GDB to**:
  - Set breakpoint after I2C read
  - Inspect buffer: `x/1xb &who_am_i_buf`
  - Validate: 0x68 (correct per datasheet)

**Commit 3: Read accel/gyro data** (Bug: Axis swap)
- Add `imu.read` command
- Bug: Tilting forward shows Z-axis change (should be Y)
- **Claude uses GDB to**:
  - Inspect raw I2C buffer: `x/14xb &i2c_buf`
  - Compare to MPU6050 register map (datasheet)
  - Find: Reading registers in wrong order
  - Fix register address sequence
  - Validate: Tilt forward → Y-axis changes ✓

**Commit 4: Button state machine**
- Button press cycles: Sleep → Monitoring → Calibrating → Monitoring
- **Claude uses GDB to**:
  - Force state transitions: `set state = STATE_MONITORING`
  - Observe behavior without button presses
  - Validate state machine logic
  - Find bug: Missing transition from Calibrating back to Monitoring
  - Fix transition logic

**Commit 5: Calibration mode** (Bug: Overflow in averaging)
- Collect 100 samples, compute average offset
- Bug: Overflow when summing accel values (i16 samples, i16 accumulator)
- **Claude uses GDB with CONDITIONAL BREAKPOINT**:
  - `break calibration_loop if accel_sum < 0 && sample_count < 50`
  - Catches overflow (sum goes negative unexpectedly)
  - Inspect: `print accel_sum` → wraps at ~32767
  - Fix: Use i32 accumulator

**Commit 6: I2C error handling** (Bug: No timeout on clock stretch)
- I2C occasionally hangs (sensor holds SCL low)
- **Claude uses GDB with CONDITIONAL BREAKPOINT**:
  - `break i2c_read if status == I2C_TIMEOUT`
  - Catches timeout condition
  - Inspect I2C status register: `x/1xw 0x60013004`
  - Find: Clock stretch timeout not configured
  - Add timeout recovery logic

**Commit 7: Tracepoints for performance profiling**
- **Advanced GDB**: Use tracepoints to profile I2C transaction rate
- **Tracepoint** (logs without stopping execution):
  ```
  (gdb) trace i2c_read_accel
  (gdb) actions
  > collect $pc, sample_count
  > end
  (gdb) tstart
  [Firmware runs...]
  (gdb) tstop
  (gdb) tfind start
  (gdb) info tracepoints
  ```
- **Claude uses** to measure:
  - I2C transaction frequency (should be 10 Hz in Monitoring)
  - Timing jitter (variance between reads)
  - Optimize read rate without stopping firmware

### Streaming Telemetry Format

```
[state=Sleep neo=off led=off i2c_disabled=1]
[state=Monitoring imu_ax=245 imu_ay=-12 imu_az=16384 imu_gx=3 imu_gy=-8 imu_gz=1 neo=green]
[state=Calibrating samples=45/100 sum_ax=11025 neo=pulsing]
[state=Monitoring [CALIBRATED] imu_ax=2 imu_ay=0 imu_az=16380 neo=green]
```

### Claude Code + GDB Workflow

**Pattern: I2C Bus Debugging with Conditional Breakpoints**
1. Symptom: I2C occasionally returns 0xFF (NAK or timeout)
2. Too rare to catch with normal breakpoint
3. **Conditional breakpoint**: `break i2c_read if ret != I2C_OK`
4. GDB breaks only on failure
5. Inspect I2C status: `x/1xw 0x60013004`
6. Find: Clock stretch timeout
7. Fix: Increase timeout, add recovery logic

**Pattern: State Machine Debugging**
1. State doesn't transition from Calibrating to Monitoring
2. GDB: `set state = STATE_CALIBRATING`
3. GDB: Set breakpoint on state transition code
4. Step through logic
5. Find: Missing condition check
6. Fix: Add `if samples >= 100 { state = Monitoring; }`

**Pattern: Performance Profiling with Tracepoints**
1. Need to measure I2C transaction rate
2. Normal breakpoints would disrupt timing
3. **Tracepoint**: Logs without stopping
4. `trace i2c_read_accel`, `actions`, `collect $pc, timestamp`
5. Analyze trace data offline
6. Find: Actual rate = 9.7 Hz (should be 10 Hz)
7. Fix timing in main loop

### Success Criteria

- [ ] All 7 commits build successfully
- [ ] I2C frequency bug found via register inspection
- [ ] WHO_AM_I reads correctly (0x68)
- [ ] Axis swap bug diagnosed and fixed
- [ ] State machine transitions correctly
- [ ] Calibration overflow bug caught with conditional breakpoint
- [ ] I2C timeout bug found and fixed
- [ ] Tracepoints measure I2C performance without stopping
- [ ] CLI commands work: `imu.*`, `state.*`
- [ ] GDB validates I2C registers after each command
- [ ] Streaming telemetry includes state + sensor data

---

## Lesson 05: Posture Monitor Device (Full Integration)

**Duration**: 300-420 minutes
**Complexity**: ⭐⭐⭐⭐⭐
**Hardware**: ESP32-C6 + all peripherals (LED, Neopixel, Button, MPU6050, UART)

### Device Specification

**Name**: Posture/Orientation Monitor
**Purpose**: Alert user when device tilts beyond safe angle (desk mount, posture reminder, tilt alarm)

**Behavior**:
- **Normal** (0-30° tilt): Neopixel green, LED off
- **Warning** (30-60° tilt): Neopixel yellow, LED slow blink (1 Hz)
- **Alert** (>60° tilt): Neopixel red, LED fast blink (5 Hz)
- **Button short press**: Calibrate "zero" orientation (current position = 0°)
- **Button long press** (3s): Enter sleep mode (all off)
- **Sleep** + button press: Wake up → Calibrating → Monitoring

### Full CLI (All Previous Commands Available)

**From Lessons 02-04** (for testing/debugging):
```
gpio.*, pwm.*, neo.*, imu.*, state.*, stream.*
```

**New in Lesson 05** (device-level commands):
```
> device.start
> device.cal_zero
> device.sleep
> device.wake
> device.status
> stream.start
```

**Example testing session**:
```
> device.start
OK [Posture Monitor started - state=Normal]

> device.status
Device: Posture Monitor
State: Normal (tilt=5.2°)
Neopixel: GREEN
LED: OFF
Thresholds: Warning=30° Alert=60°

> device.cal_zero
OK [Zero orientation calibrated]

# Manual testing without tilting device:
> state.set warning
OK [Forced WARNING state]
# Neopixel turns yellow, LED blinks 1 Hz

> imu.read
accel: x=8450 y=3200 z=12000 (tilt=42.8°)

> stream.start
[device=PostureMonitor state=Warning tilt=42.8° neo=yellow led=blink_1hz ...]
```

**The CLI enables complete device testing without physical interaction!**

### Nested State Machine

```
┌────────────────────────────────────────────────────────────┐
│                     DEVICE_ACTIVE                           │
│  ┌────────┐  (tilt>30°)  ┌─────────┐ (tilt>60°) ┌───────┐ │
│  │ Normal │─────────────→│ Warning │───────────→│ Alert │ │
│  └────┬───┘              └────┬────┘            └───┬───┘ │
│       │                       │                     │     │
│       └───────────────────────┴─────────────────────┘     │
│                   (tilt corrected)                         │
└────────────────────────────┬───────────────────────────────┘
                             │
                      (Button long press)
                             ↓
                      ┌────────────┐
                      │   SLEEP    │
                      └─────┬──────┘
                            │ (Button press)
                            ↓
                      ┌─────────────┐
                      │ CALIBRATING │ (5s or 100 samples)
                      └──────┬──────┘
                             │
                             ↓
                         [Normal]
```

### Progressive Commits (8 commits)

**Commit 1: Multi-peripheral init**
- Initialize all peripherals: LED (PWM), Neopixel, MPU6050, Button, UART
- Basic integration test: All respond to CLI commands
- **Claude validates** via CLI + GDB:
  ```
  > pwm.init 12 1000
  > neo.init 8
  > imu.init
  # GDB validates each peripheral's registers
  ```

**Commit 2: Tilt calculation** (Bug: Wrong axis for tilt)
- Calculate tilt from accelerometer: `tilt = atan2(sqrt(x² + y²), z)`
- Bug: Using wrong axes (yaw instead of pitch/roll)
- **Claude uses GDB to**:
  - Print raw accel while tilting: `print imu_ax, imu_ay, imu_az`
  - See which axis changes (X for roll, Y for pitch)
  - Fix formula to use correct axes
  - Validate: Tilt forward → Y changes, tilt calculation correct

**Commit 3: Nested state machine** (Bug: Backward transitions missing)
- Implement Normal → Warning → Alert based on tilt
- Bug: Device stuck in Alert even after correcting tilt
- **Claude uses GDB with WATCHPOINT**:
  - `watch state`
  - Catches all state changes
  - Force tilt: `set tilt_angle = 25.0` (should go to Normal)
  - Breakpoint doesn't trigger → missing transition logic
  - Fix: Add backward transitions (Alert → Warning → Normal)

**Commit 4: Button calibration** (Bug: Offsets not applied)
- Short press: Store current orientation as "zero"
- Bug: Calibration runs but tilt still shows non-zero when level
- **Claude uses GDB to**:
  - After calibration: `print cal_offset_x, cal_offset_y, cal_offset_z`
  - Offsets stored correctly: `cal_offset_x=245, cal_offset_y=-12, cal_offset_z=-4`
  - Inspect tilt calculation code
  - Find: Offsets read but not subtracted from raw values
  - Fix: Apply offsets before tilt calculation

**Commit 5: LED blink rate control**
- Normal: LED off
- Warning: 1 Hz blink
- Alert: 5 Hz blink
- Implement via PWM duty cycle modulation
- **Claude uses GDB to**:
  - Force states: `set state = STATE_ALERT`
  - Validate LED blink rate via UART telemetry
  - Stream shows: `led_freq=5.0` ✓

**Commit 6: Sleep mode** (Bug: Power not reduced)
- Long press (3s): Enter sleep mode
- Bug: Sleep mode doesn't reduce power (I2C still polling)
- **Claude uses GDB with TRACEPOINT**:
  - `trace i2c_read_accel`
  - `tstart`
  - Enter sleep mode
  - `tstop`, `tfind`
  - Find: I2C still being called in sleep state!
  - Fix: Skip I2C reads when `state == SLEEP`
  - Validate: Tracepoint shows 0 calls in sleep

**Commit 7: Race condition** (Bug: Button ISR vs main loop)
- Intermittent: Device shows invalid state (e.g., `state=255`)
- Happens ~1/500 button presses
- **Claude uses GDB with WATCHPOINT**:
  - `watch -l state`
  - Catches write from unexpected location
  - Backtrace shows: Button ISR writes while main loop reads
  - **Race condition** between ISR and main loop
  - Fix: Use atomic operations or disable interrupts during read

**Commit 8: Statistical anomaly detection** (Bug: I2C error spikes)
- Stream telemetry includes event counters: `i2c_ok`, `i2c_fail`, `btn_presses`
- **Claude analyzes UART stream**:
  - Parses 10,000 telemetry samples
  - Detects pattern: `i2c_fail` spikes when `btn_presses` increases
  - Hypothesis: Button ISR disables interrupts too long, I2C times out
  - **Validates with GDB**:
    - Measure ISR duration with cycle counter
    - Find ISR takes 50 µs (too long!)
    - Optimize ISR → I2C errors disappear

### Streaming Telemetry (JSON Format)

```json
{"ts":12345,"dev":"PostureMonitor","state":"Normal","tilt":5.2,"neo":"green","led":"off","i2c_ok":9823,"i2c_fail":2,"btn":45}
{"ts":12346,"dev":"PostureMonitor","state":"Normal","tilt":5.3,"neo":"green","led":"off","i2c_ok":9824,"i2c_fail":2,"btn":45}
{"ts":12347,"dev":"PostureMonitor","state":"Warning","tilt":35.1,"neo":"yellow","led":"blink_1hz","i2c_ok":9825,"i2c_fail":2,"btn":45}
[ERROR] i2c_timeout addr=0x68 scl_stuck=1
{"ts":12348,"dev":"PostureMonitor","state":"Warning","tilt":35.2,"neo":"yellow","led":"blink_1hz","i2c_ok":9825,"i2c_fail":3,"btn":45}
```

**Parseable by scripts** for automated validation and statistical analysis.

### Claude Code + GDB Workflow (All Advanced Techniques)

**Pattern 1: Watchpoint for Race Condition**
1. Intermittent bug: `state` shows invalid value
2. **Watchpoint**: `watch -l state`
3. GDB breaks on every write
4. Catches write from button ISR while main loop reads
5. Backtrace shows race condition
6. Fix: Atomic operations

**Pattern 2: Tracepoint for Power Profiling**
1. Sleep mode should reduce power
2. **Tracepoint**: `trace i2c_read_accel`
3. Logs calls without stopping firmware
4. Analysis shows I2C still called in sleep
5. Fix: Add state check before I2C reads

**Pattern 3: Statistical Anomaly Detection**
1. UART streams JSON telemetry
2. Claude parses 10,000 samples
3. Detects correlation: Button presses → I2C errors
4. GDB validates hypothesis (ISR too long)
5. Fix reduces I2C error rate to 0%

**Pattern 4: Python GDB for Automated Testing**
```python
# GDB script to test all state transitions
class TestPostureMonitor(gdb.Command):
    def invoke(self, arg, from_tty):
        states = ["Normal", "Warning", "Alert"]
        tilts = [15.0, 45.0, 75.0]

        for state, tilt in zip(states, tilts):
            gdb.execute(f"set tilt_angle = {tilt}")
            actual_state = gdb.parse_and_eval("state")
            expected = states.index(state)
            if int(actual_state) == expected:
                print(f"PASS: tilt={tilt}° → {state}")
            else:
                print(f"FAIL: tilt={tilt}° expected {state}, got {actual_state}")

TestPostureMonitor()
```

### Success Criteria

- [ ] All 8 commits build successfully
- [ ] All peripherals work together (LED, Neopixel, IMU, Button)
- [ ] Tilt calculation correct (validated with GDB)
- [ ] State transitions: Normal → Warning → Alert → Normal
- [ ] Button calibrates zero orientation
- [ ] Button long press enters sleep mode
- [ ] Sleep mode reduces power (tracepoint validates no I2C calls)
- [ ] Race condition caught with watchpoint and fixed
- [ ] Statistical analysis detects I2C error pattern
- [ ] **Device is fully functional and useful** (can mount on desk, use as posture reminder)
- [ ] CLI enables complete testing without physical interaction
- [ ] UART streams parseable JSON telemetry
- [ ] Python GDB scripts automate state transition testing

---

## Step 6: Comprehensive Hardware Testing

After all 5 lessons are generated and committed to main, run comprehensive validation:

```bash
# Use the dedicated testing command
/test-all-lessons
```

This command will:
- ✅ Build each lesson in sequence
- ✅ Flash to hardware via espflash
- ✅ Capture UART output for validation
- ✅ Check expected outputs match success criteria
- ✅ Validate CLI progression (each lesson extends previous)
- ✅ Generate comprehensive test report

**Success criteria:**
- [ ] All 5 lessons build without warnings
- [ ] All 5 lessons flash successfully
- [ ] UART output detected for lessons 02-05
- [ ] CLI commands present in expected lessons
- [ ] Progressive CLI validated (Lesson N includes all commands from Lesson N-1)
- [ ] Test report generated with all PASS results

**See:** `.claude/commands/test-all-lessons.md` for detailed testing workflow

**Time estimate:** 30-60 minutes for full curriculum validation

---

## Step 7: Create Lesson Branches

After all lessons are working on main and tests pass, create progressive lesson branches for the tutorial workflow.

### Existing Lesson Branches

These branches already exist and can be reused:

| Branch | Status | Recommendation |
|--------|--------|----------------|
| `lesson-01` | Exists | Update with new Lesson 01 content |
| `lesson-01-with-gpio9-input` | Exists | Keep as variant or delete |
| `lesson-02` | Exists | Update with new Lesson 02 content |

**Strategy:** Reuse `lesson-01` and `lesson-02`, create new `lesson-03`, `lesson-04`, `lesson-05`

### Branch Creation Workflow

**Option A: Update Existing Branches (Recommended)**

```bash
# Backup existing branches first
for branch in lesson-01 lesson-02; do
    git checkout "$branch"
    git tag "backup-${branch}-$(date +%Y%m%d)"
    git push origin "backup-${branch}-$(date +%Y%m%d)"
done

# Update with new content
git checkout main
git checkout -B lesson-01  # Reset to main
git push --force-with-lease origin lesson-01

git checkout main
git checkout -B lesson-02
git push --force-with-lease origin lesson-02
```

**Option B: Create Fresh Branches**

```bash
# Create clean branches for all 5 lessons
for lesson in 01 02 03 04 05; do
    git checkout -b "lesson-new-${lesson}" main
    git push -u origin "lesson-new-${lesson}"
done

# Later: migrate content to original branch names
```

### Progressive Commits on Branches

Each lesson branch should have progressive commits matching the lesson plan:

**Lesson 01 (4 commits):**
1. Broken GPIO init (Bug: Missing GPIO enable)
2. Working button polling (Bug: No debounce)
3. LED control functions
4. GDB-based register validation

**Lesson 02 (6 commits):**
1. Basic UART TX (Bug: Blocking writes)
2. CLI parser (Bug: Buffer overflow)
3. GPIO control via CLI
4. Streaming mode (Bug: DMA misconfiguration)
5. Mode switching via GDB
6. DMA optimization + error handling

**Lesson 03 (6 commits):**
1. PWM init (Bug: Wrong frequency)
2. PWM duty cycle control + CLI
3. Neopixel driver (Bug: Wrong color order)
4. Neopixel flickering fix (Bug: RMT timing)
5. Python GDB script for color testing
6. Unified streaming telemetry

**Lesson 04 (7 commits):**
1. I2C init + device scan (Bug: Wrong I2C speed)
2. Read WHO_AM_I register
3. Read accel/gyro data (Bug: Axis swap)
4. Button state machine
5. Calibration mode (Bug: Overflow in averaging)
6. I2C error handling (Bug: No timeout)
7. Tracepoints for performance profiling

**Lesson 05 (8 commits):**
1. Multi-peripheral init
2. Tilt calculation (Bug: Wrong axis)
3. Nested state machine (Bug: Backward transitions missing)
4. Button calibration (Bug: Offsets not applied)
5. LED blink rate control
6. Sleep mode (Bug: Power not reduced)
7. Race condition fix (Bug: Button ISR vs main loop)
8. Statistical anomaly detection (Bug: I2C error spikes)

**See:** Each lesson's README for commit-by-commit breakdown and GDB debugging workflows.

### Creating Progressive Commits

**Strategy:** Use `git commit --amend` and `git rebase -i` to create clean progressive history:

```bash
# Example for Lesson 01
git checkout lesson-01

# Commit 1: Broken GPIO init
git add lessons/01-*/src/bin/main.rs  # Broken version
git commit -m "feat(lesson-01): Add GPIO init (broken - missing enable)"

# Commit 2: Fix with debounce bug
git add lessons/01-*/src/bin/main.rs  # Fixed enable, but no debounce
git commit -m "fix(lesson-01): Enable GPIO peripheral, add button polling (broken - no debounce)"

# Commit 3: Add LED functions
git add lessons/01-*/src/bin/main.rs  # Add led_on/off/toggle
git commit -m "feat(lesson-01): Add LED control functions (callable from GDB)"

# Commit 4: Final with register validation
git add lessons/01-*/src/bin/main.rs lessons/01-*/README.md
git commit -m "feat(lesson-01): Add GDB register validation, complete lesson

- Hardware-based unit testing pattern
- GDB validates GPIO_OUT_REG after LED operations
- README documents all GDB commands used"
```

**Push branches:**
```bash
git push --force-with-lease origin lesson-01
git push --force-with-lease origin lesson-02
git push -u origin lesson-03
git push -u origin lesson-04
git push -u origin lesson-05
```

---

## Post-Lesson Workflow

After lesson branches are created:

### 1. Update Progress Tracker

```markdown
## esp-hal 1.0.0 + Claude Code Curriculum Progress

- [x] Lesson 01: GPIO Basics + GDB Fundamentals (⭐⭐☆☆☆)
- [x] Lesson 02: UART CLI + Streaming Infrastructure (⭐⭐⭐☆☆)
- [x] Lesson 03: PWM + Neopixel Drivers (⭐⭐⭐☆☆)
- [x] Lesson 04: MPU6050 + State Machine (⭐⭐⭐⭐☆)
- [x] Lesson 05: Posture Monitor Device (⭐⭐⭐⭐⭐)
```

### 2. Final Hardware Validation

Each lesson branch must be tested on real hardware:
- [ ] Build without warnings
- [ ] Flash successfully
- [ ] All CLI commands work
- [ ] Hardware behaves as expected
- [ ] GDB techniques work as documented
- [ ] UART telemetry parseable
- [ ] Progressive commits tell a clear story

### 3. Documentation Review

Ensure each lesson has:
- [ ] README with wiring diagrams
- [ ] GDB debugging workflows documented
- [ ] Intentional bugs explained
- [ ] Success criteria checklist
- [ ] Expected UART output examples

### 4. Archive Cleanup (Optional)

Old lessons are preserved in `archive/lessons-old-*/` for reference:

```bash
# List archived content
ls -la archive/

# Rollback if needed (use snapshot tag)
git tag | grep pre-refactor
git checkout <tag-name>
```

---

## Key Principles

### 1. Progressive Infrastructure (Not Throwaway Code)

**Traditional approach**: Each lesson is isolated, start from scratch
**This curriculum**: Each lesson **extends** the UART CLI built in Lesson 2

By Lesson 5, you have:
- Complete CLI with 20+ commands
- Full streaming telemetry
- Hardware unit tests for every peripheral
- Real debugging infrastructure

### 2. Learn by Debugging (Not by Reading)

Every commit has **intentional bugs**:
- Lesson 02: Buffer overflow, DMA misalignment
- Lesson 03: PWM frequency wrong, Neopixel color order, RMT timing
- Lesson 04: I2C speed, axis swap, calibration overflow
- Lesson 05: Race conditions, power issues, state machine bugs

Claude Code uses GDB to **diagnose and fix**. README documents the **debugging process**.

### 3. Advanced GDB Throughout (Not Separate "Advanced" Lesson)

Use advanced techniques when needed:
- **Lesson 02**: Watchpoints (buffer overflow)
- **Lesson 03**: Python scripting (color testing), Disassembly (RMT timing)
- **Lesson 04**: Conditional breakpoints (I2C errors), Tracepoints (performance)
- **Lesson 05**: Watchpoints (race conditions), Statistical analysis (anomaly detection)

### 4. Hardware-Based Unit Testing

CLI + GDB = **automated hardware validation**:

```
> gpio.init 12
OK
# GDB validates: GPIO_ENABLE_REG bit 12 = 1 ✓

> pwm.init 12 1000
OK
# GDB validates: LEDC timer configured for 1 kHz ✓

> neo.color 255 0 0
OK
# GDB validates: RMT buffer = [0x00, 0xFF, 0x00] (GRB for red) ✓
```

**No mocks. Real hardware. Automated validation.**

### 5. Build a Real Device

Lesson 05 culminates in a **functional posture monitor**:
- Can mount on desk or laptop
- Alerts when tilted beyond threshold
- Calibratable zero orientation
- Low-power sleep mode
- **Actually useful!**

---

## Expected Timeline

| Lesson | Duration | Cumulative |
|--------|----------|------------|
| Lesson 01 | 90-120 min | 2 hrs |
| Lesson 02 | 180-240 min | 6 hrs |
| Lesson 03 | 180-240 min | 10 hrs |
| Lesson 04 | 240-300 min | 15 hrs |
| Lesson 05 | 300-420 min | 22 hrs |

**Total**: 18-30 hours (spread over 5-8 sessions)

**Recommended schedule**:
- Session 1: Lesson 01 (GPIO + GDB basics)
- Session 2: Lesson 02 Part 1 (UART + CLI)
- Session 3: Lesson 02 Part 2 (Streaming + DMA)
- Session 4: Lesson 03 (PWM + Neopixel)
- Session 5: Lesson 04 Part 1 (I2C + IMU)
- Session 6: Lesson 04 Part 2 (State machine)
- Session 7: Lesson 05 Part 1 (Integration)
- Session 8: Lesson 05 Part 2 (Debugging + polish)

---

## Completion Criteria

All 5 lessons complete when:

- [ ] All lesson branches merged to main
- [ ] CLI has 20+ commands across all peripherals
- [ ] Hardware-based unit tests validate all CLI commands
- [ ] Posture Monitor device fully functional
- [ ] All advanced GDB techniques demonstrated (watchpoints, tracepoints, Python scripting, disassembly, conditional breakpoints)
- [ ] Streaming telemetry parseable (JSON format)
- [ ] Every intentional bug documented with GDB debugging workflow

---

## Tips for Success

**Hardware setup**:
- Keep FTDI UART connected permanently (you'll use it constantly)
- Test each peripheral individually before integrating
- Document working GPIO pins in a reference file

**Debugging mindset**:
- Embrace bugs as learning opportunities
- Use GDB to understand **hardware**, not just code
- UART telemetry = continuous eyes into running firmware
- CLI = interactive testing interface (no reflashing needed!)

**Reference materials**:
- esp-hal 1.0.0 docs: https://docs.esp-rs.org/esp-hal/
- MPU6050 datasheet: https://invensense.tdk.com/products/motion-tracking/6-axis/mpu-6050/
- WS2812 timing: https://cdn-shop.adafruit.com/datasheets/WS2812.pdf
- GDB manual: https://sourceware.org/gdb/current/onlinedocs/gdb/
- LEDC (PWM): ESP32-C6 Technical Reference Manual Chapter 14

---

**This curriculum builds real-world embedded firmware skills with AI-assisted debugging workflows that didn't exist before Claude Code. By the end, you have a complete testing/debugging infrastructure and a functional device.**

# GDB Capabilities for Embedded Systems: Complete Guide

**A comprehensive exploration of GDB's power for ESP32-C6 embedded development**

---

## Table of Contents

1. [Memory Inspection & Manipulation](#1-memory-inspection--manipulation)
2. [Breakpoints & Execution Control](#2-breakpoints--execution-control)
3. [Watchpoints (Hardware Monitoring)](#3-watchpoints-hardware-monitoring)
4. [Variable Operations](#4-variable-operations)
5. [Call Stack & Frame Analysis](#5-call-stack--frame-analysis)
6. [Function Calls from Debugger](#6-function-calls-from-debugger)
7. [GDB Variables & Expressions](#7-gdb-variables--expressions)
8. [Memory Dumps & Binary Operations](#8-memory-dumps--binary-operations)
9. [Python Scripting](#9-python-scripting)
10. [Reverse Debugging](#10-reverse-debugging)
11. [Register Inspection (CPU & Peripheral)](#11-register-inspection-cpu--peripheral)
12. [Signal Handling](#12-signal-handling)
13. [Multi-threaded Debugging](#13-multi-threaded-debugging)

---

## 1. Memory Inspection & Manipulation

### Capabilities

**Read memory:**
```gdb
x/1xw 0x60091020           # Examine 1 hex word at address
x/16xb 0x60091000          # Examine 16 hex bytes
x/4tw 0x60091020           # Examine 4 binary words
x/s 0x3C000000             # Examine string
x/i $pc                    # Examine instruction at program counter
```

**Write memory:**
```gdb
set *(uint32_t*)0x60091024 = 0x1000      # Write word
set *(uint8_t*)0x60091024 = 0x12         # Write byte
set {uint32_t}0x60091024 = 0x1000        # Alternative syntax
```

**Modify with operations:**
```gdb
set *(uint32_t*)0x60091024 |= (1 << 12)  # Set bit 12
set *(uint32_t*)0x60091024 &= ~(1 << 12) # Clear bit 12
set *(uint32_t*)0x60091024 ^= (1 << 12)  # Toggle bit 12
```

### Lesson 01 Use Cases

**✅ Already Planned:**
- Inspect GPIO registers to discover missing ENABLE bit
- Write to GPIO registers to control LED

**🔥 Cool Addition - "Hardware State Inspector":**
```gdb
define show_gpio12
  printf "GPIO12 State:\n"
  printf "  ENABLE: %d\n", (*(uint32_t*)0x60091020 >> 12) & 1
  printf "  OUT:    %d\n", (*(uint32_t*)0x60091004 >> 12) & 1
  printf "  IN:     %d\n", (*(uint32_t*)0x6009103C >> 12) & 1
end
```

**Demo:** "Let's create a live GPIO dashboard in GDB"

### Future Lesson Use Cases

**Lesson 02 (UART + DMA):**
- Inspect UART FIFO registers: `x/16xb 0x60000000` (UART0 TX FIFO)
- Monitor DMA descriptor chain: `x/8xw 0x3FC88000` (DMA descriptor)
- Verify baud rate register calculations

**Lesson 03 (I2C Sensors):**
- Read I2C data register after transaction: `x/1xw 0x60013010`
- Inspect sensor FIFO: `x/32xb <sensor_buffer_addr>`
- Debug I2C state machine via status registers

---

## 2. Breakpoints & Execution Control

### Capabilities

**Basic breakpoints:**
```gdb
break main.rs:42                    # Line number
break blink_led                     # Function name
break *0x42000000                   # Address
tbreak main.rs:42                   # Temporary (one-time)
```

**Conditional breakpoints:**
```gdb
break main.rs:42 if loop_count == 100
break main.rs:42 if BLINK_SPEED < 100
break uart_send if data_len > 1024
```

**Breakpoint commands (automation):**
```gdb
break main.rs:42
commands
  silent
  print loop_count
  if loop_count > 100
    set BLINK_SPEED = 100
  end
  continue
end
```

**Execution control:**
```gdb
continue    # Resume
step        # Step into
next        # Step over
finish      # Step out
until       # Run until line
advance     # Run to location
```

### Lesson 01 Use Cases

**✅ Already Planned:**
- Simple breakpoint for observation

**🔥 Cool Addition - "Mode Switcher":**
```rust
static mut MODE: u32 = 0;  // 0=slow, 1=fast, 2=pulse, 3=SOS

loop {
    match MODE {
        0 => blink_slow(&delay),
        1 => blink_fast(&delay),
        2 => pulse_pattern(&delay),
        3 => sos_morse(&delay),
        _ => {}
    }
}
```

**GDB automation:**
```gdb
# Auto-cycle through modes every 10 seconds
break main.rs:88 if loop_count % 20 == 0
commands
  silent
  set MODE = (MODE + 1) % 4
  printf "Switched to MODE %d\n", MODE
  continue
end
```

**Demo:** "Let's make GDB automatically DJ the LED patterns"

**🔥 Cool Addition - "Performance Profiler":**
```gdb
# Measure loop timing
set $start_time = 0
set $iteration_count = 0

break main.rs:88
commands
  silent
  set $iteration_count = $iteration_count + 1
  if $iteration_count % 100 == 0
    printf "100 iterations completed\n"
  end
  continue
end
```

### Future Lesson Use Cases

**Lesson 02 (UART + DMA):**
```gdb
# Break when UART buffer is full
break uart_send if tx_buffer_count > 1000

# Break on DMA completion
break dma_interrupt_handler

# Conditional: only break on errors
break uart_error_handler if error_code != 0
```

**Lesson 04 (State Machine Debugging):**
```gdb
# Track state transitions
break state_machine_update
commands
  silent
  print old_state
  print new_state
  if new_state == STATE_ERROR
    printf "ERROR STATE ENTERED!\n"
    # Stop execution to inspect
  else
    continue
  end
end
```

---

## 3. Watchpoints (Hardware Monitoring)

### Capabilities

**Data watchpoints:**
```gdb
watch BLINK_SPEED              # Break when variable changes
watch -l BLINK_SPEED           # Watch location (not variable)
rwatch BLINK_SPEED             # Break on read
awatch BLINK_SPEED             # Break on read OR write
```

**Memory watchpoints:**
```gdb
watch *(uint32_t*)0x60091004   # Watch GPIO OUT register
watch *(uint8_t*)0x3FC88000    # Watch DMA descriptor
```

**Watchpoint commands:**
```gdb
watch BLINK_SPEED
commands
  printf "BLINK_SPEED changed from %d to %d\n", $old_val, BLINK_SPEED
  backtrace
  continue
end
```

### Lesson 01 Use Cases

**🔥 AMAZING Addition - "Hardware Change Detective":**
```gdb
# Watch GPIO OUT register
watch *(uint32_t*)0x60091004
commands
  silent
  printf "GPIO OUT changed! New value: 0x%08x\n", *(uint32_t*)0x60091004
  printf "  GPIO12 is now: %s\n", ((*(uint32_t*)0x60091004 >> 12) & 1) ? "HIGH" : "LOW"
  backtrace 1  # Show who changed it
  continue
end
```

**Demo:** "Let's make GDB tell us every time ANY code touches the GPIO - even if we don't know where that code is!"

**Use case:** "What if you had a bug where LED randomly changes? Watchpoint finds the culprit instantly."

**🔥 Cool Addition - "Variable Corruption Detector":**
```rust
static mut BLINK_SPEED: u32 = 500;
static mut DEBUG_MODE: u32 = 0;

// Somewhere, a bug corrupts BLINK_SPEED
// Watchpoint finds it immediately
```

```gdb
watch BLINK_SPEED
commands
  if BLINK_SPEED > 10000 || BLINK_SPEED < 10
    printf "⚠️  BLINK_SPEED corrupted! Value: %d\n", BLINK_SPEED
    printf "Call stack:\n"
    backtrace
    # Don't continue - stop for inspection
  else
    continue
  end
end
```

**Demo:** "Introduce a subtle memory corruption bug, watch GDB catch it in real-time"

### Future Lesson Use Cases

**Lesson 02 (UART + DMA):**
```gdb
# Detect UART FIFO overflow
watch *(uint32_t*)0x6000001C  # UART0 status register
commands
  if (*(uint32_t*)0x6000001C & (1 << 3)) != 0
    printf "UART FIFO OVERFLOW DETECTED!\n"
    # Stop execution
  else
    continue
  end
end

# Watch DMA descriptor updates
watch *(uint32_t*)0x3FC88000
commands
  printf "DMA descriptor updated\n"
  x/8xw 0x3FC88000
  continue
end
```

**Lesson 03 (Sensor Integration):**
```gdb
# Detect sensor data ready
watch *(uint32_t*)0x60013000  # I2C status
commands
  if (*(uint32_t*)0x60013000 & (1 << 7)) != 0
    printf "I2C data ready!\n"
    x/16xb 0x60013010  # Read data register
  end
  continue
end
```

---

## 4. Variable Operations

### Capabilities

**Read variables:**
```gdb
print BLINK_SPEED              # Show value
print &BLINK_SPEED             # Show address
print/x BLINK_SPEED            # Hex format
print/t BLINK_SPEED            # Binary format
print/d BLINK_SPEED            # Decimal (default)
print sizeof(BLINK_SPEED)      # Size in bytes
```

**Write variables:**
```gdb
set BLINK_SPEED = 100
set BLINK_SPEED = BLINK_SPEED * 2
set {uint32_t}0x3FC88000 = 0x12345678
```

**Array/struct inspection:**
```gdb
print buffer
print buffer[5]
print buffer[0]@16             # Print 16 elements starting at index 0
print sensor_data.temperature
print *dma_descriptor
```

### Lesson 01 Use Cases

**✅ Already Planned:**
- Read/write `BLINK_SPEED` to change timing

**🔥 Cool Addition - "Live Configuration Panel":**
```rust
static mut CONFIG: Config = Config {
    blink_speed: 500,
    pattern: 0,        // 0=blink, 1=pulse, 2=SOS
    brightness: 100,   // Future: PWM duty cycle
    enabled: 1,
};
```

```gdb
# GDB as a configuration UI
print CONFIG
set CONFIG.blink_speed = 100
set CONFIG.pattern = 2
set CONFIG.enabled = 0  # Pause blinking
```

**Demo:** "Let's build a live hardware control panel in GDB"

**🔥 Cool Addition - "Statistics Dashboard":**
```rust
static mut STATS: Statistics = Statistics {
    total_blinks: 0,
    total_runtime_ms: 0,
    errors: 0,
    mode_switches: 0,
};

// Firmware increments these
// GDB reads them
```

```gdb
define show_stats
  printf "═══════════════════════════════════\n"
  printf "  Live Statistics Dashboard\n"
  printf "═══════════════════════════════════\n"
  printf "Total Blinks:     %d\n", STATS.total_blinks
  printf "Runtime:          %d ms\n", STATS.total_runtime_ms
  printf "Errors:           %d\n", STATS.errors
  printf "Mode Switches:    %d\n", STATS.mode_switches
  printf "═══════════════════════════════════\n"
end

# Auto-refresh every second
break main.rs:88 if loop_count % 2 == 0
commands
  silent
  show_stats
  continue
end
```

**Demo:** "GDB as a live telemetry dashboard - watch stats update in real-time"

### Future Lesson Use Cases

**Lesson 02 (UART + DMA):**
```gdb
# Inspect circular buffer state
print uart_buffer
print uart_buffer.head
print uart_buffer.tail
print uart_buffer.data[0]@32  # Print 32 bytes

# Inject test data
set uart_buffer.data[0] = 0xAB
set uart_buffer.data[1] = 0xCD
```

**Lesson 03 (Sensor Fusion):**
```gdb
# Read sensor struct
print mpu9250_data
print mpu9250_data.accel_x
print mpu9250_data.accel_y
print mpu9250_data.accel_z

# Manually inject test values
set mpu9250_data.accel_x = 1000
set mpu9250_data.accel_y = -500
```

---

## 5. Call Stack & Frame Analysis

### Capabilities

**Stack inspection:**
```gdb
backtrace              # Full call stack (alias: bt)
backtrace 5            # Top 5 frames
bt full                # Include local variables
where                  # Alias for backtrace
```

**Frame navigation:**
```gdb
frame 0                # Select frame 0 (current)
frame 2                # Select frame 2
up                     # Move up stack
down                   # Move down stack
info frame             # Current frame details
info args              # Function arguments
info locals            # Local variables
```

### Lesson 01 Use Cases

**🔥 Cool Addition - "Bug Hunt Game":**

Create intentionally nested code with a subtle bug:

```rust
fn calculate_blink_delay(mode: u32, base_speed: u32) -> u32 {
    match mode {
        0 => base_speed,
        1 => base_speed / 2,
        2 => pulse_delay(base_speed),
        _ => base_speed,
    }
}

fn pulse_delay(speed: u32) -> u32 {
    // Bug: forgot to handle speed < 10
    speed / 10  // Causes very fast blinking when speed=50
}

fn blink_led(delay: &Delay) {
    let delay_ms = calculate_blink_delay(MODE, BLINK_SPEED);
    delay.delay_millis(delay_ms);
}

fn main() -> ! {
    loop {
        blink_led(&delay);
    }
}
```

**GDB investigation:**
```gdb
# Notice LED blinks too fast
break blink_led
continue

# Inspect call stack
backtrace
# #0 blink_led at main.rs:45
# #1 main at main.rs:88

# Check local variables
info locals
# delay_ms = 5  (too fast!)

# Move up stack to see why
up
info locals
# MODE = 2
# BLINK_SPEED = 50

# Step into calculate_blink_delay
step
step
# Now in pulse_delay - see the bug!
print speed  # 50
print speed / 10  # 5 (aha!)
```

**Demo:** "Let's play detective - LED is too fast, what's wrong? Use call stack to trace the problem."

**🔥 Cool Addition - "Performance Profiling":**
```gdb
# Count function calls
set $blink_count = 0
set $calculate_count = 0

break blink_led
commands
  silent
  set $blink_count = $blink_count + 1
  continue
end

break calculate_blink_delay
commands
  silent
  set $calculate_count = $calculate_count + 1
  continue
end

# After running:
print $blink_count
print $calculate_count
```

### Future Lesson Use Cases

**Lesson 02 (UART Panic Debugging):**
```gdb
# Catch panic in UART ISR
catch panic

# When panic occurs:
backtrace full
# #0  panic_handler
# #1  uart_isr at uart.rs:145
# #2  interrupt_handler
# ...

# Inspect each frame
frame 1
info locals
# Shows: buffer_overflow = true
```

---

## 6. Function Calls from Debugger

### Capabilities

**Call Rust functions:**
```gdb
call delay.delay_millis(1000)       # Call method
call led.set_high()                 # Call method
call led.toggle()                   # Call method
call blink_led(&delay, 500)         # Call function
print calculate_blink_delay(2, 500) # Call and show return value
```

**Call with side effects:**
```gdb
# Change hardware state by calling firmware functions
call gpio_init()
call uart_send_byte(0xAB)
call dma_start_transfer()
```

### Lesson 01 Use Cases

**🔥 MIND-BLOWING Addition - "Remote Control LED":**

```rust
fn led_on() {
    unsafe {
        *(0x60091008 as *mut u32) = 0x1000;
    }
}

fn led_off() {
    unsafe {
        *(0x6009100C as *mut u32) = 0x1000;
    }
}

fn led_toggle() {
    static mut STATE: bool = false;
    unsafe {
        if STATE {
            led_off();
        } else {
            led_on();
        }
        STATE = !STATE;
    }
}
```

**GDB as remote control:**
```gdb
# Pause firmware
Ctrl-C

# Now YOU control the LED
call led_on()
# LED turns on!

call led_off()
# LED turns off!

call led_toggle()
# LED toggles!

# Resume firmware
continue
```

**Demo:** "Let's pause the firmware and take over LED control from GDB. We're calling Rust functions from the debugger!"

**🔥 Cool Addition - "Interactive Mode Tester":**
```rust
fn test_mode(mode: u32) {
    unsafe { MODE = mode; }
    for _ in 0..5 {
        match mode {
            0 => blink_slow(&DELAY),
            1 => blink_fast(&DELAY),
            2 => pulse_pattern(&DELAY),
            3 => sos_morse(&DELAY),
            _ => {}
        }
    }
}
```

```gdb
# Pause firmware
Ctrl-C

# Test each mode without changing code or reflashing
call test_mode(0)  # See slow blink 5 times
call test_mode(1)  # See fast blink 5 times
call test_mode(2)  # See pulse pattern 5 times
call test_mode(3)  # See SOS morse 5 times

# Resume normal operation
continue
```

**Demo:** "Let's test all LED modes without recompiling. Just call the test function from GDB!"

### Future Lesson Use Cases

**Lesson 02 (UART Testing):**
```gdb
# Send test data without writing test code
call uart_send_string("Hello from GDB!\n")
call uart_send_byte(0xAB)
call uart_flush()

# Test error handling
call uart_inject_error(UART_ERROR_OVERFLOW)
```

**Lesson 03 (Sensor Debugging):**
```gdb
# Trigger sensor read
call mpu9250_read()
print mpu9250_data

# Inject test data
call mpu9250_inject_data(1000, 500, -200)
call state_machine_update()  # Process injected data
```

---

## 7. GDB Variables & Expressions

### Capabilities

**GDB convenience variables:**
```gdb
set $gpio12 = 12
set $mask = 1 << $gpio12
set *(uint32_t*)0x60091024 = $mask

# Persistent across commands
print/x $mask
# $1 = 0x1000
```

**Complex expressions:**
```gdb
# Bit math calculator
print/t (1 << 12)           # Binary: 1000000000000
print/x (1 << 12)           # Hex: 0x1000
print/d (1 << 12)           # Decimal: 4096

# Register manipulation
set $reg = *(uint32_t*)0x60091004
set $reg = $reg | (1 << 5)  # Set bit 5
set $reg = $reg & ~(1 << 3) # Clear bit 3
```

**Arrays and loops:**
```gdb
set $i = 0
while $i < 16
  print buffer[$i]
  set $i = $i + 1
end
```

### Lesson 01 Use Cases

**🔥 Cool Addition - "Bit Math Workshop":**

```gdb
# Teach bit manipulation interactively
set $gpio_num = 12
print/t (1 << $gpio_num)
# $1 = 1000000000000

# Build complex masks
set $mask_12 = (1 << 12)
set $mask_13 = (1 << 13)
set $both = $mask_12 | $mask_13
print/t $both
# $2 = 11000000000000

# Apply to hardware
set *(uint32_t*)0x60091024 = $both  # Enable GPIO 12 and 13
```

**Demo:** "GDB as your bit math calculator - learn while doing"

**🔥 Cool Addition - "GPIO Pin Scanner":**

```gdb
# Scan all GPIO pins to see which are enabled
set $pin = 0
printf "GPIO Pin Status:\n"
while $pin < 24
  set $bit = (*(uint32_t*)0x60091020 >> $pin) & 1
  if $bit == 1
    printf "  GPIO%d: ENABLED\n", $pin
  end
  set $pin = $pin + 1
end
```

**Demo:** "Let's scan all 24 GPIO pins to see which ones are configured"

### Future Lesson Use Cases

**Lesson 02 (UART Buffer Analysis):**
```gdb
# Analyze buffer contents
set $i = 0
set $ascii_count = 0
while $i < 256
  if uart_buffer[$i] >= 32 && uart_buffer[$i] <= 126
    set $ascii_count = $ascii_count + 1
  end
  set $i = $i + 1
end
print $ascii_count
# Shows how much of buffer is printable ASCII
```

---

## 8. Memory Dumps & Binary Operations

### Capabilities

**Dump memory to file:**
```gdb
dump memory gpio_regs.bin 0x60091000 0x60091100
dump binary memory uart_buffer.bin buffer_addr buffer_addr+1024
dump ihex memory firmware.hex 0x42000000 0x42010000
```

**Restore memory from file:**
```gdb
restore gpio_regs.bin binary 0x60091000
```

**Compare memory:**
```gdb
# Save before
dump memory before.bin 0x60091000 0x60091100
# ... run code ...
# Save after
dump memory after.bin 0x60091000 0x60091100
# Compare externally with diff/hexdump
```

### Lesson 01 Use Cases

**🔥 Cool Addition - "Peripheral State Snapshots":**

```gdb
# Capture GPIO state before and after
define save_gpio_state
  dump binary memory /tmp/gpio_state_$arg0.bin 0x60091000 0x60091100
  printf "Saved GPIO state to /tmp/gpio_state_%s.bin\n", $arg0
end

# Usage:
save_gpio_state "before"
# ... run some code ...
save_gpio_state "after"

# Compare:
shell hexdump -C /tmp/gpio_state_before.bin > /tmp/before.hex
shell hexdump -C /tmp/gpio_state_after.bin > /tmp/after.hex
shell diff /tmp/before.hex /tmp/after.hex
```

**Demo:** "Let's capture hardware state snapshots and diff them - see exactly what changed!"

### Future Lesson Use Cases

**Lesson 02 (UART Buffer Forensics):**
```gdb
# Dump UART buffer for analysis
dump binary memory uart_capture.bin &uart_buffer sizeof(uart_buffer)

# Analyze externally:
# hexdump -C uart_capture.bin
# strings uart_capture.bin
# python3 analyze_uart.py uart_capture.bin
```

**Lesson 03 (Sensor Data Logging):**
```gdb
# Capture 1000 sensor readings
set $i = 0
while $i < 1000
  call mpu9250_read()
  dump append binary memory sensor_log.bin &mpu9250_data sizeof(mpu9250_data)
  set $i = $i + 1
end

# Analyze with Python/MATLAB
```

---

## 9. Python Scripting

### Capabilities

**Execute Python in GDB:**
```gdb
python print("Hello from Python!")
python gdb.execute("info registers")
```

**Python scripts:**
```python
# In GDB:
python
import gdb

def show_gpio_state():
    # Read ENABLE register
    enable = int(gdb.parse_and_eval("*(uint32_t*)0x60091020"))
    # Read OUT register
    out = int(gdb.parse_and_eval("*(uint32_t*)0x60091004"))

    print("GPIO State:")
    for pin in range(24):
        enabled = (enable >> pin) & 1
        value = (out >> pin) & 1
        if enabled:
            print(f"  GPIO{pin}: {'HIGH' if value else 'LOW'}")

# Register as GDB command
class ShowGPIO(gdb.Command):
    def __init__(self):
        super().__init__("show-gpio", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        show_gpio_state()

ShowGPIO()
end

# Now use:
show-gpio
```

**Pretty printers:**
```python
python
class ConfigPrinter:
    def __init__(self, val):
        self.val = val

    def to_string(self):
        speed = self.val['blink_speed']
        pattern = self.val['pattern']
        enabled = self.val['enabled']

        patterns = ['BLINK', 'PULSE', 'SOS']
        pattern_name = patterns[pattern] if pattern < 3 else 'UNKNOWN'

        return f"Config {{ speed={speed}ms, pattern={pattern_name}, enabled={enabled} }}"

gdb.pretty_printers.append(lambda val: ConfigPrinter(val) if str(val.type) == 'Config' else None)
end
```

### Lesson 01 Use Cases

**🔥 ADVANCED Addition - "Custom GDB Commands":**

```python
python
import gdb

class LEDControl(gdb.Command):
    """Control LED from GDB: led on|off|toggle|blink [speed]"""

    def __init__(self):
        super().__init__("led", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        args = arg.split()
        if not args:
            print("Usage: led on|off|toggle|blink [speed]")
            return

        cmd = args[0]

        if cmd == "on":
            gdb.execute("set *(uint32_t*)0x60091008 = 0x1000")
            print("LED ON")
        elif cmd == "off":
            gdb.execute("set *(uint32_t*)0x6009100C = 0x1000")
            print("LED OFF")
        elif cmd == "toggle":
            out = int(gdb.parse_and_eval("*(uint32_t*)0x60091004"))
            if (out >> 12) & 1:
                gdb.execute("set *(uint32_t*)0x6009100C = 0x1000")
                print("LED OFF")
            else:
                gdb.execute("set *(uint32_t*)0x60091008 = 0x1000")
                print("LED ON")
        elif cmd == "blink":
            speed = int(args[1]) if len(args) > 1 else 500
            gdb.execute(f"set BLINK_SPEED = {speed}")
            print(f"Blink speed set to {speed}ms")

LEDControl()
end

# Usage:
led on
led off
led toggle
led blink 100
```

**Demo:** "Let's create custom GDB commands for LED control - make GDB feel like a hardware control app!"

### Future Lesson Use Cases

**Lesson 02 (UART Data Visualizer):**
```python
python
import gdb
import matplotlib.pyplot as plt

class UARTPlot(gdb.Command):
    """Plot UART buffer contents"""

    def invoke(self, arg, from_tty):
        # Read buffer
        data = []
        for i in range(256):
            val = int(gdb.parse_and_eval(f"uart_buffer[{i}]"))
            data.append(val)

        # Plot
        plt.plot(data)
        plt.title("UART Buffer Contents")
        plt.xlabel("Index")
        plt.ylabel("Value")
        plt.show()

UARTPlot()
end
```

**Lesson 03 (Sensor Data Analysis):**
```python
python
import gdb
import numpy as np

class SensorStats(gdb.Command):
    """Analyze sensor data statistics"""

    def invoke(self, arg, from_tty):
        # Read accelerometer data
        accel_x = []
        for i in range(100):
            gdb.execute("call mpu9250_read()")
            x = float(gdb.parse_and_eval("mpu9250_data.accel_x"))
            accel_x.append(x)

        # Statistics
        print(f"Mean:   {np.mean(accel_x):.2f}")
        print(f"StdDev: {np.std(accel_x):.2f}")
        print(f"Min:    {np.min(accel_x):.2f}")
        print(f"Max:    {np.max(accel_x):.2f}")

SensorStats()
end
```

---

## 10. Reverse Debugging

### Capabilities

**Record & replay:**
```gdb
record              # Start recording execution
reverse-continue    # Run backwards to previous breakpoint
reverse-step        # Step backwards
reverse-next        # Step over backwards
reverse-finish      # Run backwards out of function
```

**Time-travel debugging:**
```gdb
# Run forward
continue
# Hit bug at line 100

# Go back in time
reverse-continue
# Now at previous breakpoint (line 50)

# Step forward slowly
step
step
step
# Watch bug develop
```

### Lesson 01 Use Cases

**⚠️ Limited support on embedded** - Requires simulator or QEMU
- ESP32-C6 with probe-rs: **NOT supported**
- ESP32-C6 with QEMU: **Supported**

**Could work in future "Simulation Mode" lesson:**
```gdb
# In QEMU simulation:
record
continue
# Bug occurs

reverse-continue
# Go back to before bug
reverse-step
# See what caused it
```

### Future Lesson Use Cases

**Lesson 0X (QEMU Simulation):**
- Teach reverse debugging in simulation
- Then contrast with hardware debugging limitations
- Show why good logging is critical on real hardware

---

## 11. Register Inspection (CPU & Peripheral)

### Capabilities

**CPU registers:**
```gdb
info registers              # All registers
info all-registers          # Include FPU, special
print/x $pc                 # Program counter
print/x $sp                 # Stack pointer
print/x $ra                 # Return address (RISC-V)
set $t0 = 0x1234           # Modify temp register
```

**Peripheral registers (memory-mapped):**
```gdb
# GPIO
x/1xw 0x60091020           # ENABLE
x/1xw 0x60091004           # OUT
x/1xw 0x6009103C           # IN

# UART
x/1xw 0x60000000           # UART0_FIFO
x/1xw 0x6000001C           # UART0_STATUS

# DMA
x/8xw 0x3FC88000           # DMA descriptor
```

### Lesson 01 Use Cases

**✅ Already planned** - GPIO register inspection

**🔥 Cool Addition - "CPU State Inspector":**
```gdb
define show_cpu
  printf "CPU State:\n"
  printf "  PC:  0x%08x\n", $pc
  printf "  SP:  0x%08x\n", $sp
  printf "  RA:  0x%08x\n", $ra

  # RISC-V specific
  printf "  T0:  0x%08x\n", $t0
  printf "  T1:  0x%08x\n", $t1
  printf "  A0:  0x%08x\n", $a0
end
```

### Future Lesson Use Cases

**All lessons:** Inspect peripheral registers to understand hardware state

---

## 12. Signal Handling

### Capabilities

**Catch signals:**
```gdb
catch signal SIGINT         # Catch Ctrl-C
catch signal SIGSEGV        # Catch segfault
catch throw                 # Catch C++ exceptions
catch panic                 # Catch Rust panics (with debuginfo)
```

**Signal commands:**
```gdb
catch panic
commands
  backtrace full
  print panic_message
  # Don't continue - stop for inspection
end
```

### Lesson 01 Use Cases

**🔥 Cool Addition - "Panic Inspector":**

```rust
fn divide_delay(speed: u32, divisor: u32) -> u32 {
    if divisor == 0 {
        panic!("Division by zero!");
    }
    speed / divisor
}
```

```gdb
catch panic
commands
  printf "PANIC CAUGHT!\n"
  backtrace full
  # Show all local variables at panic site
end
```

**Demo:** "Let's trigger a panic and see GDB catch it, show us exactly where and why"

### Future Lesson Use Cases

**All lessons:** Catch panics for debugging

---

## 13. Multi-threaded Debugging

### Capabilities

**Thread inspection:**
```gdb
info threads               # List all threads
thread 2                   # Switch to thread 2
thread apply all bt        # Backtrace all threads
thread apply 1-3 print $pc # Run command on threads 1-3
```

**Thread-specific breakpoints:**
```gdb
break uart_isr thread 2    # Only break in thread 2
```

### Lesson 01 Use Cases

**⚠️ Not applicable** - No threading in bare-metal

### Future Lesson Use Cases

**Lesson 0X (RTOS/Embassy):**
- Debug async tasks
- Inspect task states
- Find deadlocks

---

---

---

## Quick Reference Tables

### ESP32-C6 Peripheral Register Map

| Peripheral | Base Address | Key Registers |
|-----------|--------------|---------------|
| GPIO | 0x60091000 | ENABLE (0x20), OUT (0x04), IN (0x3C) |
| UART0 | 0x60000000 | FIFO (0x00), STATUS (0x1C), CONF (0x20) |
| UART1 | 0x60010000 | Same offsets as UART0 |
| I2C0 | 0x60013000 | DATA (0x10), STATUS (0x08), CMD (0x58) |
| I2C1 | 0x60027000 | Same offsets as I2C0 |
| SPI1 | 0x60002000 | CMD (0x00), ADDR (0x04), W0-W15 (0x58+) |
| SPI2 | 0x60024000 | Same offsets as SPI1 |
| ADC | 0x60040000 | DATA (0x00), CONF (0x04) |
| LEDC | 0x60019000 | DUTY (varies by channel) |

### GDB Command Quick Reference

**Memory:**
```gdb
x/[count][format][size] <addr>
  count: number of units
  format: x(hex) d(dec) t(bin) s(string) i(instruction)
  size: b(byte) h(half) w(word) g(giant)

set *(type*)addr = value
dump [format] memory file start_addr end_addr
restore file [binary] offset
```

**Breakpoints:**
```gdb
break location [if condition]
tbreak location                # Temporary
watch expr [if condition]      # Data watchpoint
rwatch / awatch               # Read/access watchpoint
commands ... end              # Breakpoint actions
```

**Execution:**
```gdb
continue / c
step / s                      # Step into
next / n                      # Step over
finish                        # Step out
until location                # Run until
advance location              # Run to
call function(args)           # Execute function
```

**Inspection:**
```gdb
print[/format] expr
print/x $var                  # Hex
print/t $var                  # Binary
print/d $var                  # Decimal
backtrace [count]             # Call stack
info frame                    # Current frame
info locals                   # Local variables
info args                     # Function arguments
info registers                # CPU registers
```

**Python:**
```gdb
python <code>
python ... end               # Multi-line
python gdb.execute("cmd")
python gdb.parse_and_eval("expr")
```

---

**See also:**
- Executive Summary: `GDB_EXECUTIVE_SUMMARY.md`
- Lesson Plans: `GDB_LESSON_PLANS.md`

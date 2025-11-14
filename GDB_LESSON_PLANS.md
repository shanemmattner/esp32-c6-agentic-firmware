# GDB Lesson Plans: Detailed Breakdown

**Complete lesson-by-lesson implementation guide**

---

## Lesson 01: GPIO + GDB Fundamentals

### Overview
**Peripheral:** GPIO (LED control)
**Duration:** 60-90 minutes
**Complexity:** ⭐⭐☆☆☆
**Hardware:** ESP32-C6 + LED + 220Ω resistor

### GDB Techniques (3)
1. **Memory inspection/writes** - Read/write GPIO registers
2. **GDB variables** - Bit math calculator, expressions
3. **Function calls** - Execute firmware functions from GDB

### Learning Objectives
- Understand memory-mapped I/O
- Use GDB to inspect hardware registers
- Control hardware without firmware code
- Call Rust functions from debugger

### Commit Structure

**Commit 1: "Broken Firmware"**
- LED blink code that doesn't work
- Missing GPIO ENABLE configuration
- Investigation with GDB reveals the issue

**Commit 2: "GDB Register Control"**
- Remove LED code, use GDB to control
- Teach bit math with GDB variables
- Direct register manipulation

**Commit 3: "Function Calls"**
- Add led_on(), led_off(), led_toggle() functions
- Call from GDB while firmware paused
- Mind-blowing "remote control" moment

### Demo Scripts

**GDB Bit Math:**
```gdb
set $gpio = 12
set $mask = 1 << $gpio
print/x $mask  # 0x1000
print/t $mask  # Binary: 1000000000000

# Apply to hardware
set *(uint32_t*)0x60091024 = $mask  # Enable
set *(uint32_t*)0x60091008 = $mask  # LED ON
```

**Function Calls:**
```gdb
# Pause firmware
Ctrl-C

# Control LED from GDB
call led_on()    # LED turns on!
call led_off()   # LED turns off!
call led_toggle()  # LED toggles!

# Resume firmware
continue
```

### Why These 3 Techniques
- **Memory ops** - Foundation for everything
- **GDB variables** - Immediately useful for all bit manipulation
- **Function calls** - Most impressive, shows GDB's true power

### Skip for Later
- Watchpoints → Save for Lesson 02 (async debugging)
- Python scripting → Save for Lesson 04 (advanced)
- Call stack → Save for Lesson 02 (panic debugging)

---

## Lesson 02: UART + DMA

### Overview
**Peripheral:** UART0 with DMA streaming
**Duration:** 90-120 minutes
**Complexity:** ⭐⭐⭐☆☆
**Hardware:** ESP32-C6 + FTDI UART adapter

### GDB Techniques (3)
1. **Watchpoints** - Break when UART FIFO overflows
2. **Conditional breakpoints** - Only break on errors
3. **Call stack** - Debug panic in ISR

### Learning Objectives
- Debug asynchronous hardware
- Catch buffer overflows in real-time
- Analyze ISR panics
- Understand circular buffers

### Peripheral Details
- UART0 @ 921600 baud
- DMA circular buffer (1024 bytes)
- High-speed variable streaming (100 vars/sec)
- Error detection (overflow, framing)

### Demo Scripts

**Watchpoint on UART Status:**
```gdb
# Break when FIFO overflows
watch *(uint32_t*)0x6000001C
commands
  set $status = *(uint32_t*)0x6000001C
  if ($status & (1 << 3)) != 0
    printf "⚠️  UART FIFO OVERFLOW!\n"
    backtrace
    # Stop for inspection
  else
    continue
  end
end
```

**Conditional Breakpoint:**
```gdb
# Only break on errors, skip normal operation
break uart_isr if error_flags != 0
```

**Panic Analysis:**
```gdb
catch panic
commands
  printf "PANIC in ISR!\n"
  backtrace full
  print uart_buffer
  print dma_descriptor
end
```

### Why These 3 Techniques
- **Watchpoints** - Shine with async hardware
- **Conditionals** - Essential for ISR debugging (avoid breakpoint spam)
- **Call stack** - Critical for panic analysis

---

## Lesson 03: I2C + IMU Sensor

### Overview
**Peripheral:** I2C1 + MPU9250 (9-axis IMU)
**Duration:** 90-120 minutes
**Complexity:** ⭐⭐⭐⭐☆
**Hardware:** ESP32-C6 + MPU9250 breakout

### GDB Techniques (2)
1. **Memory dumps** - Capture sensor data to file
2. **Variable injection** - Test sensor fusion algorithms

### Learning Objectives
- Capture large datasets for analysis
- Inject test data for algorithm debugging
- External data analysis with Python/MATLAB
- Debug sensor fusion calculations

### Peripheral Details
- MPU9250 over I2C1 @ 400 kHz
- 9-axis data (3-axis accel, gyro, mag)
- FIFO buffering (1024 samples)
- Sensor fusion (quaternions, Euler angles)

### Demo Scripts

**Capture 1000 Readings:**
```gdb
# Capture sensor data to binary file
set $i = 0
while $i < 1000
  call mpu9250_read()
  dump append binary memory sensor_data.bin &accel_data sizeof(Vector3)
  set $i = $i + 1
  if $i % 100 == 0
    printf "Captured %d samples...\n", $i
  end
end

printf "Done! Analyze with: python3 plot_sensor.py sensor_data.bin\n"
```

**Inject Test Data:**
```gdb
# Test sensor fusion with known inputs
set accel_x = 1000   # 1g upward
set accel_y = 0
set accel_z = 0
call update_sensor_fusion()
print quaternion  # Check output

# Test edge case: free fall
set accel_x = 0
set accel_y = 0
set accel_z = 0
call update_sensor_fusion()
print quaternion  # Should detect free fall
```

**Python Analysis:**
```python
# plot_sensor.py
import numpy as np
import matplotlib.pyplot as plt

data = np.fromfile('sensor_data.bin', dtype=np.float32)
data = data.reshape(-1, 3)  # 3 axes

plt.plot(data[:, 0], label='X')
plt.plot(data[:, 1], label='Y')
plt.plot(data[:, 2], label='Z')
plt.legend()
plt.title('Accelerometer Data')
plt.xlabel('Sample')
plt.ylabel('Acceleration (mg)')
plt.show()
```

### Why These 2 Techniques
- **Memory dumps** - Essential for data analysis
- **Variable injection** - Perfect for testing complex algorithms

---

## Lesson 04: SPI + SD Card

### Overview
**Peripheral:** SPI1 + SD card filesystem
**Duration:** 120-150 minutes
**Complexity:** ⭐⭐⭐⭐⭐
**Hardware:** ESP32-C6 + SD card module

### GDB Techniques (2)
1. **Python scripting** - Custom GDB commands
2. **Memory dumps** - Inspect FAT32 structures

### Learning Objectives
- Create custom GDB commands
- Professional debugging workflow
- Filesystem debugging
- Binary data inspection

### Peripheral Details
- SPI1 @ 20 MHz
- SD card in SPI mode
- FAT32 filesystem
- Read/write sectors
- Directory operations

### Demo Scripts

**Custom SD Card Commands:**
```python
# In GDB Python console
python

import gdb

class SDCard(gdb.Command):
    """SD card operations: sd read|write|ls|cat"""

    def __init__(self):
        super().__init__("sd", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        args = arg.split()
        if not args:
            print("Usage: sd read|write|ls|cat")
            return

        cmd = args[0]

        if cmd == "read":
            sector = int(args[1])
            gdb.execute(f"call sd_read_sector({sector})")
            gdb.execute("x/128xw sector_buffer")

        elif cmd == "ls":
            gdb.execute("call sd_list_dir()")
            # Print directory entries
            count = int(gdb.parse_and_eval("dir_entry_count"))
            for i in range(count):
                name = gdb.parse_and_eval(f"dir_entries[{i}].name")
                size = gdb.parse_and_eval(f"dir_entries[{i}].size")
                print(f"{name.string():20s} {size:10d} bytes")

        elif cmd == "cat":
            filename = args[1]
            gdb.execute(f'call sd_read_file("{filename}")')
            gdb.execute("x/s file_buffer")

        elif cmd == "write":
            filename = args[1]
            data = ' '.join(args[2:])
            gdb.execute(f'call sd_write_file("{filename}", "{data}")')
            print(f"Wrote to {filename}")

SDCard()
end
```

**Usage:**
```gdb
# List files on SD card
sd ls

# Read file
sd cat README.txt

# Read raw sector
sd read 42

# Write file
sd write test.txt Hello from GDB!
```

**FAT32 Debugging:**
```gdb
# Inspect FAT32 boot sector
call sd_read_sector(0)
x/128xw sector_buffer

# Look for FAT32 signature
x/4xb sector_buffer+510  # Should be 0x55 0xAA
```

### Why These 2 Techniques
- **Python scripting** - Professional tooling, huge productivity boost
- **Memory dumps** - Critical for filesystem debugging

---

## Lesson 05: PWM + ADC

### Overview
**Peripheral:** LEDC PWM + ADC (light sensor)
**Duration:** 60-90 minutes
**Complexity:** ⭐⭐⭐☆☆
**Hardware:** ESP32-C6 + photoresistor + LED

### GDB Techniques (2)
1. **Statistics dashboard** - Live telemetry display
2. **Automated tuning** - GDB implements control loop

### Learning Objectives
- Real-time control systems
- Automatic parameter tuning
- Feedback loops with GDB
- Live data visualization

### Peripheral Details
- LEDC PWM @ 5 kHz (LED brightness)
- ADC @ 12-bit (light sensor)
- Closed-loop brightness control
- PID-like algorithm

### Demo Scripts

**Automated Brightness Control:**
```gdb
# GDB implements a control loop!
define auto_brightness
  set $running = 1
  while $running
    # Read ADC
    set $adc = *(uint32_t*)0x60040000

    # Calculate PWM duty cycle (0-100%)
    set $target_brightness = 2048  # Target ADC value
    set $error = $target_brightness - $adc
    set $pwm_adjustment = $error / 20

    # Constrain to 0-100%
    set $current_pwm = pwm_duty_cycle
    set $new_pwm = $current_pwm + $pwm_adjustment
    if $new_pwm < 0
      set $new_pwm = 0
    end
    if $new_pwm > 100
      set $new_pwm = 100
    end

    # Apply
    call set_pwm_duty($new_pwm)

    # Display
    printf "ADC: %4d, Target: 2048, Error: %+5d, PWM: %3d%%\n", \
           $adc, $error, $new_pwm

    # Update rate
    shell sleep 0.1
  end
end

# Run it
auto_brightness

# Stop with Ctrl-C, then:
set $running = 0
```

**Statistics Dashboard:**
```gdb
define show_stats
  printf "╔════════════════════════════════════╗\n"
  printf "║  Light Control Statistics          ║\n"
  printf "╠════════════════════════════════════╣\n"
  printf "║  ADC Value:    %4d               ║\n", *(uint32_t*)0x60040000
  printf "║  PWM Duty:     %3d%%              ║\n", pwm_duty_cycle
  printf "║  Target:       2048               ║\n"
  printf "║  Error:        %+5d              ║\n", 2048 - *(uint32_t*)0x60040000
  printf "║  Loop Count:   %6d            ║\n", control_loop_count
  printf "╚════════════════════════════════════╝\n"
end

# Auto-refresh
break main.rs:88 if loop_count % 10 == 0
commands
  silent
  show_stats
  continue
end
```

### Why These 2 Techniques
- Shows GDB for control systems
- Automated feedback loop is impressive
- Real-time visualization

---

## Lesson 06: Multi-Peripheral Integration

### Overview
**Peripherals:** GPIO + UART + I2C + SPI (simultaneously)
**Duration:** 120-180 minutes
**Complexity:** ⭐⭐⭐⭐⭐
**Hardware:** ESP32-C6 + all previous peripherals

### GDB Techniques (3)
1. **All previous techniques** - Combined usage
2. **Multi-watchpoints** - Track peripheral interactions
3. **System orchestration** - Python control scripts

### Learning Objectives
- Coordinate multiple peripherals
- Debug complex systems
- Advanced GDB workflows
- Professional debugging strategies

### System Architecture
- **GPIO:** Status LED (system state indicator)
- **UART:** High-speed data streaming (sensor telemetry)
- **I2C:** MPU9250 sensor readings (100 Hz)
- **SPI:** SD card data logging (continuous)

### Demo Scripts

**System Orchestration:**
```python
python

import gdb

class SystemControl(gdb.Command):
    """Control entire sensor logging system: system start|stop|status"""

    def __init__(self):
        super().__init__("system", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        if arg == "start":
            print("Starting sensor logging system...")
            gdb.execute("call led_blink_fast()")  # Status: starting
            gdb.execute("call uart_start_stream()")
            gdb.execute("call mpu9250_enable()")
            gdb.execute("call sd_open_log_file()")
            gdb.execute("call led_on()")  # Status: running
            print("✅ System started!")

        elif arg == "stop":
            print("Stopping system...")
            gdb.execute("call led_blink_slow()")  # Status: stopping
            gdb.execute("call sd_close_file()")
            gdb.execute("call mpu9250_disable()")
            gdb.execute("call uart_stop_stream()")
            gdb.execute("call led_off()")  # Status: stopped
            print("✅ System stopped!")

        elif arg == "status":
            print("System Status:")
            print("-" * 40)

            # UART
            uart_active = int(gdb.parse_and_eval("uart_is_active()"))
            print(f"UART:  {'🟢 Active' if uart_active else '🔴 Inactive'}")

            # I2C
            i2c_active = int(gdb.parse_and_eval("mpu9250_is_enabled()"))
            print(f"I2C:   {'🟢 Active' if i2c_active else '🔴 Inactive'}")

            # SPI
            sd_active = int(gdb.parse_and_eval("sd_is_open()"))
            print(f"SPI:   {'🟢 Active' if sd_active else '🔴 Inactive'}")

            # Statistics
            sample_count = int(gdb.parse_and_eval("total_samples"))
            bytes_logged = int(gdb.parse_and_eval("bytes_written"))
            print("-" * 40)
            print(f"Samples:     {sample_count}")
            print(f"Bytes logged: {bytes_logged}")

        elif arg == "dump":
            print("Dumping sensor data...")
            gdb.execute("dump binary memory sensor_dump.bin &sensor_buffer 4096")
            print("✅ Saved to sensor_dump.bin")

SystemControl()
end
```

**Usage:**
```gdb
system start     # Start logging
system status    # Check status
system stop      # Stop logging
system dump      # Save data
```

### Why These 3 Techniques
- Demonstrates professional workflow
- Shows GDB's power for complex systems
- Impressive orchestration demo

---

## Lesson 07: Production Debugging

### Overview
**Focus:** Real-world debugging scenarios
**Duration:** 90-120 minutes
**Complexity:** ⭐⭐⭐⭐⭐
**Hardware:** ESP32-C6 (reuse from previous lessons)

### GDB Techniques (3)
1. **Signal handling** - Catch panics, analyze crashes
2. **Post-mortem analysis** - Memory dumps from crashes
3. **Performance profiling** - Find bottlenecks

### Learning Objectives
- Debug production firmware
- Analyze crashes without reproduction
- Performance optimization
- Real-world scenarios

### Scenarios

**Scenario 1: Panic in ISR**
```gdb
catch panic
commands
  printf "╔══════════════════════════════════════╗\n"
  printf "║  PANIC DETECTED!                     ║\n"
  printf "╚══════════════════════════════════════╝\n"

  backtrace full

  # Save crash dump
  dump binary memory crash_dump.bin 0x3C000000 0x3C080000

  # Print panic info
  print panic_message
  print panic_location

  printf "\nCrash dump saved to crash_dump.bin\n"
end
```

**Scenario 2: Performance Profiling**
```gdb
# Count function calls
set $main_loop = 0
set $i2c_reads = 0
set $spi_writes = 0
set $uart_sends = 0

break main.rs:88
commands
  silent
  set $main_loop = $main_loop + 1
  continue
end

break mpu9250_read
commands
  silent
  set $i2c_reads = $i2c_reads + 1
  continue
end

break sd_write
commands
  silent
  set $spi_writes = $spi_writes + 1
  continue
end

break uart_send
commands
  silent
  set $uart_sends = $uart_sends + 1
  continue
end

# Run for 10 seconds, then:
printf "Performance Profile (10 seconds):\n"
printf "  Main loops:  %d (%d Hz)\n", $main_loop, $main_loop / 10
printf "  I2C reads:   %d (%d Hz)\n", $i2c_reads, $i2c_reads / 10
printf "  SPI writes:  %d (%d Hz)\n", $spi_writes, $spi_writes / 10
printf "  UART sends:  %d (%d Hz)\n", $uart_sends, $uart_sends / 10
printf "\nRatios:\n"
printf "  I2C per loop: %.2f\n", (float)$i2c_reads / $main_loop
printf "  SPI per loop: %.2f\n", (float)$spi_writes / $main_loop
```

**Scenario 3: Memory Corruption Hunt**
```gdb
# Find who's corrupting our buffer
watch uart_buffer[0]
commands
  printf "⚠️  Buffer corruption detected!\n"
  backtrace
  print/x uart_buffer[0]
  # Stop for investigation
end
```

### Why These 3 Techniques
- Essential for production debugging
- Real-world applicable skills
- Completes the GDB mastery

---

## Summary: Progressive Skill Building

### Lesson Progression

```
01: GPIO          →  Foundation (memory, vars, calls)
02: UART+DMA      →  Async (watchpoints, conditionals, stack)
03: I2C+IMU       →  Data (dumps, injection, analysis)
04: SPI+SD        →  Advanced (Python, custom commands)
05: PWM+ADC       →  Control (stats, automation)
06: Multi         →  Integration (orchestration, all skills)
07: Debug         →  Production (panics, profiling, real-world)
```

### Skill Accumulation

- After L01: 3/13 skills (23%)
- After L02: 6/13 skills (46%)
- After L03: 8/13 skills (62%)
- After L04: 10/13 skills (77%) ← Minimum viable
- After L05: 10/13 skills (77%)
- After L06: 11/13 skills (85%)
- After L07: 11/13 skills (85%) ← Complete

---

**For full technical reference:** See `GDB_REFERENCE.md`

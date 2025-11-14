# GDB Executive Summary: Quick Reference

**ESP32-C6 + esp-hal 1.0.0 - Progressive Embedded Systems Curriculum**

**Philosophy**: Build a complete embedded system progressively (button → UART → state machine → tasks → HIL testing), learning GDB techniques at each stage.

---

## 5-Lesson Curriculum Overview

| Lesson | Build | Embedded Practice | GDB Techniques | Duration |
|--------|-------|-------------------|----------------|----------|
| 01 | Button + NeoPixel | Event-driven, debouncing | Memory ops, variables, function calls | 60-90 min |
| 02 | UART Memory Streamer | UART DMA, circular buffers | Watchpoints, conditional breaks | 90-120 min |
| 03 | State Machine + IMU | Statig FSM, I2C drivers | Register diff, tracepoints, Python | 120-150 min |
| 04 | Task Scheduler | Atomics, cooperative scheduling | Watchpoints on atomics, profiling | 120-150 min |
| 05 | Virtual HIL Testing | HAL abstraction, TDD, CI/CD | Automated testing, reverse debug | 150-240 min |

**Total: 9-13.5 hours** | **Hardware: ESP32-C6-DevKitC-1 + FTDI + MPU9250**

---

## Progressive Build Path

**Lesson 01** → Simple button + NeoPixel (foundation)
**Lesson 02** → Add UART observability for complex debugging
**Lesson 03** → Add state machine + I2C sensor
**Lesson 04** → Refactor into concurrent tasks with atomics
**Lesson 05** → Test everything on host without hardware

**Each lesson builds on the previous**, creating a production-ready embedded system.

---

## Top 5 "Wow Moments"

1. **Function calls (L01)** - `call neopixel_set_color(255, 0, 0)` changes LED from GDB!
2. **UART + GDB combo (L02)** - UART streams continuously, GDB catches exact moment
3. **Python state visualizer (L03)** - Live ASCII state machine diagram in terminal
4. **Watchpoint on atomics (L04)** - Catch exact moment shared state changes
5. **Reverse debugging (L05)** - Step backward through code to find bug root cause

---

## GDB Capabilities Taught

### Lesson 01: Fundamentals
- **Memory inspection** - `x/1xw 0x6009103C` (read GPIO register)
- **Memory writes** - `set *(uint32_t*)0x60091008 = 0x200` (write GPIO)
- **GDB variables** - `set $gpio = 9; set $mask = 1 << $gpio` (bit math)
- **Function calls** - `call neopixel_set_color(255, 0, 0)` (control hardware)

### Lesson 02: Watchpoints & Breakpoints
- **Watchpoints** - `watch *(uint32_t*)0x60013024` (break when memory changes)
- **Conditional breakpoints** - `break uart_tx if addr == 0x60091000` (conditional)
- **Memory compare** - Verify streamed data matches actual memory

### Lesson 03: Advanced Debugging
- **Register diff** - Compare peripheral registers before/after operations
- **Tracepoints** - `trace state_transition` (log without stopping)
- **Python scripting** - Custom GDB commands for state visualization

### Lesson 04: Performance Analysis
- **Watchpoints on atomics** - `watch CURRENT_HUE` (lock-free debugging)
- **Call stack analysis** - `backtrace`, `up`, `down` (understand execution)
- **Performance profiling** - Measure task execution time with cycle counters

### Lesson 05: Automated Testing
- **Automated test harness** - GDB scripts run tests automatically
- **Reverse debugging** - `reverse-continue`, `reverse-step` (time travel!)
- **Record/replay** - `rr record`, `rr replay` (deterministic replay)

---

## Hardware Requirements

| Component | Used In | Cost | Notes |
|-----------|---------|------|-------|
| ESP32-C6-DevKitC-1 | All lessons | $15 | Onboard button (GPIO9) + NeoPixel (GPIO8) |
| FTDI UART adapter | Lesson 02+ | $5 | GPIO16 TX, GPIO17 RX |
| MPU9250 IMU | Lesson 03+ | $5 | I2C: GPIO2 SDA, GPIO11 SCL |
| **Total** | | **$25** | |

**Lesson 05 needs no hardware** - runs on host!

---

## Decision Matrix: Which Lesson Teaches What?

| Concept | Lesson | Why |
|---------|--------|-----|
| **GDB basics** | 01 | Start simple (button + LED) |
| **UART debugging** | 02 | Add observability before complexity |
| **State machines** | 03 | Real embedded pattern |
| **Atomics** | 04 | Lock-free concurrency |
| **HAL abstraction** | 05 | Test without hardware |
| **I2C drivers** | 03 | Sensor integration |
| **Task scheduling** | 04 | Split monolith into tasks |
| **TDD** | 05 | Professional workflow |
| **CI/CD** | 05 | Automate everything |

---

## UART + GDB Combined Strategy

**UART**: Continuous monitoring (big picture)
- Stream GPIO registers every 50ms
- Stream state machine state + sensor data
- Stream task execution times
- Machine-parseable format for analysis

**GDB**: Precise breakpoints (exact moments)
- Break when specific memory address changes (watchpoints)
- Break only when condition is true (conditional breakpoints)
- Inspect registers/memory at exact moment of interest
- Step through code instruction-by-instruction

**Power combo**: UART shows trends, GDB catches exact moments!

---

## Embedded Best Practices Taught

1. **Event-driven architecture** (L01) - Edge detection, not polling
2. **Debouncing** (L01) - Clean input handling
3. **UART DMA** (L02) - Non-blocking high-speed streaming
4. **Circular buffers** (L02) - Handle streaming without data loss
5. **Statig state machines** (L03) - Macro-based hierarchical FSM
6. **I2C drivers** (L03) - Register-based sensor integration
7. **Fixed-point math** (L03) - HSV→RGB without floats
8. **Cooperative scheduling** (L04) - Fixed-interval task execution
9. **Lock-free atomics** (L04) - No mutexes, no critical sections
10. **HAL abstraction** (L05) - Separate business logic from hardware
11. **Dependency injection** (L05) - Traits instead of concrete types
12. **TDD** (L05) - Write tests first, hardware second
13. **CI/CD** (L05) - Automate testing on every commit

---

## Claude's Teaching Approach

**Collaborative investigation**:
- "What value do you see in the GPIO_IN register?"
- "Try writing to address 0x60091008. What happens?"

**Guided discovery**:
- "Let's use GDB to find out why the button isn't working"
- "Set a watchpoint on the UART buffer. What do you notice?"

**Hands-on learning**:
- "Call this function from GDB and watch the LED"
- "Step through the I2C transaction and inspect the registers"

**Progressive complexity**:
- Lesson 01: Basic GDB commands
- Lesson 02: Watchpoints and breakpoints
- Lesson 03: Advanced techniques (tracepoints, Python)
- Lesson 04: Performance profiling
- Lesson 05: Automated testing and reverse debugging

---

## Quick Start

```bash
# Read detailed lesson plans
cat GDB_LESSON_PLANS.md

# Generate all lessons sequentially
/gen-all-lessons

# Or generate one lesson at a time
/gen-lesson "Create Lesson 01: Button + NeoPixel + GDB"
```

---

## Files

- **GDB_LESSON_PLANS.md** (27KB) - Complete curriculum with commit structures
- **GDB_REFERENCE.md** (29KB) - All GDB commands and examples
- **LESSON_GENERATION_GUIDE.md** (15KB) - How to use generation commands

---

**Last Updated**: 2025-11-14

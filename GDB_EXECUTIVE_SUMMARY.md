# GDB Capabilities: Executive Summary

**Quick reference for planning ESP32-C6 lessons with GDB debugging**

---

## 13 GDB Capabilities (Quick List)

| # | Capability | Use Case | Lessons |
|---|-----------|----------|---------|
| 1 | **Memory Inspection** | Read/write peripheral registers | All |
| 2 | **Breakpoints** | Pause execution at specific points | All |
| 3 | **Watchpoints** | Break when memory/register changes | 02, 07 |
| 4 | **Variable Ops** | Read/write firmware variables | 01, 03, 05 |
| 5 | **Call Stack** | Trace function calls, debug panics | 02, 07 |
| 6 | **Function Calls** | Execute firmware functions from GDB | 01, 06 |
| 7 | **GDB Variables** | Store values, do calculations | 01, 05 |
| 8 | **Memory Dumps** | Save buffers/registers to files | 03, 07 |
| 9 | **Python Scripting** | Custom GDB commands | 04, 06 |
| 10 | **Reverse Debug** | Time-travel debugging | ❌ Not supported |
| 11 | **Register Inspection** | View CPU/peripheral state | All |
| 12 | **Signal Handling** | Catch panics/exceptions | 07 |
| 13 | **Multi-threading** | Debug RTOS tasks | ❌ Future |

**Coverage:** 11/13 capabilities (85%)

---

## 7-Lesson Curriculum (Overview)

### Minimal Viable (Lessons 1-4)
**Duration:** 5-7 hours | **GDB Coverage:** 77%

| Lesson | Peripheral | Duration | GDB Skills | Complexity |
|--------|-----------|----------|------------|------------|
| **01** | GPIO LED | 60-90 min | Memory, Vars, Function calls | ⭐⭐☆☆☆ |
| **02** | UART+DMA | 90-120 min | Watchpoints, Conditionals, Stack | ⭐⭐⭐☆☆ |
| **03** | I2C IMU | 90-120 min | Memory dumps, Injection | ⭐⭐⭐⭐☆ |
| **04** | SPI SD Card | 120-150 min | Python scripting, Custom cmds | ⭐⭐⭐⭐⭐ |

### Extended Curriculum (Lessons 5-7)
**Duration:** +5-6 hours | **GDB Coverage:** 85%

| Lesson | Peripheral | Duration | GDB Skills | Complexity |
|--------|-----------|----------|------------|------------|
| **05** | PWM+ADC | 60-90 min | Statistics, Auto-tuning | ⭐⭐⭐☆☆ |
| **06** | Multi-peripheral | 120-180 min | Orchestration, All techniques | ⭐⭐⭐⭐⭐ |
| **07** | Debug Scenarios | 90-120 min | Panics, Profiling, Post-mortem | ⭐⭐⭐⭐⭐ |

---

## Top 10 "Wow Moments" (Ranked)

### Must-Have 🔥🔥🔥
1. **Function calls** (L01) - `call led.toggle()` from GDB
2. **Custom commands** (L04) - `sd ls` reads SD card in GDB
3. **Watchpoint detective** (L02) - Catches buffer overflow instantly
4. **System orchestration** (L06) - Controls 4 peripherals at once

### Really Cool 🔥🔥
5. **Memory dumps + analysis** (L03) - Binary dump → Python plots
6. **Auto-tuning** (L05) - GDB implements PID controller
7. **Panic debugging** (L07) - Debug real crashes with call stack

### Nice to Have 🔥
8. **Statistics dashboard** - Live telemetry counters
9. **GPIO scanner** - Inspect all 24 pins at once
10. **Bit math calculator** - GDB as learning tool

---

## Quick Decision Matrix

### "I want to teach..."

**Basic GDB (1-2 hours):**
→ Lesson 01 only (GPIO + Memory + Function calls)

**Core embedded (4-6 hours):**
→ Lessons 1-3 (GPIO, UART, I2C)

**Professional skills (6-8 hours):**
→ Lessons 1-4 (Add SPI + Python scripting)

**Complete curriculum (10-15 hours):**
→ All 7 lessons

### "I have this hardware..."

**Just ESP32-C6 + LED:**
→ Lesson 01

**+ UART adapter:**
→ Lessons 01-02

**+ I2C IMU (MPU9250):**
→ Lessons 01-03

**+ SD card module:**
→ Lessons 01-04

**+ Light sensor + PWM LED:**
→ Lessons 01-05

**Everything:**
→ All 7 lessons

---

## Technique Distribution

### Lesson 01 (Foundation)
- Memory inspection/writes ✅
- GDB variables (bit math) 🔥
- Function calls 🔥🔥🔥

### Lesson 02 (Async/ISR)
- Watchpoints 🔥🔥
- Conditional breakpoints 🔥
- Call stack debugging 🔥

### Lesson 03 (Data Analysis)
- Memory dumps 🔥
- Variable injection 🔥

### Lesson 04 (Advanced)
- Python scripting 🔥🔥🔥
- Custom commands 🔥🔥

### Lesson 05 (Control)
- Statistics dashboard 🔥
- Automated tuning 🔥

### Lesson 06 (Integration)
- Multi-watchpoints 🔥
- System orchestration 🔥🔥🔥

### Lesson 07 (Production)
- Panic handling 🔥
- Performance profiling 🔥
- Post-mortem analysis 🔥

---

## Implementation Priority

### Phase 1: Core (Do First)
✅ Lesson 01 - GPIO (foundation)
✅ Lesson 02 - UART (async debugging)

### Phase 2: Essential (Do Next)
⏳ Lesson 03 - I2C (data analysis)
⏳ Lesson 04 - SPI (Python scripting)

### Phase 3: Advanced (Optional)
⏳ Lesson 05 - PWM+ADC
⏳ Lesson 06 - Multi-peripheral
⏳ Lesson 07 - Production debugging

---

## Key Takeaways

1. **Start simple:** Lesson 01 teaches 3 core GDB techniques
2. **Build progressively:** Each lesson adds 2-3 new techniques
3. **One wow per lesson:** Keep students engaged
4. **Real hardware:** Every lesson uses actual peripherals
5. **77% coverage in 4 lessons:** Efficient learning curve

---

**For detailed info:** See `GDB_LESSON_PLANS.md` and `GDB_REFERENCE.md`

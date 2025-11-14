# Lesson 09: RTT ADS1015 Driver - Complete Work Plan

## Overview
Build ADS1015 12-bit ADC driver with comprehensive RTT-based telemetry and infrastructure for autonomous debugging.

---

## Phase 1: Core ADS1015 Driver (C4)

### 1.1 Create ADS1015 Driver Struct & I2C Interface
- [ ] Create `src/lib.rs` with ADS1015 struct
- [ ] Define I2C address (0x48) and register addresses
  - 0x00: Conversion register (read ADC result)
  - 0x01: Config register (write settings)
  - 0x02: Low threshold
  - 0x03: High threshold
- [ ] Create write_register() function (2-byte write)
- [ ] Create read_register() function (2-byte read)
- [ ] Add I2C error tracking (write attempts, read attempts, failures, timeouts)

### 1.2 Configuration Management
- [ ] Create ConfigRegister struct with bit fields
  - MUX (input multiplexer): bits [14:12]
  - PGA (programmable gain): bits [11:9]
  - MODE (single-shot vs continuous): bit [8]
  - DR (data rate): bits [7:5]
  - COMP_MODE, COMP_POL, COMP_LAT, COMP_QUE
- [ ] Implement set_mux(), set_pga(), set_mode(), set_data_rate()
- [ ] Implement write_config() with I2C write + readback verification
- [ ] Log config write and readback (raw + decoded bits)

### 1.3 Single-Shot Conversion
- [ ] Implement start_conversion() (set OS bit)
- [ ] Implement poll_ready() (check OS bit)
- [ ] Implement read_conversion() (read conversion register)
- [ ] Add conversion status tracking (busy flag, ready flag)
- [ ] Log ADC result (raw 12-bit value + converted voltage)

### 1.4 Basic RTT Telemetry Structure
- [ ] Create debug stats struct
  ```rust
  struct ADS1015Debug {
      i2c_writes: u32,
      i2c_write_success: u32,
      i2c_reads: u32,
      i2c_read_success: u32,
      i2c_errors: u32,
      config_matches: bool,
      last_adc_raw: u16,
      last_adc_volts: f32,
  }
  ```
- [ ] Add periodic logging every 100ms (defmt::info!)
- [ ] Verify RTT output appears in probe-rs

---

## Phase 2: Comprehensive Telemetry (C4 Continuation)

### 2.1 Register-Level Telemetry
- [ ] Log register writes (raw hex value)
- [ ] Log register readbacks (raw hex value)
- [ ] Log decoded bit fields (mux, pga, mode, dr)
- [ ] Log write vs readback comparison (match = true/false)

### 2.2 I2C Transaction Logging
- [ ] Track last write address and data
- [ ] Track last read address and data
- [ ] Log I2C errors with error codes
- [ ] Log timeout count and ACK errors

### 2.3 ADC Data Quality Metrics
- [ ] Track min value seen
- [ ] Track max value seen
- [ ] Calculate range (max - min)
- [ ] Detect stuck-at-same (increment counter when same)
- [ ] Calculate rolling variance
- [ ] Detect saturation (stuck at 0x8000 or 0x7FFF)

### 2.4 State Machine Tracking
- [ ] Define ADS1015State enum (Uninitialized, Configuring, Idle, Converting, Ready, Error)
- [ ] Track current state
- [ ] Track state transitions (count)
- [ ] Track time in each state
- [ ] Log state machine every 100ms

### 2.5 Complete Logging Template
Log every 100ms via defmt::info!:
- I2C status
- Config register write + readback
- ADC result (raw + voltage)
- Data quality (min/max/range/stuck)
- State machine (current state, time)

---

## Phase 3: RTT Infrastructure & Parser (C5)

### 3.1 Python RTT Parser Setup
- [ ] Create `rtt_parser.py` script
- [ ] Connect to probe-rs RTT (using defmt-print output)
- [ ] Parse defmt log messages
- [ ] Extract structured fields (i2c status, adc value, etc.)

### 3.2 Real-Time Data Display
- [ ] Create terminal UI showing:
  - I2C health (wr/rd success rates)
  - Config register (mux, pga, mode, dr)
  - ADC result (raw, voltage, trend)
  - Data quality (range, variance, stuck count)
  - State machine (current, time, transitions)
- [ ] Highlight anomalies (red for errors, yellow for warnings)

### 3.3 Data Recording
- [ ] Log RTT output to file (CSV or JSON)
- [ ] Timestamp each message
- [ ] Enable offline analysis

### 3.4 Pattern Detection
- [ ] Detect I2C failures (wr success rate drops)
- [ ] Detect config mismatch (write != readback)
- [ ] Detect stuck data (stuck count > threshold)
- [ ] Detect saturation (range too small or at extremes)
- [ ] Alert Claude Code to anomalies

---

## Phase 4: Testing & Profiling (C6-C7)

### 4.1 Variable Count Sweep
- [ ] Test with 50 variables @ 100 Hz
- [ ] Test with 100 variables @ 100 Hz
- [ ] Test with 200 variables @ 100 Hz
- [ ] Test with 500 variables @ 100 Hz
- [ ] Measure: RTT frame drops, latency, accuracy

### 4.2 Sample Rate Sweep
- [ ] Test @ 10 Hz
- [ ] Test @ 50 Hz
- [ ] Test @ 100 Hz
- [ ] Test @ 500 Hz
- [ ] Test @ 1000 Hz
- [ ] Measure: Throughput, saturation point

### 4.3 JTAG Clock Variation
- [ ] Test @ 1 MHz JTAG clock
- [ ] Test @ 4 MHz JTAG clock
- [ ] Test @ 10 MHz JTAG clock
- [ ] Measure: Throughput improvement vs clock

### 4.4 Bottleneck Identification
- [ ] Identify if limited by JTAG bandwidth
- [ ] Identify if limited by probe-rs parsing
- [ ] Identify if limited by USB speed
- [ ] Document findings

---

## Phase 5: Driver Robustness (Bonus)

### 5.1 Error Handling
- [ ] Add timeout logic (max retries)
- [ ] Add automatic retry on I2C failure
- [ ] Add state recovery (reset on error)
- [ ] Log all error conditions

### 5.2 Continuous Mode
- [ ] Implement continuous conversion mode
- [ ] Add interrupt capability (if using comparator)
- [ ] Test data stream @ high sample rates

### 5.3 Calibration
- [ ] Implement gain calibration
- [ ] Test with known reference voltage
- [ ] Verify accuracy (within ±1 count)

---

## Checkpoint Dependencies

```
C4: Basic Driver + Telemetry
    ├─ ADS1015 struct & I2C interface
    ├─ Config register management
    ├─ Single-shot conversion
    └─ Comprehensive RTT logging
        ↓
C5: RTT Infrastructure + Parser
    ├─ Python RTT parser
    ├─ Real-time data display
    ├─ Pattern detection
    └─ Data recording
        ↓
C6: Variable Count Profiling
    ├─ 50/100/200/500 variable sweeps
    ├─ Measure frame drops & latency
    └─ Identify saturation point
        ↓
C7: Comprehensive Profiling
    ├─ Sample rate sweeps (10-1000 Hz)
    ├─ JTAG clock variation
    ├─ Bottleneck identification
    └─ Final throughput report
```

---

## Technical Details

### ADS1015 Register Format

**Config Register (0x01):**
```
Bit  Field
───  ─────────────────────
14:12 MUX (input select, 0-7)
11:9  PGA (gain, 0-7)
8     MODE (0=continuous, 1=single-shot)
7:5   DR (data rate, 0-7)
4     COMP_MODE
3     COMP_POL
2     COMP_LAT
1:0   COMP_QUE
```

**Conversion Register (0x00):**
```
Bits [15:4] = ADC result (12-bit, right-aligned)
Bits [3:0]  = Don't care
```

### Voltage Calculation
```rust
// For each PGA setting:
// PGA=0: Reserved (invalid)
// PGA=1: ±6.144V (187.5 µV/count)
// PGA=2: ±4.096V (125 µV/count)
// PGA=3: ±2.048V (62.5 µV/count)
// PGA=4: ±1.024V (31.25 µV/count)
// PGA=5: ±0.512V (15.625 µV/count)
// PGA=6: ±0.256V (7.8125 µV/count)

volts = (raw_12bit as i16) as f32 * lsb_volts / 2048.0
```

### RTT Bandwidth Budget
```
6 defmt::info calls × ~80 bytes = 480 bytes per cycle
@ 100 Hz = 48 KB/s
Available = 1-10 MB/s
Utilization = 0.48-4.8%
Headroom: can log 10-20x MORE
```

---

## Success Criteria

- [ ] ADS1015 reads stable values (< 1 LSB variance on quiet input)
- [ ] RTT logs show all telemetry without frame drops @ 100 Hz
- [ ] Pattern detection catches stuck data, saturation, I2C errors
- [ ] Profiling complete: variable limits documented
- [ ] Python parser shows real-time hardware state
- [ ] Documentation shows concrete debugging examples

---

## Notes

- Use L08 as reference (same I2C, similar structure)
- Focus on observability first, robustness second
- Every bug should be catchable via RTT telemetry
- This lesson is about building debugging infrastructure, not just a driver

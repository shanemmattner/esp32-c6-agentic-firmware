# Data-Driven ADS1015 Driver Development

Using RTT telemetry to catch mistakes instantly and get the driver working fast.

## Core Philosophy: Virtual Debug Ears and Eyes

**Traditional embedded debugging:**
- You're blind while firmware runs
- You can stop at breakpoints (but freeze timing)
- You can print to UART (but block the firmware)
- You guess what's happening based on symptoms

**Data-driven debugging with RTT:**
- You have **eyes** inside the hardware (register values, ADC outputs, GPIO states, memory)
- You have **ears** listening to I2C transactions, state changes, error flags, event counters
- **Everything runs live** - firmware never stops, timing stays accurate
- Patterns jump out immediately - you don't guess, you observe

### RTT as Virtual Instrumentation

Think of RTT logging as placing probes throughout your firmware:

```
Physical Hardware          Virtual Instrumentation (RTT Logs)
──────────────────         ──────────────────────────────────

I2C Bus      ────────→     "i2c: wr=5/5 rd=5/5 last_val=0x8483"
Config Reg   ────────→     "ads_cfg: mux=0 pga=1 mode=0 dr=7"
ADC Output   ────────→     "ads_adc: raw=0x0ABC volts=1.234"
State FSM    ────────→     "ads_fsm: state=Reading time_ms=45"
Error Flags  ────────→     "i2c: errs=0 timeouts=0 acks=0"
```

Instead of stopping the system to inspect a value (breakpoint), you let it run and **stream all the data to your terminal in real-time via RTT.**

### Why This Matters for Autonomous Development

Claude Code can:
1. **See what's happening** - Full telemetry stream
2. **Spot patterns** - Correlations reveal root causes
3. **Propose fixes** - Based on actual hardware behavior, not guesses
4. **Validate fixes** - Watch the telemetry change in real-time

This is **100x faster** than "set a breakpoint → inspect → hypothesis → repeat."

## Philosophy

Instead of: "Write driver → test → debug incrementally"
Use: "Write driver with maximum observability → mistakes reveal themselves immediately"

## Key Variables to Track

### I2C Communication Layer

**Why:** Most ADS1015 problems are I2C-related (bus errors, timeouts, address conflicts)

```rust
// Track every I2C transaction AND raw register values
struct I2CStats {
    write_attempts: u32,
    write_success: u32,
    write_failures: u32,
    read_attempts: u32,
    read_success: u32,
    read_failures: u32,
    timeout_count: u32,
    ack_errors: u32,
    last_error_code: u32,

    // Last values we tried to write/read
    last_write_addr: u8,
    last_write_data: u16,
    last_read_addr: u8,
    last_read_data: u16,
}
```

**Log every 100ms:**
```rust
defmt::info!("i2c: wr={}/{} rd={}/{} err={} last_wr_addr=0x{:02x} last_wr_val=0x{:04x} last_rd_val=0x{:04x}",
    i2c_stats.write_success, i2c_stats.write_attempts,
    i2c_stats.read_success, i2c_stats.read_attempts,
    i2c_stats.timeout_count,
    i2c_stats.last_write_addr,
    i2c_stats.last_write_data,
    i2c_stats.last_read_data
);
```

**What you'll see:**
- `wr=0/5 last_wr_addr=0x48` → Writing to address 0x48, but failing (device not there? address mismatch?)
- `rd=0/5 last_rd_addr=0x01` → Trying to read config register, failing (device powered?)
- `last_wr_val=0xFFFF last_rd_val=0x0000` → Writing max, reading zero (data line stuck? pull-ups wrong?)

### Hardware Configuration - All Register Values

**Why:** Most bugs are in register writes. Log EVERYTHING: what we write, what comes back, and decoded bit fields

```rust
// Log raw register values AND decoded bit fields
struct ADS1015Registers {
    // Config register (0x01) - all bits visible
    config_written: u16,
    config_readback: u16,

    // Decoded from config register
    mux: u8,            // Bits [14:12] - Input multiplexer (0-7)
    pga: u8,            // Bits [11:9] - Programmable gain (0-7 = 6.144V to 0.256V)
    mode: u8,           // Bit [8] - 0=continuous, 1=single-shot
    dr: u8,             // Bits [7:5] - Data rate (0-7 = 128 SPS to 3300 SPS)
    comp_mode: u8,      // Bit [4] - Comparator mode
    comp_polarity: u8,  // Bit [3] - Comparator polarity
    comp_latching: u8,  // Bit [2] - Latching comparator
    comp_queue: u8,     // Bits [1:0] - Comparator queue

    // Conversion register (0x00) - raw ADC result
    conversion_raw: u16,

    // Threshold registers for debugging
    lo_thresh: u16,     // Reg 0x02 - Low threshold
    hi_thresh: u16,     // Reg 0x03 - High threshold
}
```

**Log EVERY register read and write:**
```rust
// After writing config
defmt::info!("ads_cfg_wr: wrote=0x{:04x} mux={} pga={} mode={} dr={}",
    config_written,
    (config_written >> 12) & 0x7,
    (config_written >> 9) & 0x7,
    (config_written >> 8) & 0x1,
    (config_written >> 5) & 0x7
);

// After reading back config to verify
defmt::info!("ads_cfg_rb: read=0x{:04x} mux={} pga={} mode={} match={}",
    config_readback,
    (config_readback >> 12) & 0x7,
    (config_readback >> 9) & 0x7,
    (config_readback >> 8) & 0x1,
    config_written == config_readback
);

// After every conversion result
defmt::info!("ads_adc: raw=0x{:04x} decimal={} volts={:.3}",
    conversion_raw,
    (conversion_raw as i16) >> 4,  // ADS1015 uses 12-bit right-aligned
    calculate_volts(conversion_raw, pga)
);

// Threshold registers (useful for comparator debugging)
defmt::info!("ads_thresh: lo=0x{:04x} hi=0x{:04x}",
    lo_thresh, hi_thresh
);
```

**What you'll see (immediate problems jump out):**
- `cfg_wr: mux=7 pga=0 dr=0` → PGA=0 is INVALID (why did we set it to 0?)
- `cfg_rb: read=0xFFFF match=0` → Read back all 1s, mismatch with write (I2C corruption? wrong address?)
- `adc: raw=0x8000 volts=-4.096` → Sat at negative full scale (input shorted to ground?)
- `adc: raw=0x7FFF volts=4.095` → Sat at positive full scale (input floating or too high?)
- `match=0 wrote=0x8483 read=0x0000` → We wrote valid config but read zeros (I2C lines stuck?)

### Conversion Results

**Why:** Know if readings are stuck, oscillating, or correct**

```rust
struct ADS1015Reading {
    raw_value: i16,
    volts: f32,

    // Track stuckness
    last_value: i16,
    same_count: u32,           // How many times in a row same value?

    // Track valid range
    min_seen: i16,
    max_seen: i16,
    range: i16,

    // Track conversion timing
    conversion_time_us: u32,
    ready_count: u32,          // How many times READY bit went high?
}
```

**Log every conversion:**
```rust
defmt::info!("adc: raw={} volts={} same_for={} range={}..{} ready_flag={}",
    reading.raw_value, reading.volts, reading.same_count,
    reading.min_seen, reading.max_seen, reading.ready_count
);
```

**What you'll see:**
- `raw=0 volts=0.0 same_for=1000` → Device stuck at 0 (not configured? wrong input selected?)
- `raw=32000 volts=4.096 same_for=1000` → Saturated (input shorted? wrong gain?)
- `ready_flag=0` → Conversion never completes (continuous mode? clock issue?)
- `range=32670..32767` → Reading entire MSB range = good data

### State Machine Tracking

**Why:** Catch logic errors early**

```rust
#[derive(Debug, Clone, Copy)]
enum ADS1015State {
    Uninitialized,
    Initializing,
    ConfigWritten,     // Sent config, waiting for readback
    ConfigVerified,    // Config matches
    Idle,
    ConversionInProgress,
    ResultReady,
    ResultRead,
}

struct ADS1015Debug {
    state: ADS1015State,
    state_changes: u32,
    time_in_state_ms: u32,
    state_stuck_count: u32,    // How many times we logged same state?
}
```

**Log every state change + every 100ms:**
```rust
defmt::info!("ads1015_state: current={:?} changed={} stuck_for={} time_ms={}",
    debug.state, debug.state_changes, debug.state_stuck_count, debug.time_in_state_ms
);
```

**What you'll see:**
- `stuck_for=1000` → State machine frozen (waiting for something that never happens?)
- `time_ms=5000` → Taking too long in one state (slow I2C? polling timeout too long?)
- Rapid state changes → State machine oscillating (logic error in transitions)

## Development Phases with Data Feedback

### Phase 1: I2C Communication
```rust
// Initialize I2C, write config register
// WATCH: Can we even talk to the chip?

defmt::info!("phase1: i2c_ok={} write_ok={} readback_ok={}",
    i2c_initialized, config_write_success, readback_matches
);
```

**Red flags:**
- `write_ok=0` → I2C peripheral not working
- `readback_ok=0` → Device not at address 0x48

### Phase 2: Configuration
```rust
// Set gain, channel, data rate
// WATCH: Do our settings stick?

defmt::info!("phase2: gain=0x{:02x} channel={} rate={} cfg_match={}",
    pga_gain, mux_channel, data_rate, config_matches
);
```

**Red flags:**
- `cfg_match=0` → Settings not persisting (I2C corruption? timing issue?)
- `gain=0xFF` → Read garbage (bad I2C read?)

### Phase 3: Conversions
```rust
// Enable single-shot, wait for ready
// WATCH: Does conversion complete?

defmt::info!("phase3: os_bit={} ready_bit={} retry_count={} result={}",
    os_bit_set, ready_bit_high, retries, raw_result
);
```

**Red flags:**
- `ready_bit=0 retry_count=1000` → Conversion never completes (wrong bit? polling wrong register?)
- `result=0` → Data is all zeros (input not connected? wrong channel selected?)

### Phase 4: Data Quality
```rust
// Read multiple samples
// WATCH: Do we get reasonable values?

defmt::info!("phase4: samples={} range={}..{} variance={} outliers={}",
    sample_count, min, max, variance, outlier_count
);
```

**Red flags:**
- `range=0..0` → All same value (noise floor is zero, input floating?)
- `variance=30000` → Extremely noisy (bad ground? noise on input?)
- `outliers=100` → Random spikes (I2C glitches?)

## Complete Logging Strategy for ADS1015

Log EVERYTHING every 10-100ms (defmt is non-blocking via RTT):

### Comprehensive Log Example

```rust
// I2C layer status
defmt::info!("i2c: wr={}/{} rd={}/{} errs={} last_addr=0x{:02x} last_val=0x{:04x}",
    write_success, write_attempts, read_success, read_attempts,
    error_count, last_address, last_value
);

// Register values as written
defmt::info!("ads_reg: cfg_wr=0x{:04x} mux={} pga={} mode={} dr={}",
    config_written,
    (config_written >> 12) & 0x7,
    (config_written >> 9) & 0x7,
    (config_written >> 8) & 0x1,
    (config_written >> 5) & 0x7
);

// Register values as read back
defmt::info!("ads_ver: cfg_rb=0x{:04x} mux={} pga={} mode={} OK={}",
    config_readback,
    (config_readback >> 12) & 0x7,
    (config_readback >> 9) & 0x7,
    (config_readback >> 8) & 0x1,
    config_written == config_readback
);

// Conversion status and raw data
defmt::info!("ads_adc: raw=0x{:04x} decimal={} volts={:.3} busy={} ready={}",
    conversion_raw,
    (conversion_raw as i16) >> 4,
    calculate_volts(conversion_raw, pga),
    conversion_busy_flag,
    conversion_ready_flag
);

// Data quality and stuck detection
defmt::info!("ads_dat: min=0x{:04x} max=0x{:04x} range={} stuck={} var={}",
    min_value_seen,
    max_value_seen,
    max_value_seen - min_value_seen,
    stuck_at_same_count,
    rolling_variance
);

// Thresholds and comparator (if using)
defmt::info!("ads_thr: lo=0x{:04x} hi=0x{:04x} cmp_out={}",
    low_threshold, high_threshold, comparator_output
);

// State machine and timing
defmt::info!("ads_fsm: state={:?} changed={} time_ms={} timeout_pending={}",
    current_state, state_change_count, time_in_state_ms, timeout_active
);
```

### Bandwidth Calculation

Per log entry:
- 6 defmt::info calls × ~80 bytes each = 480 bytes per cycle
- @ 100 Hz: 480 × 100 = 48 KB/s
- Available RTT: 1-10 MB/s
- **Utilization: 0.48% to 4.8%** - plenty of headroom!

You can log even MORE details if needed without saturation.

## Example: Debugging a Stuck-at-Zero Problem

**Scenario:** ADC always reads 0, driver doesn't work

**With massive logging:**
```
ads: state=Reading i2c_err=0 cfg_ok=1 adc_val=0 volts=0.00 ready=1 same=100
→ State machine fine, I2C OK, config matches, but ADC stuck at 0

ads: state=Reading i2c_err=0 cfg_ok=1 adc_val=0 mux=0 gain=0 ready=1
→ Channel 0 is selected, gain is 0 (wait... gain=0 might be invalid!)

Check ADS1015 datasheet: Gain 0 = invalid! Must be 1-7
→ BUG FOUND: Default gain is out of range
→ FIX: Set gain to 1 (6.144V full scale)
```

**Time to diagnose:** ~30 seconds (you can SEE the gain is 0)

**Without logging:**
- Spend 30 minutes: "Why is ADC reading zero?"
- Debug I2C (it's fine)
- Check wiring (it's fine)
- Read datasheet again...
- Ah! Gain field might be wrong
- Spend 10 minutes confirming

**Time to diagnose:** ~40 minutes

## Variable Bandwidth Budget for ADS1015

```
Example message with 12 variables:
defmt::info!("ads: state={:?} i2c={} cfg={} val={} volts={:.2} ready={} same={}",
    7 variables...

defmt overhead: ~20 bytes
Per variable: ~5-10 bytes average
Total per message: ~70 bytes

Logging at 100 Hz: 70 bytes × 100 Hz = 7 KB/s
RTT capacity: 1-10 MB/s
Utilization: 0.07% of available bandwidth
```

We have TONS of headroom. Add more variables if needed!

## Summary: Data-Driven ADS1015 Development

**Instead of:** "Write driver, hope it works, debug when broken"
**Do:** "Write driver with comprehensive logging, watch it work in real-time, mistakes appear instantly"

**The variables reveal:**
1. **I2C health** → Can we even talk to the device?
2. **Configuration correctness** → Did our register writes work?
3. **Conversion status** → Is the ADC actually measuring?
4. **Data quality** → Are readings reasonable or garbage?
5. **State machine health** → Is logic flowing correctly?

With all this data streaming @ 100 Hz via RTT, bugs jump out at you.
You don't need to hypothesis-test; the data tells you what's wrong.

---

**Ready to build a robust ADS1015 driver with visibility!** 🔍

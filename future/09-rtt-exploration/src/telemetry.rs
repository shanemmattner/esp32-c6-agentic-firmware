/// RTT-based telemetry system for streaming arbitrary variables to host
///
/// This module provides structured logging of hardware state via defmt+RTT.
/// All logs are non-blocking and can handle 50-500+ variables @ 100 Hz.
///
/// Usage:
/// ```rust
/// let mut telemetry = Telemetry::new();
/// telemetry.log_i2c_status(writes, reads, errors);
/// telemetry.log_adc_result(raw_value, voltage);
/// telemetry.log_state(state, time_ms);
/// ```

use defmt::{info, Format};

/// I2C communication statistics
#[derive(Clone, Copy, Debug, Format)]
pub struct I2CStatus {
    pub write_attempts: u32,
    pub write_success: u32,
    pub read_attempts: u32,
    pub read_success: u32,
    pub error_count: u32,
    pub last_addr: u8,
    pub last_value: u16,
}

impl I2CStatus {
    pub fn new() -> Self {
        Self {
            write_attempts: 0,
            write_success: 0,
            read_attempts: 0,
            read_success: 0,
            error_count: 0,
            last_addr: 0,
            last_value: 0,
        }
    }

    pub fn record_write(&mut self, addr: u8, value: u16) {
        self.write_attempts += 1;
        self.last_addr = addr;
        self.last_value = value;
    }

    pub fn record_write_success(&mut self) {
        self.write_success += 1;
    }

    pub fn record_read(&mut self, addr: u8) {
        self.read_attempts += 1;
        self.last_addr = addr;
    }

    pub fn record_read_success(&mut self) {
        self.read_success += 1;
    }

    pub fn record_error(&mut self) {
        self.error_count += 1;
    }

    pub fn log(&self) {
        info!("i2c: wr={}/{} rd={}/{} err={} last_addr=0x{:02x} last_val=0x{:04x}",
            self.write_success,
            self.write_attempts,
            self.read_success,
            self.read_attempts,
            self.error_count,
            self.last_addr,
            self.last_value
        );
    }

    pub fn write_success_rate(&self) -> u8 {
        if self.write_attempts == 0 {
            0
        } else {
            ((self.write_success as u32 * 100) / self.write_attempts as u32) as u8
        }
    }

    pub fn read_success_rate(&self) -> u8 {
        if self.read_attempts == 0 {
            0
        } else {
            ((self.read_success as u32 * 100) / self.read_attempts as u32) as u8
        }
    }
}

/// Configuration register state
#[derive(Clone, Copy, Debug, Format)]
pub struct ConfigRegisterState {
    pub written: u16,
    pub readback: u16,
    pub mux: u8,
    pub pga: u8,
    pub mode: u8,
    pub dr: u8,
}

impl ConfigRegisterState {
    pub fn new() -> Self {
        Self {
            written: 0,
            readback: 0,
            mux: 0,
            pga: 0,
            mode: 0,
            dr: 0,
        }
    }

    pub fn from_raw(raw: u16) -> Self {
        Self {
            written: raw,
            readback: 0,
            mux: ((raw >> 12) & 0x7) as u8,
            pga: ((raw >> 9) & 0x7) as u8,
            mode: ((raw >> 8) & 0x1) as u8,
            dr: ((raw >> 5) & 0x7) as u8,
        }
    }

    pub fn set_readback(&mut self, raw: u16) {
        self.readback = raw;
    }

    pub fn matches(&self) -> bool {
        self.written == self.readback
    }

    pub fn log_write(&self) {
        info!("cfg_wr: wrote=0x{:04x} mux={} pga={} mode={} dr={}",
            self.written,
            self.mux,
            self.pga,
            self.mode,
            self.dr
        );
    }

    pub fn log_readback(&self) {
        info!("cfg_rb: read=0x{:04x} mux={} pga={} mode={} match={}",
            self.readback,
            ((self.readback >> 12) & 0x7) as u8,
            ((self.readback >> 9) & 0x7) as u8,
            ((self.readback >> 8) & 0x1) as u8,
            self.matches()
        );
    }
}

/// ADC conversion results
#[derive(Clone, Copy, Debug, Format)]
pub struct ADCResult {
    pub raw: u16,
    pub volts: f32,
    pub ready: bool,
    pub busy: bool,
}

impl ADCResult {
    pub fn new() -> Self {
        Self {
            raw: 0,
            volts: 0.0,
            ready: false,
            busy: false,
        }
    }

    pub fn from_raw_12bit(raw_12bit: u16, pga: u8) -> Self {
        // ADS1015: 12-bit right-aligned
        let lsb_mv = match pga {
            0 => 0.0,           // Invalid
            1 => 187.5,         // ±6.144V
            2 => 125.0,         // ±4.096V
            3 => 62.5,          // ±2.048V
            4 => 31.25,         // ±1.024V
            5 => 15.625,        // ±0.512V
            6 => 7.8125,        // ±0.256V
            _ => 0.0,
        };

        let signed_12bit = (raw_12bit as i16) >> 4; // Convert to signed 12-bit
        let volts = (signed_12bit as f32) * (lsb_mv / 1000.0) / 2048.0;

        Self {
            raw: raw_12bit,
            volts,
            ready: false,
            busy: false,
        }
    }

    pub fn log(&self) {
        // Convert volts to millivolts for integer logging (defmt doesn't support float formatting)
        let mv = (self.volts * 1000.0) as i32;
        info!("adc: raw=0x{:04x} mv={} busy={} ready={}",
            self.raw,
            mv,
            self.busy,
            self.ready
        );
    }
}

/// Data quality metrics
#[derive(Clone, Copy, Debug, Format)]
pub struct DataQuality {
    pub min: u16,
    pub max: u16,
    pub stuck_count: u16,
    pub last_value: u16,
}

impl DataQuality {
    pub fn new() -> Self {
        Self {
            min: 0xFFFF,
            max: 0,
            stuck_count: 0,
            last_value: 0xFFFF,
        }
    }

    pub fn update(&mut self, value: u16) {
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }

        if value == self.last_value {
            self.stuck_count += 1;
        } else {
            self.stuck_count = 0;
        }

        self.last_value = value;
    }

    pub fn range(&self) -> u16 {
        if self.min <= self.max {
            self.max - self.min
        } else {
            0
        }
    }

    pub fn log(&self) {
        info!("dat: min=0x{:04x} max=0x{:04x} range={} stuck={}",
            self.min,
            self.max,
            self.range(),
            self.stuck_count
        );
    }
}

/// State machine tracking
#[derive(Clone, Copy, Debug, Format)]
pub enum SystemState {
    Uninitialized,
    Initializing,
    ConfigWritten,
    ConfigVerified,
    Idle,
    ConversionInProgress,
    ResultReady,
    Error,
}

#[derive(Clone, Copy, Debug, Format)]
pub struct StateTracking {
    pub state: SystemState,
    pub state_changes: u32,
    pub time_in_state_ms: u32,
}

impl StateTracking {
    pub fn new() -> Self {
        Self {
            state: SystemState::Uninitialized,
            state_changes: 0,
            time_in_state_ms: 0,
        }
    }

    pub fn transition(&mut self, new_state: SystemState) {
        self.state = new_state;
        self.state_changes += 1;
        self.time_in_state_ms = 0;
    }

    pub fn update_time(&mut self, delta_ms: u32) {
        self.time_in_state_ms += delta_ms;
    }

    pub fn log(&self) {
        info!("fsm: state={:?} changes={} time_ms={}",
            self.state,
            self.state_changes,
            self.time_in_state_ms
        );
    }
}

/// Main telemetry coordinator
pub struct Telemetry {
    pub i2c: I2CStatus,
    pub config: ConfigRegisterState,
    pub adc: ADCResult,
    pub data_quality: DataQuality,
    pub state: StateTracking,
}

impl Telemetry {
    pub fn new() -> Self {
        Self {
            i2c: I2CStatus::new(),
            config: ConfigRegisterState::new(),
            adc: ADCResult::new(),
            data_quality: DataQuality::new(),
            state: StateTracking::new(),
        }
    }

    /// Log all telemetry (call this every 100ms)
    pub fn log_all(&self) {
        self.i2c.log();
        self.config.log_readback();
        self.adc.log();
        self.data_quality.log();
        self.state.log();
    }

    /// Log only critical state (lighter weight)
    pub fn log_critical(&self) {
        let adc_mv = (self.adc.volts * 1000.0) as i32;
        info!("sys: i2c_ok={} cfg_ok={} adc_mv={} state={:?}",
            self.i2c.error_count == 0,
            self.config.matches(),
            adc_mv,
            self.state.state
        );
    }
}

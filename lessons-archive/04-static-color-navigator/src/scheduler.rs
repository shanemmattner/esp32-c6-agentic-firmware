//! Simple cooperative task scheduler
//!
//! Based on Lesson 02 scheduler, simplified for Lesson 04.

use crate::{BUTTON_PERIOD_MS, IMU_PERIOD_MS, LED_PERIOD_MS, TICK_MS};
use esp_hal::delay::Delay;

/// Simple cooperative scheduler for three tasks
pub struct Scheduler {
    current_time_ms: u64,
    button_next_run_ms: u64,
    imu_next_run_ms: u64,
    led_next_run_ms: u64,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            current_time_ms: 0,
            button_next_run_ms: 0,
            imu_next_run_ms: 0,
            led_next_run_ms: 0,
        }
    }

    /// Run one scheduler tick
    ///
    /// Delays for TICK_MS, then runs tasks that are due
    pub fn tick<F1, F2, F3>(&mut self, delay: &Delay, mut button_task: F1, mut imu_task: F2, mut led_task: F3)
    where
        F1: FnMut(),
        F2: FnMut(),
        F3: FnMut(),
    {
        // Advance time
        self.current_time_ms += TICK_MS;
        delay.delay_millis(TICK_MS as u32);

        // Run button task if due
        if self.current_time_ms >= self.button_next_run_ms {
            button_task();
            self.button_next_run_ms = self.current_time_ms + BUTTON_PERIOD_MS;
        }

        // Run IMU task if due
        if self.current_time_ms >= self.imu_next_run_ms {
            imu_task();
            self.imu_next_run_ms = self.current_time_ms + IMU_PERIOD_MS;
        }

        // Run LED task if due
        if self.current_time_ms >= self.led_next_run_ms {
            led_task();
            self.led_next_run_ms = self.current_time_ms + LED_PERIOD_MS;
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

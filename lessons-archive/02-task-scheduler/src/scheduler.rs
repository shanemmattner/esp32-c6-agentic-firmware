//! Simple cooperative scheduler for running tasks at different periods.
//!
//! Tasks are functions that run at regular intervals without interrupts.
//! This is a "cooperative" scheduler - tasks must return control to the scheduler.

use crate::{BUTTON_PERIOD_MS, LED_PERIOD_MS, TICK_MS};
use esp_hal::delay::Delay;

/// Task function type - takes no parameters, returns nothing
pub type TaskFn<'a> = &'a dyn Fn();

/// Simple scheduler state
pub struct Scheduler {
    /// Current virtual time in milliseconds
    current_time_ms: u64,
    /// Next time button task should run
    button_next_run_ms: u64,
    /// Next time LED task should run
    led_next_run_ms: u64,
}

impl Scheduler {
    /// Create a new scheduler starting at time 0
    pub fn new() -> Self {
        Self {
            current_time_ms: 0,
            button_next_run_ms: 0,
            led_next_run_ms: 0,
        }
    }

    /// Run one scheduler tick
    ///
    /// This advances time by TICK_MS and runs tasks that are due.
    /// Call this repeatedly in your main loop.
    pub fn tick<F1, F2>(&mut self, delay: &Delay, mut button_task: F1, mut led_task: F2)
    where
        F1: FnMut(),
        F2: FnMut(),
    {
        // Advance time
        self.current_time_ms += TICK_MS;
        delay.delay_millis(TICK_MS as u32);

        // Run button task if period elapsed
        if self.current_time_ms >= self.button_next_run_ms {
            button_task();
            self.button_next_run_ms = self.current_time_ms + BUTTON_PERIOD_MS;
        }

        // Run LED task if period elapsed
        if self.current_time_ms >= self.led_next_run_ms {
            led_task();
            self.led_next_run_ms = self.current_time_ms + LED_PERIOD_MS;
        }
    }

    /// Get current virtual time in milliseconds
    pub fn current_time_ms(&self) -> u64 {
        self.current_time_ms
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

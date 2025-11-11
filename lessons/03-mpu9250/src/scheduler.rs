//! Simple task scheduler

use esp_hal::i2c::master::I2c;
use esp_hal::DriverMode;

/// A task that runs at a fixed period
pub struct Task<'a, Dm: DriverMode> {
    pub run: fn(&mut Context<'a, Dm>),
    pub period_ms: u64,
    pub last_run: u64,
}

/// Context passed to all tasks - holds hardware references
pub struct Context<'a, Dm: DriverMode> {
    pub i2c: &'a mut I2c<'a, Dm>,
}

impl<'a, Dm: DriverMode> Task<'a, Dm> {
    /// Check if this task should run based on current time
    pub fn should_run(&self, now: u64) -> bool {
        (now - self.last_run) >= self.period_ms
    }

    /// Execute the task and update last run time
    pub fn execute(&mut self, now: u64, ctx: &mut Context<'a, Dm>) {
        (self.run)(ctx);
        self.last_run = now;
    }
}

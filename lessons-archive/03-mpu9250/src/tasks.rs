//! Task implementations

use crate::mpu9250;
use crate::scheduler::Context;
use esp_hal::DriverMode;
use log::info;

/// Task: Read IMU sensor data and print to console
pub fn imu_task<Dm: DriverMode>(ctx: &mut Context<Dm>) {
    // Read accelerometer
    match mpu9250::read_accel(ctx.i2c) {
        Ok(accel) => {
            info!("[Accel] X={:6} Y={:6} Z={:6}", accel.x, accel.y, accel.z);
        }
        Err(_) => {
            info!("[Accel] Read failed");
        }
    }

    // Read gyroscope
    match mpu9250::read_gyro(ctx.i2c) {
        Ok(gyro) => {
            info!("[Gyro]  X={:6} Y={:6} Z={:6}", gyro.x, gyro.y, gyro.z);
        }
        Err(_) => {
            info!("[Gyro] Read failed");
        }
    }

    info!("");
}

//! MPU9250 sensor helper functions

use esp_hal::i2c::master::I2c;
use esp_hal::DriverMode;

pub const MPU_ADDR: u8 = 0x68;
pub const WHO_AM_I_REG: u8 = 0x75;
pub const PWR_MGMT_1: u8 = 0x6B;
pub const ACCEL_XOUT_H: u8 = 0x3B;
pub const GYRO_XOUT_H: u8 = 0x43;

#[derive(Debug, Clone, Copy)]
pub struct AccelData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Debug, Clone, Copy)]
pub struct GyroData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// Wake up the MPU9250 from sleep mode
pub fn wake_sensor<Dm: DriverMode>(i2c: &mut I2c<Dm>) -> Result<(), ()> {
    i2c.write(MPU_ADDR, &[PWR_MGMT_1, 0x00]).map_err(|_| ())
}

/// Read WHO_AM_I register
pub fn read_who_am_i<Dm: DriverMode>(i2c: &mut I2c<Dm>) -> Result<u8, ()> {
    let mut buf = [0u8; 1];
    i2c.write_read(MPU_ADDR, &[WHO_AM_I_REG], &mut buf)
        .map_err(|_| ())?;
    Ok(buf[0])
}

/// Read accelerometer data
pub fn read_accel<Dm: DriverMode>(i2c: &mut I2c<Dm>) -> Result<AccelData, ()> {
    let mut buf = [0u8; 6];
    i2c.write_read(MPU_ADDR, &[ACCEL_XOUT_H], &mut buf)
        .map_err(|_| ())?;

    Ok(AccelData {
        x: i16::from_be_bytes([buf[0], buf[1]]),
        y: i16::from_be_bytes([buf[2], buf[3]]),
        z: i16::from_be_bytes([buf[4], buf[5]]),
    })
}

/// Read gyroscope data
pub fn read_gyro<Dm: DriverMode>(i2c: &mut I2c<Dm>) -> Result<GyroData, ()> {
    let mut buf = [0u8; 6];
    i2c.write_read(MPU_ADDR, &[GYRO_XOUT_H], &mut buf)
        .map_err(|_| ())?;

    Ok(GyroData {
        x: i16::from_be_bytes([buf[0], buf[1]]),
        y: i16::from_be_bytes([buf[2], buf[3]]),
        z: i16::from_be_bytes([buf[4], buf[5]]),
    })
}

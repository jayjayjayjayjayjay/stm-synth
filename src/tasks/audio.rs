use embassy_stm32::gpio::Output;
use embassy_stm32::peripherals::I2C1;
use embassy_stm32::{
    i2s::{Error as I2SError, I2S},
    mode::Async,
};
use embassy_time::Timer;

use core::f32::consts::PI;
use libm::sinf;

const SAMPLE_RATE: f32 = 48_000.0;
const FREQUENCY: f32 = 440.0;
const AMPLITUDE: f32 = 500.0;

use defmt::{error, info};

use crate::helper::audio::sine;

#[embassy_executor::task]
pub async fn task(mut i2s: I2S<'static, u16>) {
    const SINE_32: [u16; 8 ] = [
    0, 0,
    951, 951,
    827, 827,
    951, 951,
];

    Timer::after_secs(5).await;
    i2s.write(&SINE_32);
    i2s.start();

    loop {
        // Send 256 stereo frames to the CS43L22
        if let Err(e) = i2s.write(&SINE_32).await {
        }
    }
}

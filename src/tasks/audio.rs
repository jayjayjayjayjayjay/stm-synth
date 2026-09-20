use embassy_stm32::gpio::Output;
use embassy_stm32::peripherals::I2C1;
use embassy_stm32::{
    i2s::{Error as I2SError, I2S},
    mode::Async,
};
use embassy_time::Timer;

use defmt::{error, info};

#[embassy_executor::task]
pub async fn task(mut i2s: I2S<'static, u16>) {
    static AUDIO_DATA: [u16; 256] = [0; 256];
    i2s.start();

    loop {
        Timer::after_secs(1).await;
    }
}

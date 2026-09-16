use embassy_stm32::peripherals::I2C1;
use embassy_stm32::{
    i2c::{I2c, Master},
    mode::Async,
};
use embassy_stm32::gpio::{Output};
use embassy_time::Timer;

use defmt::info;

 pub async fn initalise(i2c: &mut I2c<'static, Async, Master>, reset_pin: &mut Output<'static>) {
    reset_pin.set_high(); 
    Timer::after_secs(1).await;
    info!("Init dac");

    let mut data = [0x00,0x99];
    i2c.write(0x4A, &data).await;

    let mut data = [0x47,0x80];
    i2c.write(0x4A, &data).await;

    let mut write_data = [0x1];
    let mut read_data = [0x0];
    i2c.write_read(0x4A, &mut write_data, &mut read_data).await;
    info!("DATA:{}", read_data[0] >> 3);


}

#[embassy_executor::task]
pub async fn task(mut i2c: I2c<'static, Async, Master>, mut reset_pin: Output<'static>) {
    initalise(&mut i2c, &mut reset_pin).await;

    loop {
        info!("Task!");
        Timer::after_secs(1).await;
    }
}

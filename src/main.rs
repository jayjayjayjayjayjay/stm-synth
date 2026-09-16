#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::i2c::{Config as I2cConfig, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, bind_interrupts, dma, i2c, peripherals};
use embassy_time::Timer;

use panic_probe as _;

mod tasks;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C1>;

    DMA1_STREAM0 => dma::InterruptHandler<peripherals::DMA1_CH0>;
    DMA1_STREAM6 => dma::InterruptHandler<peripherals::DMA1_CH6>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);
    let mut led = Output::new(p.PD14, Level::High, Speed::Low);
    let mut dac_reset_pin = Output::new(p.PD4, Level::Low, Speed::Low);

    let mut config = I2cConfig::default();
    config.frequency = Hertz(400_000);

    let i2c = I2c::new(
        p.I2C1, p.PB6,      // SCL
        p.PB9,      // SDA
        p.DMA1_CH6, // TX
        p.DMA1_CH0, // RX
        Irqs, config,
    );

    spawner.spawn(tasks::cs43l22::task(i2c, dac_reset_pin).unwrap());

    loop {
        info!("Hello World!");
        led.set_high();
        Timer::after_secs(1).await;
        led.set_low();
        Timer::after_secs(1).await;
    }
}

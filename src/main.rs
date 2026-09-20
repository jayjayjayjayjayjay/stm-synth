#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Input, Speed, Pull};
use embassy_stm32::i2c::{Config as I2cConfig, I2c};
use embassy_stm32::i2s::{Config as I2sConfig, I2S};
use embassy_stm32::rcc::{Pll, PllMul, PllPreDiv, PllRDiv};
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, bind_interrupts, dma, i2c, i2s, peripherals};
use embassy_time::Timer;
use static_cell::StaticCell;

use panic_probe as _;

mod tasks;
mod types;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<embassy_stm32::peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<embassy_stm32::peripherals::I2C1>;


    DMA1_STREAM0 => dma::InterruptHandler<peripherals::DMA1_CH0>;
    DMA1_STREAM5 => dma::InterruptHandler<peripherals::DMA1_CH5>;
    DMA1_STREAM6 => dma::InterruptHandler<peripherals::DMA1_CH6>;
});

static AUDIO_BUFFER: StaticCell<[u16; 256]> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = Config::default();

    config.rcc.plli2s = Some(Pll {
        prediv: PllPreDiv::DIV8,
        mul: PllMul::MUL192,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV3),
    });


    let p = embassy_stm32::init(config);
    let mut led = Output::new(p.PD14, Level::High, Speed::Low);
    let mut dac_reset_pin = Output::new(p.PD4, Level::Low, Speed::Low);
    let mut button = Input::new(p.PA0, Pull::Down);

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);

    let mut i2s_config = I2sConfig::default();

    let audio_buffer = AUDIO_BUFFER.init([0u16; 256]);

    let i2c = I2c::new(
        p.I2C1, p.PB6,      // SCL
        p.PB9,      // SDA
        p.DMA1_CH6, // TX
        p.DMA1_CH0, // RX
        Irqs, i2c_config,
    );
    let i2s = I2S::new_txonly(
        p.SPI3,
        p.PC12,
        p.PA4,
        p.PC10,
        p.PC7,
        p.DMA1_CH5,
        audio_buffer,
        Irqs,
        i2s_config,
    );

    spawner.spawn(tasks::cs43l22::task(i2c, dac_reset_pin, button).unwrap());
    spawner.spawn(tasks::audio::task(i2s).unwrap());

    loop {
        //info!("Hello World!");
        led.set_high();
        Timer::after_secs(1).await;
        led.set_low();
        Timer::after_secs(1).await;
    }
}

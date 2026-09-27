#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pull, Speed};

use embassy_stm32::adc::{Adc, AdcChannel};
use embassy_stm32::exti;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::i2c::{Config as I2cConfig, I2c};
use embassy_stm32::i2s::{Config as I2sConfig, I2S};
use embassy_stm32::interrupt::typelevel;
use embassy_stm32::peripherals::ADC1;
use embassy_stm32::rcc::*;
use embassy_stm32::rcc::{Pll, PllMul, PllPreDiv, PllRDiv};
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, adc, bind_interrupts, dma, i2c, peripherals};
use embassy_time::Timer;
use static_cell::StaticCell;

use panic_probe as _;

mod helper;
mod tasks;
mod types;

use types::audio::{AUDIO_BUFFER_SIZE, UserInputs};

bind_interrupts!(struct Irqs {

    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;


    DMA1_STREAM0 => dma::InterruptHandler<peripherals::DMA1_CH0>;
    DMA1_STREAM5 => dma::InterruptHandler<peripherals::DMA1_CH5>;
    DMA1_STREAM6 => dma::InterruptHandler<peripherals::DMA1_CH6>;

    EXTI0 => exti::InterruptHandler<typelevel::EXTI0>;
});

static AUDIO_BUFFER: StaticCell<[u16; AUDIO_BUFFER_SIZE]> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    }); // Bypass on Nucleo (ST-LINK MCO)
    config.rcc.pll_src = PllSource::HSE;

    config.rcc.plli2s = Some(Pll {
        prediv: PllPreDiv::DIV4,
        mul: PllMul::MUL172,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV7),
    });

    let p = embassy_stm32::init(config);
    let _led = Output::new(p.PD14, Level::High, Speed::Low);
    let dac_reset_pin = Output::new(p.PD4, Level::Low, Speed::Low);
    let button_exti = ExtiInput::new(p.PA0, p.EXTI0, Pull::Down, Irqs);

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);

    let mut i2s_config = I2sConfig::default();
    i2s_config.frequency = Hertz(48000);

    let audio_buffer = AUDIO_BUFFER.init([0u16; AUDIO_BUFFER_SIZE]);

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
    let adc = Adc::new(p.ADC1);
    let pot_pin = p.PA1.degrade_adc();
    let pot_pin_2 = p.PA2.degrade_adc();
    spawner.spawn(tasks::pot_read::task(adc,pot_pin,pot_pin_2).unwrap());
    spawner.spawn(tasks::cs43l22::task(i2c, dac_reset_pin, button_exti).unwrap());

    spawner.spawn(tasks::audio::task(i2s, spawner).unwrap());

    loop {
        //info!("Hello World!");
        Timer::after_secs(10).await;
    }
}

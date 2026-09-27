use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_stm32::adc::Adc;
use embassy_stm32::peripherals::ADC1;

pub const AUDIO_BUFFER_SIZE: usize = 480;

pub type AudioChannel = Channel<ThreadModeRawMutex, [u16; AUDIO_BUFFER_SIZE], 3>;

pub struct AdcInputs{
    pub pot_1_adc: Adc<'static, ADC1>,

}

pub struct UserInputs {
    pub pot_1_adc: Adc<'static, ADC1>,
}

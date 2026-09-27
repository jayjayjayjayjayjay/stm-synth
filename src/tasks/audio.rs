use embassy_executor::Spawner;
use embassy_stm32::i2s::I2S;
use embassy_stm32::peripherals::ADC1;
use embassy_time::Timer;

use embassy_sync::channel::Channel;

use crate::tasks;
use crate::types::audio as AudioTypes;

static audio_generator: AudioTypes::AudioChannel = Channel::new();

#[embassy_executor::task]
pub async fn task(
    mut i2s: I2S<'static, u16>,
    spawner: Spawner,
) {
    spawner.spawn(tasks::audio_generator::task(&audio_generator).unwrap());
    Timer::after_secs(5).await;
    let rx_buffer = audio_generator.receive().await;
    i2s.write(&rx_buffer);
    i2s.start();

    loop {
        let rx_buffer = audio_generator.receive().await;
        i2s.write(&rx_buffer).await;
    }
}

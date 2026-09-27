use crate::prelude::*;

use embassy_executor::Spawner;
use embassy_stm32::i2s::I2S;
use embassy_time::Timer;

use embassy_sync::channel::Channel;

use crate::tasks;
use types::audio as AudioTypes;

static AUDIO_GENERATOR: AudioTypes::AudioChannel = Channel::new();

#[embassy_executor::task]
pub async fn task(
    mut i2s: I2S<'static, u16>,
    spawner: Spawner,
) {
    spawner.spawn(tasks::audio_generator::task(&AUDIO_GENERATOR).unwrap());
    Timer::after_secs(5).await;
    let rx_buffer = AUDIO_GENERATOR.receive().await;
    i2s.write(&rx_buffer);
    i2s.start();

    loop {
        let rx_buffer = AUDIO_GENERATOR.receive().await;
        i2s.write(&rx_buffer).await;
    }
}

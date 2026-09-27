use embassy_stm32::adc::{Adc, Instance};
use defmt::{error, info};
use core::sync::atomic::{AtomicU16, Ordering};

use crate::helper::audio::SineWave;
use crate::tasks::pot_read::POT_VALUE;
use crate::types::audio as AudioTypes;
use crate::types::audio::{AUDIO_BUFFER_SIZE, AudioChannel};

#[embassy_executor::task]
pub async fn task(channel: &'static AudioChannel) {
    let mut freq = 200.0;
    let mut sine = SineWave::new(
        freq, 48_000.0, 400.0, // 25% of i16 full scale
    );
    let mut buffer = [0u16; AUDIO_BUFFER_SIZE];

    let _flag = 0;
    sine.fill_buffer(&mut buffer);
    channel.send(buffer).await;
    let mut pot_value = POT_VALUE.load(Ordering::Relaxed);

    loop {
        sine.fill_buffer(&mut buffer);
        channel.send(buffer).await;
        freq = (POT_VALUE.load(Ordering::Relaxed) + 50).into();
        sine.change_frequency(freq);
    }
}

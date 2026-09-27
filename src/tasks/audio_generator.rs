use crate::prelude::*;

use core::sync::atomic::Ordering;

use crate::helper::audio::crossfade_buffers;
use crate::helper::FmWave::{FmWave};
use crate::helper::SineWave::{SineWave};
use crate::tasks::potentiometer_read::{POTENTIOMETER_VALUE, POTENTIOMETER_VALUE_2};
use types::audio::{AUDIO_BUFFER_SIZE, AudioChannel};

#[embassy_executor::task]
pub async fn task(channel: &'static AudioChannel) {
    let mut freq = 200.0;
    let _mix = 0;
    let mut sine = SineWave::new(
        freq, 48_000.0, 200.0, // 25% of i16 full scale
    );
    let mut fm = FmWave::new(
        freq, 48_000.0, 200.0, // 25% of i16 full scale
    );
    let mut sine_buffer = [0u16; AUDIO_BUFFER_SIZE];
    let mut fm_buffer = [0u16; AUDIO_BUFFER_SIZE];
    let mut mix = POTENTIOMETER_VALUE_2.load(Ordering::Relaxed) + 30;
    let mut mix_buffer = [0u16; AUDIO_BUFFER_SIZE];

    sine.fill_buffer(&mut sine_buffer);
    fm.fill_buffer(&mut fm_buffer);
    channel.send(mix_buffer).await;

    loop {
        sine.fill_buffer(&mut sine_buffer);
        fm.fill_buffer(&mut fm_buffer);
        crossfade_buffers(&fm_buffer, &sine_buffer, &mut mix_buffer, mix);
        //info!("mix:{}, fm:{}", mix_buffer[0], fm_buffer[0] );
        channel.send(mix_buffer).await;
        freq = (POTENTIOMETER_VALUE.load(Ordering::Relaxed) + 30).into();
        sine.change_frequency(freq);
        fm.change_frequency(freq);
        mix = POTENTIOMETER_VALUE_2.load(Ordering::Relaxed) + 30;
    }
}

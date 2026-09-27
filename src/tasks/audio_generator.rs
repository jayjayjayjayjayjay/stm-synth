use core::sync::atomic::{AtomicU16, Ordering};
use defmt::{error, info};
use embassy_stm32::adc::{Adc, Instance};

use crate::helper::audio::{SineWave, SquareWave, crossfade_buffers};
use crate::tasks::pot_read::{POT_VALUE, POT_VALUE_2};
use crate::types::audio as AudioTypes;
use crate::types::audio::{AUDIO_BUFFER_SIZE, AudioChannel};

#[embassy_executor::task]
pub async fn task(channel: &'static AudioChannel) {
    let mut freq = 200.0;
    let mut mix = 0;
    let mut sine = SineWave::new(
        freq, 48_000.0, 200.0, // 25% of i16 full scale
    );
    let mut square = SquareWave::new(
        freq, 48_000.0, 1200.0, // 25% of i16 full scale
    );
    let mut sine_buffer = [0u16; AUDIO_BUFFER_SIZE];
    let mut square_buffer = [0u16; AUDIO_BUFFER_SIZE];
    let mut mix = ((POT_VALUE_2.load(Ordering::Relaxed) + 30) / 42);
    let mut mix_buffer = [0u16; AUDIO_BUFFER_SIZE];

    let _flag = 0;
    sine.fill_buffer(&mut sine_buffer);
    square.fill_buffer(&mut square_buffer);
    channel.send(mix_buffer).await;

    loop {
        //info!("{}", square_buffer);
        sine.fill_buffer(&mut sine_buffer);
        square.fill_buffer(&mut square_buffer);
        crossfade_buffers(&sine_buffer, &square_buffer, &mut mix_buffer, mix);
        channel.send(mix_buffer).await;
        freq = (POT_VALUE.load(Ordering::Relaxed) + 30).into();
        sine.change_frequency(freq);
        square.change_frequency(freq);
        mix = ((POT_VALUE_2.load(Ordering::Relaxed) + 30) / 41);
    }
}

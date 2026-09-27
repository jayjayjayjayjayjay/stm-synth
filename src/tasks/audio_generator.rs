
use crate::helper::audio::SineWave;
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
    loop {
        sine.fill_buffer(&mut buffer);
        channel.send(buffer).await;
        freq += 1.0;
        sine.change_frequency(freq);
    }
}

use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

pub const AUDIO_BUFFER_SIZE: usize = 480;

pub type AudioChannel = Channel<
    ThreadModeRawMutex,
    [u16; AUDIO_BUFFER_SIZE],
    3,
>;

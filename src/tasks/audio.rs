use embassy_stm32::i2s::I2S;
use embassy_time::Timer;


const SAMPLE_RATE: f32 = 48_000.0;
const FREQUENCY: f32 = 440.0;
const AMPLITUDE: f32 = 500.0;



#[embassy_executor::task]
pub async fn task(mut i2s: I2S<'static, u16>) {
    const SINE_32: [u16; 8 ] = [
    0, 0,
    951, 951,
    827, 827,
    951, 951,
];

    Timer::after_secs(5).await;
    i2s.write(&SINE_32);
    i2s.start();

    loop {
        // Send 256 stereo frames to the CS43L22
        if let Err(_e) = i2s.write(&SINE_32).await {
        }
    }
}

use core::sync::atomic::{AtomicU16, Ordering};
use embassy_stm32::peripherals::ADC1;
use embassy_stm32::adc::{Adc, AdcChannel, AnyAdcChannel, SampleTime};
use embassy_time::Timer;

pub static POT_VALUE: AtomicU16 = AtomicU16::new(0);

#[embassy_executor::task]
pub async fn task(mut adc: Adc<'static, ADC1>, mut pin: AnyAdcChannel<'static, ADC1>) {
    //adc.set_sample_time(SampleTime::CYCLES112);

    loop {
        // A single conversion at CYCLES112 takes low single-digit microseconds —
        // blocking here is fine, it only blocks this task, not your audio path.
        let raw = adc.blocking_read(&mut pin,SampleTime::CYCLES112);
        POT_VALUE.store(raw, Ordering::Relaxed);

        // Knobs don't need sub-millisecond responsiveness — pick a poll rate
        // that feels right (5-20ms is plenty for smooth-feeling parameter control).
        Timer::after_millis(10).await;
    }
}

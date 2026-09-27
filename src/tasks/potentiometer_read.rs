use core::sync::atomic::{AtomicU16, Ordering};
use embassy_stm32::peripherals::ADC1;
use embassy_stm32::adc::{Adc, AdcChannel, AnyAdcChannel, SampleTime};
use embassy_time::Timer;

// TODO improve logic, maybe macro and seperate task creation?
pub static POTENTIOMETER_VALUE: AtomicU16 = AtomicU16::new(0);
pub static POTENTIOMETER_VALUE_2: AtomicU16 = AtomicU16::new(0);

#[embassy_executor::task]
pub async fn task(mut adc: Adc<'static, ADC1>, mut pin: AnyAdcChannel<'static, ADC1>, mut pin2: AnyAdcChannel<'static, ADC1>) {
    //adc.set_sample_time(SampleTime::CYCLES112);

    loop {
        let raw = adc.blocking_read(&mut pin,SampleTime::CYCLES112);
        POTENTIOMETER_VALUE.store(raw, Ordering::Relaxed);

        let raw = adc.blocking_read(&mut pin2,SampleTime::CYCLES112);
        POTENTIOMETER_VALUE_2.store(raw, Ordering::Relaxed);

        Timer::after_millis(10).await;
    }
}

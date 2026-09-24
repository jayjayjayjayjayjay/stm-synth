use core::f32::consts::TAU;
use libm::sinf;
/// f32 -> `u16` (holding i16 bits). Clamps to full scale instead of wrapping.
#[inline]
pub fn from_f32(x: f32) -> u16 {
    ((x.clamp(-1.0, 1.0) * 32767.0) as i16) as u16
}

pub fn periodic(
    buf: &mut [u16],
    sample_rate: u32,
    freq_hz: f32,
    amp: f32,
    wave: impl Fn(f32) -> f32,
) {
    let step = freq_hz as f64 / sample_rate as f64;
    for (n, out) in buf.iter_mut().enumerate() {
        // f64 phase keeps long buffers from drifting out of tune
        let phase = ((n as f64 * step) % 1.0) as f32;
        *out = from_f32(amp * wave(phase));
    }
}
 
pub fn sine(buf: &mut [u16], sample_rate: u32, freq_hz: f32, amp: f32) {
    periodic(buf, sample_rate, freq_hz, amp, |p| sinf((TAU * p)));
}

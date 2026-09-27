use core::f32::consts::PI;

pub struct SquareWave {
    phase: f32,
    phase_increment: f32,
    amplitude: f32,
    sample_rate: f32,
}

impl SquareWave {
    pub fn new(frequency: f32, sample_rate: f32, amplitude: f32) -> Self {
        Self {
            phase: 0.0,
            phase_increment: 2.0 * PI * 2.0 * frequency / sample_rate,
            amplitude,
            sample_rate,
        }
    }

    pub fn change_frequency(&mut self,frequency :f32) {
        self.phase_increment = 2.0 * PI * 2.0 * frequency / self.sample_rate;

    }


   pub fn next_sample(&mut self) -> u16 {
        let sample = if self.phase < PI {
            self.amplitude
        } else {
            -self.amplitude
        };

        self.phase += self.phase_increment;

        if self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        sample as i16 as u16
    }
    pub fn fill_buffer(&mut self, buffer: &mut [u16]) {
        for frame in buffer.as_chunks_mut::<2>().0 {
            let sample = self.next_sample();

            frame[0] = sample; // Left
            frame[1] = sample; // Right
        }
    }
}

// Takes min max of adc
// Maybe rework to take percentage
pub fn crossfade_buffers(a: &[u16], b: &[u16], out: &mut [u16], mix: u16) {
    const MIX_MAX: i32 = 4095;
    let mix = mix.min(4095) as i32;
    let gain_b = mix;
    let gain_a = MIX_MAX - mix;

    for ((&sa, &sb), so) in a.iter().zip(b.iter()).zip(out.iter_mut()) {
        let sample_a = sa as i16 as i32;
        let sample_b = sb as i16 as i32;

        let sum = sample_a * gain_a + sample_b * gain_b;
        let result = (sum / MIX_MAX).clamp(i16::MIN as i32, i16::MAX as i32);

        *so = result as i16 as u16;
    }
}

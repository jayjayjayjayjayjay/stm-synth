use core::f32::consts::PI;
use micromath::F32Ext;

pub struct SineWave {
    phase: f32,
    phase_increment: f32,
    amplitude: f32,
    sample_rate: f32,
}

impl SineWave {
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
        let sample = self.phase.sin() * self.amplitude;

        self.phase += self.phase_increment;

        if self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        sample as i16 as u16
    }

    pub fn fill_buffer(&mut self, buffer: &mut [u16]) {
        /*for sample in buffer.iter_mut() {
            *sample = self.next_sample();
        }*/
        for frame in buffer.as_chunks_mut::<2>().0 {
            let sample = self.next_sample();

            frame[0] = sample; // Left
            frame[1] = sample; // Right
        }
    }
}

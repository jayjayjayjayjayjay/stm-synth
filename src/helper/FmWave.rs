use core::f32::consts::PI;
use micromath::F32Ext;

pub struct FmWave {
    carrier_phase: f32,
    modulator_phase: f32,
    carrier_increment: f32,
    modulator_increment: f32,
    mod_index: f32,   // how much the modulator warps the carrier — try 2.0-8.0
    amplitude: f32,
    sample_rate: f32,
    ratio: f32,       // modulator freq as a multiple of carrier freq — try 1.0, 2.0, 1.5
}

impl FmWave {
    pub fn new(frequency: f32, sample_rate: f32, amplitude: f32) -> Self {
        let ratio = 2.3;
        let mod_index = 2.0;

        Self {
            carrier_phase: 0.0,
            modulator_phase: 0.0,
            carrier_increment: 2.0 * PI * frequency / sample_rate,
            modulator_increment: 2.0 * PI * frequency * ratio / sample_rate,
            mod_index,
            amplitude,
            sample_rate,
            ratio,
        }
    }

    pub fn change_frequency(&mut self, frequency: f32) {
        self.carrier_increment = 2.0 * PI * frequency / self.sample_rate;
        self.modulator_increment = 2.0 * PI * frequency * self.ratio / self.sample_rate;
    }

    pub fn next_sample(&mut self) -> u16 {
        // Modulator warps the carrier's phase — this is what generates the
        // extra harmonics/sidebands that make FM sound richer than a plain sine.
        let modulator = self.modulator_phase.sin() * self.mod_index;
        let sample = (self.carrier_phase + modulator).sin() * self.amplitude;

        self.carrier_phase += self.carrier_increment;
        self.modulator_phase += self.modulator_increment;

        if self.carrier_phase >= 2.0 * PI {
            self.carrier_phase -= 2.0 * PI;
        }
        if self.modulator_phase >= 2.0 * PI {
            self.modulator_phase -= 2.0 * PI;
        }

        sample as i16 as u16
    }

    pub fn fill_buffer(&mut self, buffer: &mut [u16]) {
        for frame in buffer.chunks_exact_mut(2) {
            let sample = self.next_sample();

            frame[0] = sample; // Left
            frame[1] = sample; // Right
        }
    }
}

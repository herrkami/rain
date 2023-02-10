/// Convenience trait to convert between different frequency units
/// mHz is millihertz, bHz is 1/1024 Hz
trait Frequency<T> {
    fn to_mHz(&self) -> T;
    fn from_mHz(&self) -> T;
    fn to_bHz(&self) -> T;
    fn from_bHz(&self) -> T;
}

impl Frequency<u32> for u32 {
    fn to_mHz(&self) -> u32 {
        self * 1000
    }
    fn from_mHz(&self) -> u32 {
        self / 1000
    }
    fn to_bHz(&self) -> u32 {
        self * 1024
    }
    fn from_bHz(&self) -> u32 {
        self / 1024
    }
}

/// Stateful wavetable signal generator
pub struct WaveTableOscillator<T: 'static> {
    repeat: bool,
    running: bool,

    mfreq: u32,
    msample_rate: u32,

    wavetable: &'static [T],

    phi: i32,
    phi_max: i32,
    delta_phi: i32,

    idx: usize,
    idx_max: usize,
}

impl<T> WaveTableOscillator<T> {
    fn update_idx(&mut self) {
        self.idx = (((self.idx_max as i32) * self.phi) / self.phi_max) as usize;
    }

    fn update_delta_phi(&mut self) {
        self.delta_phi =
            (((self.mfreq as i64) * (self.phi_max as i64)) / (self.msample_rate as i64)) as i32;
    }

    /// Set repeat to true or false
    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }

    /// Set the generator into "running" mode
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the generator (disable "running" mode)
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Resets the phase accumulator
    pub fn reset(&mut self) {
        self.phi = 0;
    }

    /// Resets the phase accumulator and set the generator into "running" mode
    pub fn reset_and_start(&mut self) {
        self.reset();
        self.running = true;
    }

    /// Stops the generator and reset the phase accumulator
    pub fn stop_and_reset(&mut self) {
        self.running = false;
        self.reset();
    }

    /// Returns whether the generator is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Sets the frequency
    pub fn set_mfreq(&mut self, mfreq: u32) {
        self.mfreq = mfreq;
        self.update_delta_phi();
    }

    pub fn set_freq(&mut self, freq: u32) {
        self.mfreq = freq.to_mHz();
        self.update_delta_phi();
    }

    pub fn set_msample_rate(&mut self, msample_rate: u32) {
        self.msample_rate = msample_rate;
        self.update_delta_phi();
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.msample_rate = sample_rate.to_mHz();
        self.update_delta_phi();
    }
}

impl WaveTableOscillator<i32> {
    pub fn new() -> Self {
        Self {
            repeat: true,
            running: false,

            mfreq: 440.to_mHz(),
            msample_rate: 44100.to_mHz(),

            wavetable: &[],

            phi: 0,
            phi_max: 1 << 16,
            delta_phi: 0,

            idx: 0,
            idx_max: 0,
        }
    }

    /// Increments phase accumulator and returns either the next sample or None
    /// if the generator is not running
    pub fn _next(&mut self) -> Option<i32> {
        self.phi += self.delta_phi;
        if self.phi > self.phi_max {
            self.phi -= self.phi_max;
            if !self.repeat {
                self.stop_and_reset();
            }
        };
        if self.is_running() {
            self.update_idx();
            Some(self.wavetable[self.idx])
        } else {
            None
        }
    }

    /// Set the wavetable
    pub fn set_wavetable(&mut self, wavetable: &'static [i32]) {
        self.wavetable = wavetable;
        self.idx_max = self.wavetable.len();
    }
}

impl Iterator for WaveTableOscillator<i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self._next()
    }
}

impl WaveTableOscillator<i16> {
    pub fn new() -> Self {
        Self {
            repeat: true,
            running: false,

            mfreq: 440.to_mHz(),
            msample_rate: 44100.to_mHz(),

            wavetable: &[],

            phi: 0,
            phi_max: 1 << 16,
            delta_phi: 0,

            idx: 0,
            idx_max: 0,
        }
    }

    /// Increments phase accumulator and returns either the next sample or None
    /// if the generator is not running
    pub fn _next(&mut self) -> Option<i16> {
        self.phi += self.delta_phi;
        if self.phi > self.phi_max {
            self.phi -= self.phi_max;
            if !self.repeat {
                self.stop_and_reset();
            }
        };
        if self.is_running() {
            self.update_idx();
            let out = self.wavetable[self.idx];
            Some(out)
        } else {
            None
        }
    }

    /// Set the wavetable
    pub fn set_wavetable(&mut self, wavetable: &'static [i16]) {
        self.wavetable = wavetable;
        self.idx_max = self.wavetable.len();
    }
}

impl Iterator for WaveTableOscillator<i16> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self._next()
    }
}

use core::time::Duration;
use rodio::source::Source;
impl Source for WaveTableOscillator<i16> {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.msample_rate.from_mHz();
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

pub type WaveTableOsc16 = WaveTableOscillator<i16>;
pub type WaveTableOsc32 = WaveTableOscillator<i32>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::osc::wave_tables::SINE_I16;

    #[test]
    fn test_wave_table_oscillator() {
        let mut osc = WaveTableOscillator::<i16>::new();
        osc.set_wavetable(&SINE_I16);
        osc.set_freq(2);
        osc.set_sample_rate(100);
        osc.set_repeat(false);
        osc.start();
        for x in 0..110 {
            let _y = osc._next();
            match _y {
                Some(y) => println!("{}: {}\n", x, (y as f64) / (i32::MAX as f64)),
                None => {
                    println!("Generator stopped at {}", x);
                    break;
                }
            }
        }
    }
}

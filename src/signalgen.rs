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
pub struct SignalGenerator<T: 'static> {
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

impl<T> SignalGenerator<T> {
    fn update_idx(&mut self) {
        self.idx = (((self.idx_max as i32) * self.phi) / self.phi_max) as usize;
    }

    fn update_delta_phi(&mut self) {
        self.delta_phi = ((self.mfreq as i32) * self.phi_max) / (self.msample_rate as i32);
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

    pub fn set_msamplerate(&mut self, msample_rate: u32) {
        self.msample_rate = msample_rate;
        self.update_delta_phi();
    }

    pub fn set_samplerate(&mut self, sample_rate: u32) {
        self.msample_rate = sample_rate.to_mHz();
        self.update_delta_phi();
    }
}

impl SignalGenerator<i32> {
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

impl Iterator for SignalGenerator<i32> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        return self._next();
    }
}

impl SignalGenerator<i16> {
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

impl Iterator for SignalGenerator<i16> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        return self._next();
    }
}

use core::time::Duration;
use rodio::source::Source;
impl Source for SignalGenerator<i16> {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::wavetables::SINE_I16;

    #[test]
    fn test_signal_generator() {
        let mut siggen = SignalGenerator::<i16>::new();
        siggen.set_wavetable(&SINE_I16);
        siggen.set_freq(2);
        siggen.set_samplerate(100);
        siggen.set_repeat(false);
        siggen.start();
        for x in 0..110 {
            let _y = siggen._next();
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

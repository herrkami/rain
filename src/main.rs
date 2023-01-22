use core::time::Duration;
use rain::{
    linexp::LinExp,
    signalgen::{self, SignalGenerator},
    wavetables::SINE_I32,
};
use rodio::{source::Source, OutputStream};

// impl Source for signalgen::SignalGenerator<i32> {
//     fn channels(&self) -> u16 {
//         return 1;
//     }

//     fn sample_rate(&self) -> u32 {
//         return self.sample_rate;
//     }

//     fn current_frame_len(&self) -> Option<usize> {
//         return None;
//     }

//     fn total_duration(&self) -> Option<Duration> {
//     fn total_duration(&self) -> Option<Duration> {
//         return None;
//     }
// }

// impl Iterator for WavetableOscillator {
//     type Item = f32;

//     fn next(&mut self) -> Option<Self::Item> {
//         return Some(self.get_sample());
//     }
// }

fn main() {
    // print!("Hello World!\n");
    // print!("{:?}\n", wavetables::SINE_I8);

    // let mut linexp = LinExp::new();
    // let sigma_max = linexp.get_sigma_max();
    // let norm = linexp.get_norm();
    // linexp.set_sigma(sigma_max - sigma_max / 16);
    // for x in (0..norm + 1).step_by(norm as usize / 16) {
    //     print!("{:?}\n", linexp.y(x));
    // }

    let mut siggen = SignalGenerator::new();
    siggen.set_wavetable(&SINE_I32);
    siggen.set_freq(1);
    siggen.set_samplerate(100);
    for x in 0..110 {
        let y = siggen.next();
        print!("{}: {}\n", x, (y as f64) / (i32::MAX as f64));
    }
}

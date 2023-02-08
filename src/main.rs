use core::time::Duration;
use rain::{
    linexp::LinExp,
    signalgen::{self, SignalGenerator},
    wavetables::{SINE_I16, SINE_I32},
};
use rodio::{source::Source, OutputStream};

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

    let mut siggen = SignalGenerator::<i16>::new();
    siggen.set_wavetable(&SINE_I16);
    siggen.set_freq(440);
    siggen.set_samplerate(100);
    siggen.set_repeat(false);
    siggen.start();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(siggen.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(5));

    // for x in 0..110 {
    //     let _y = siggen.next();
    //     match _y {
    //         Some(y) => println!("{}: {}\n", x, (y as f64) / (i16::MAX as f64)),
    //         None => {
    //             println!("Generator stopped at {}", x);
    //             break;
    //         }
    //     }
    // }
}

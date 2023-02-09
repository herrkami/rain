use core::time::Duration;
use rain::{
    linexp::LinExp,
    signalgen::{self, SignalGenerator},
    wavetables::SINE_I16,
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

    let mut sine = SignalGenerator::<i16>::new();
    sine.set_wavetable(&SINE_I16);
    sine.set_freq(440);
    sine.set_samplerate(44100);
    sine.set_repeat(true);
    sine.start();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(sine.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(1));

    // for x in 0..110 {
    //     let _y = siggen.next();
    //     match _y {
    //         Some(y) => println!("{}: {}, {}\n", x, (y as f64) / (i16::MAX as f64), y),
    //         None => {
    //             println!("Generator stopped at {}", x);
    //             break;
    //         }
    //     }
    // }
}

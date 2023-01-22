trait Frequency<T> {
    fn to_mHz(&self) -> T;
    fn from_mHz(&self) -> T;
    fn to_bHz(&self) -> T;
    fn from_bHz(&self) -> T;
}

impl Frequency<i32> for i32 {
    fn to_mHz(&self) -> i32 {
        self * 1000
    }
    fn from_mHz(&self) -> i32 {
        self / 1000
    }
    fn to_bHz(&self) -> i32 {
        self * 1024
    }
    fn from_bHz(&self) -> i32 {
        self / 1024
    }
}

pub struct SignalGenerator<T: 'static> {
    repeat: bool,
    phi: T,
    phi_max: T,
    delta_phi: T,
    cfreq: T,
    csample_rate: T,
    wavetable: &'static [T],
    idx: usize,
    idx_max: usize,
}

impl SignalGenerator<i32> {
    pub fn new() -> Self {
        Self {
            repeat: true,

            cfreq: 440.to_mHz(),
            csample_rate: 44100.to_mHz(),

            phi: 0,
            phi_max: 1 << 16,
            delta_phi: 0,

            idx: 0,
            idx_max: 0,

            wavetable: &[],
        }
    }
    fn update_idx(&mut self) {
        self.idx = ((self.idx_max as i32 * self.phi) / self.phi_max) as usize;
    }
    fn update_delta_phi(&mut self) {
        self.delta_phi = (self.cfreq * self.phi_max) / self.csample_rate;
    }
    pub fn next(&mut self) -> i32 {
        self.phi += self.delta_phi;
        if self.phi > self.phi_max {
            self.phi -= self.phi_max
        };
        self.update_idx();
        self.wavetable[self.idx]
    }
    pub fn set_wavetable(&mut self, wavetable: &'static [i32]) {
        self.wavetable = wavetable;
        self.idx_max = self.wavetable.len();
    }
    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }
    pub fn set_cfreq(&mut self, cfreq: i32) {
        self.cfreq = cfreq;
        self.update_delta_phi();
    }
    pub fn set_freq(&mut self, freq: i32) {
        self.cfreq = freq.to_mHz();
        self.update_delta_phi();
    }
    pub fn set_csamplerate(&mut self, csample_rate: i32) {
        self.csample_rate = csample_rate;
        self.update_delta_phi();
    }
    pub fn set_samplerate(&mut self, sample_rate: i32) {
        self.csample_rate = sample_rate.to_mHz();
        self.update_delta_phi();
    }
}

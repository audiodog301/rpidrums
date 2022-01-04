pub struct Sine {
    phase: f32,
    freq: f32,
    sample_rate: f32,
}

impl Sine {
    pub fn new(freq: f32, sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            freq,
            sample_rate,
        }
    }

    pub fn process(&mut self) -> f32 { //lol credit to WeirdConstructor for this code i hate writing oscillators
        let output = (self.phase * std::f32::consts::TAU).sin();
        self.phase = (self.phase + self.freq / self.sample_rate).fract();
        output
    }
}
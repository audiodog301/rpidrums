pub enum Instruction {
    Kick,
}

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

    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq;
    }

    pub fn process(&mut self) -> f32 {
        //lol credit to WeirdConstructor for this code i hate writing oscillators
        let output = (self.phase * std::f32::consts::TAU).sin();
        self.phase = (self.phase + self.freq / self.sample_rate).fract();
        output
    }
}

pub struct Envelope {
    sample_rate: f32,
    decay: f32,
    counter: i32,
    val: f32,
}

impl Envelope{
    pub fn new(sample_rate: f32, decay: f32) -> Self {
        Self {
            sample_rate,
            decay,
            counter: 0,
            val: 1.0,
        }
    }

    pub fn trigger(&mut self) {
        self.val = 1.0;
    }

    pub fn process(&mut self) -> f32 {
        if self.counter < (self.decay * self.sample_rate) as i32 && self.val > 0.0{
            self.val -= 1.0 / ((self.decay * self.sample_rate) as f32);
            self.counter += 1;
        } else {
            self.counter = 0;
        }

        self.val
    }
}
// pub struct Kick {
//     osc: Sine,

// }

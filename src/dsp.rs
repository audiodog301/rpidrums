use rand::Rng;
use hound;

pub fn noise() -> f32 {
    ((rand::thread_rng().gen_range(0..10_000) as f32) / 10_000.0) - 0.5
}

pub enum Instruction {
    Kick,
    Hat,
    Sample,
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

impl Envelope {
    pub fn new(sample_rate: f32, decay: f32) -> Self {
        Self {
            sample_rate,
            decay,
            counter: 0,
            val: 0.0,
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
pub struct Kick {
    osc: Sine,
    env: Envelope,
}

impl Kick {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            osc: Sine::new(150.0, sample_rate),
            env: Envelope::new(sample_rate, 0.35)
        }
    }

    pub fn process(&mut self) -> f32 {
        self.osc.set_freq(self.env.process() * 150.0);
        self.env.process();
        self.osc.process() / 2.3
    }

    pub fn trigger(&mut self) {
        self.env.trigger();
    }
}

pub struct Hat {
    env: Envelope,
    hpf_one: HPF,
    hpf_two: HPF,
}

impl Hat {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            env: Envelope::new(sample_rate, 0.04),
            hpf_one: HPF::new(0.0, 0.25),
            hpf_two: HPF::new(0.0, 0.25),
        }
    }

    pub fn trigger(&mut self) {
        self.env.trigger();
    }

    pub fn process(&mut self) -> f32 {
        self.hpf_two.process(self.hpf_one.process(self.env.process() * noise()))
    }
}

pub struct HPF {
    former_in: f32,
    former_out: f32,
    feedback: f32,
    cutoff: f32,
}

impl HPF {
    pub fn new(feedback: f32, cutoff: f32) -> Self {
        Self {
            former_in: 0.0,
            former_out: 0.0,
            feedback,
            cutoff,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let out: f32 = (self.feedback * self.former_out) + (self.cutoff * (input - self.former_in));
        self.former_out = out;
        self.former_in = input;
        out
    }
}

pub struct LPF {
    former_in: f32,
    former_out: f32,
    feedback: f32,
    cutoff: f32,
}

impl LPF {
    pub fn new(feedback: f32, cutoff: f32) -> Self {
        Self {
            former_in: 0.0,
            former_out: 0.0,
            feedback,
            cutoff,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let out: f32 = (self.feedback * self.former_out) + (self.cutoff * (self.former_in - input));
        self.former_out = out;
        self.former_in = input;
        out
    }
}

pub struct Sampler {
    buffer: Vec<f32>,
    index: usize,
}

impl Sampler {
    pub fn new(filename: &str) -> Self {
        let mut reader = hound::WavReader::open(filename).unwrap();
        let mut buffer: Vec<f32> = Vec::new();
        for sample in reader.samples::<i32>() {
            buffer.push(((sample.unwrap() as f32) / (i32::MAX as f32)))
        }
        let len = buffer.len();
        Self {
            buffer: buffer,
            index: len + 1,
        }
    }

    pub fn trigger(&mut self) {
        self.index = 0
    }

    pub fn process(&mut self) -> f32 {
        if self.index < self.buffer.len() {
            let out = self.buffer[self.index];
            self.index += 1;
            out
        } else {
            0.0
        }
    }
}

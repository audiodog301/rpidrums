use cpal::{Sample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

mod dsp;
use dsp::Sine;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");

    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();

    let mut sine = Sine::new(110.0, 44_100.0);
    
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = Sample::from(&sine.process());
            }
        },
        move |err| {
            // react to errors here.
        },
    ).unwrap();

    stream.play().unwrap();
    
    loop {}
}
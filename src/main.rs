use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use cpal::StreamConfig;

mod dsp;
use dsp::{Instruction, Kick};

fn main() {
    let (command_sender, command_receiver) = crossbeam_channel::bounded(1024);
    
    std::thread::spawn(move || {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();

        let sample_format = supported_config.sample_format();
        let config: StreamConfig = supported_config.into();
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let mut kick = Kick::new(sample_rate);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {
                         // Try to receive a message from the gui thread
                        while let Ok(instruction) = command_receiver.try_recv() { 
                            match instruction {
                                Instruction::Kick => {
                                    kick.trigger();
                                }
                            }
                        }
                        
                        for sample in frame.iter_mut() {
                            *sample = Sample::from(&kick.process());
                        }
                    }
                },
                move |err| {
                    // react to errors here.
                },
            )
            .unwrap();

        stream.play().unwrap();

        loop {}
    });

    loop {
        command_sender.send(Instruction::Kick);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
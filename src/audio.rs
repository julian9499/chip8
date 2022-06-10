extern crate cpal;

use cpal::{Device, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub(crate) struct Opt {
    device: cpal::Device,
}

impl Opt {
    pub(crate) fn new() -> Self {
        let available_hosts = cpal::available_hosts();
        let mut default_out;
        for host_id in available_hosts {
            let host = cpal::host_from_id(host_id).unwrap();

            default_out = host.default_output_device().map(|e| e.name().unwrap());
            println!("  Default Output Device:\n    {:?}", default_out);
        }
        let host = cpal::default_host();
        let default_device = host.default_output_device().unwrap();
        Opt{ device: default_device}
    }

    pub fn beep(self) -> Stream{

        let device = self.device;

        let config = device.default_output_config().unwrap();

        match config.sample_format() {
            cpal::SampleFormat::F32 => Opt::run::<f32>(&device, &config.into()),
            cpal::SampleFormat::I16 => Opt::run::<i16>(&device, &config.into()),
            cpal::SampleFormat::U16 => Opt::run::<u16>(&device, &config.into()),
        }
    }

    fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Stream
        where
            T: cpal::Sample,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 780.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                Opt::write_data(data, channels, &mut next_value)
            },
            err_fn,
        ).unwrap();
        stream.pause();

        stream
    }

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
        where
            T: cpal::Sample,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = cpal::Sample::from::<f32>(&next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }
}



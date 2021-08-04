//! Baby monitor
//! - attaches to Usb mic using cpal audio lib
//! - monitors stream
//! - if baby is awake (according to heuristic), call phone
//! - phone number is determined by schedule, in config file

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() ->Result<()> {
    let host = cpal::default_host();
    for input_device in host.input_devices()? {
        if &input_device.name()? == "plughw:CARD=USB,DEV=0" {
            println!("{}", input_device.name()?);
            let config = input_device.default_input_config()?.into();
            let stream = input_device.build_input_stream(&config,
            move |data: &[f32],_: &cpal::InputCallbackInfo| {
                // just look at peak amplitudes (pos)
                let count = data.iter().filter(|&&x| x > 0.0).count();
                let mean = data.iter().filter(|&&x| x > 0.0).sum::<f32>() / count as f32;
                if mean > 0.005 {
                    println!("{:?}", mean)
                }
            },
            move |_err| {},
            )?;

            stream.play()?;
            loop{};
        }
    }

    Ok(())
}

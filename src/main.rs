use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host = cpal::default_host();

    let devices = host.devices().expect("Failed to get devices");

    for device in devices {
        println!(
            "Device: {:?}:",
            device.name().unwrap_or("Unknown".to_string())
        );
    }
}

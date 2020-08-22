use crate::devices::device::Device;
use std::error::Error;
use std::fs;

pub struct LedDevices {}

impl LedDevices {
    pub fn get_all_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        let mut led_devices = Vec::new();
            for entry in fs::read_dir("/sys/class/leds")? {
                let entry = entry?;
                let device = entry.file_name().to_str().unwrap().to_string();
                let brightness_file = format!("/sys/class/leds/{}/brightness", device);
                let brightness: i32 = fs::read_to_string(&brightness_file)?.trim().parse()?;
                let max_brightness_file = format!("/sys/class/leds/{}/max_brightness", device);
                let max_brightness: i32 = fs::read_to_string(&max_brightness_file)?.trim().parse()?;
                let led_device = Device::new(
                    &device,
                    brightness,
                    &brightness_file,
                    max_brightness,
                    &max_brightness_file,
                );
                led_devices.push(led_device);
        }
        Ok(led_devices)
    }
}

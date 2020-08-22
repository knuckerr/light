use crate::devices::device::Device;
use std::error::Error;
use std::fs;

pub struct BackLight {}

impl BackLight {
    pub fn get_all_devices(&self) -> Result<Device, Box<dyn Error>> {
        let mut led_device: Device = Default::default();
        for entry in fs::read_dir("/sys/class/backlight")? {
            let entry = entry?;
            let device = entry.file_name().to_str().unwrap().to_string();
            let brightness_file = format!("/sys/class/backlight/{}/brightness", device);
            let brightness: i32 = fs::read_to_string(&brightness_file)?.trim().parse()?;
            let max_brightness_file = format!("/sys/class/backlight/{}/max_brightness", device);
            let max_brightness: i32 = fs::read_to_string(&max_brightness_file)?.trim().parse()?;
            led_device = Device::new(
                &device,
                brightness,
                &brightness_file,
                max_brightness,
                &max_brightness_file,
            );
        }
        Ok(led_device)
    }
}

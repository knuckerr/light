use std::error::Error;
use std::fs;
use std::io::Write;
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Device {
    pub name: String,
    pub brightness: i32,
    pub brightness_file: String,
    pub max_brightness: i32,
    pub max_brightness_file: String,
}

impl Device {
    pub fn new(
        name: &str,
        brightness_n: i32,
        brightness_file: &str,
        max_brightness_n: i32,
        max_brightness_file: &str,
    ) -> Device {
        Device {
            name: name.to_string(),
            brightness: brightness_n,
            brightness_file: brightness_file.to_string(),
            max_brightness: max_brightness_n,
            max_brightness_file: max_brightness_file.to_string(),
        }
    }
    pub fn save_brightnes(&mut self, brightness: &str) -> Result<(), Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .open(&self.brightness_file)?;
        file.write_all(brightness.as_bytes())?;
        let new_brightnes: i32 = brightness.parse()?;
        self.brightness = new_brightnes;
        Ok(())

    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {} Brightness: {} Max Brightness: {}", self.name, self.brightness, self.max_brightness)
    }
}

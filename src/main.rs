use clap::{App, Arg};
use std::error::Error;
use std::process::exit;

use crate::devices::backlight::BackLight;
use crate::devices::device::Device;
use crate::devices::leds::LedDevices;

mod devices;

fn init() -> Result<Vec<Device>, Box<dyn Error>> {
    let led_devices = LedDevices {};
    let backlight = BackLight {};
    let mut all_devices = led_devices.get_all_devices()?;
    all_devices.push(backlight.get_all_devices()?);
    Ok(all_devices)
}

fn find_device_set_value(devices: Vec<Device>, device_name: &str, brightness: &str) {
    let mut device_exist: Vec<Device> = devices
        .into_iter()
        .filter(|x| x.name == device_name)
        .collect();
    if device_exist.is_empty() {
        println!("No device found with name {}", device_name);
        exit(1)
    }
    let device = &mut device_exist[0];
    if let Err(d) = device.save_brightnes(brightness) { eprintln!("{}", d) }
    println!("{}", device);
}

fn main() {
    let check_devices = init();
    if check_devices.is_err() {
        eprintln!("{}", check_devices.as_ref().unwrap_err());
    }
    let all_devices = check_devices.unwrap();
    let matches = App::new("My Super Program")
        .version("0.1")
        .author("Knucker")
        .about("Set brightness for led and screen devices")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .value_name("list")
                .help("List of all devices")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("device")
                .short("d")
                .long("device")
                .value_name("Device")
                .help("Select a device")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("brightness")
                .short("b")
                .long("brightness")
                .value_name("Brightness")
                .help("Set the value of Brightness")
                .takes_value(true),
        )
        .get_matches();

    if let 1 = matches.occurrences_of("list") {
        for x in &all_devices {
            println!("{}", x);
        }
        exit(1)
    }

    let select_device = matches.value_of("device").unwrap_or("");
    if select_device == "" {
        println!("No device was Selected....");
        exit(1)
    }

    let brightness = matches.value_of("brightness").unwrap_or("0").parse::<i32>();
    if brightness.is_err() {
        println!("Brightness value must be numeric");
        exit(1)
    }
    let brightness_value = brightness.unwrap();
    find_device_set_value(all_devices, select_device, &brightness_value.to_string());
}

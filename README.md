# light
This is a command-line application written in Rust that allows you to control the brightness of LED and screen devices. It provides a simple way to list available devices and set their brightness levels.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
  - [List Devices](#list-devices)
  - [Set Brightness](#set-brightness)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This application is designed to manage the brightness of LED and screen devices on your system. It utilizes the `clap` crate to provide a user-friendly command-line interface. The application allows you to list available devices and adjust their brightness levels.

## Installation

To use this application, make sure you have Rust installed on your system. If not, you can install it by following the instructions at [Rust Programming Language](https://www.rust-lang.org/tools/install).

1. Clone the repository:
   ```sh
   git clone https://github.com/your_username/your_repository.git
   ```

2. Navigate to the project directory:
   ```sh
   cd your_repository
   ```

3. Build the application:
   ```sh
   cargo build --release
   ```

## Usage

The application provides two main functionalities: listing available devices and setting their brightness levels.

### List Devices

To list all available devices and their information, use the following command:

```sh
./target/release/your_program_name -l
```

### Set Brightness

To set the brightness of a specific device, provide the device name and the desired brightness value:

```sh
./target/release/your_program_name -d <device_name> -b <brightness_value>
```

- `<device_name>`: The name of the device you want to adjust the brightness for.
- `<brightness_value>`: The desired brightness value to set for the device.

## Examples

List all available devices:
```sh
./target/release/your_program_name -l
```

Set brightness for a specific device:
```sh
./target/release/your_program_name -d led_device_1 -b 50
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or create a pull request in the [repository](https://github.com/your_username/your_repository).

When contributing, please make sure to follow the existing code style and provide clear commit messages.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

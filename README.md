# Time App

A simple fullscreen digital clock application built with Rust and eframe/egui.

## Dependencies

- `eframe` - Cross-platform GUI framework
- `chrono` - Date and time handling

## Installation

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

1. Clone or download this repository
2. Navigate to the project directory
3. Build and run the application:

```bash
cargo run
```

## Usage

Simply run the application and it will open in fullscreen mode displaying the current time. Press `ESC` or `Alt+F4` to exit the application.

## Build for Release

To build an optimized release version:

```bash
cargo build --release
```

The executable will be located in `target/release/time-app`.

## License

This project is open source and available under the MIT License.

# Click Counter

A cross-platform desktop application for tracking clicks per second (CPS). Built with Tauri, Rust, and modern web technologies.

## Features

- Real-time CPS (Clicks Per Second) tracking
- Interactive click area with visual feedback
- Ripple effect animations on click
- Maximum CPS tracking
- One-click counter reset
- Smooth, responsive UI
- Cross-platform support (Windows, macOS, Linux)

## Architecture

This project follows clean code principles with a well-organized, platform-specific architecture:

```
src/
├── main.rs           # Application entry point
├── lib.rs            # Library setup and module exports
├── app.rs            # Application logic
├── core/             # Platform-agnostic business logic
│   └── mod.rs       # Core state management
└── platform/         # Platform abstraction layer
    ├── mod.rs       # Platform trait and auto-detection
    ├── windows/     # Windows-specific implementations
    ├── macos/       # macOS-specific implementations
    └── linux/       # Linux-specific implementations
```

### Design Principles

- **Clean Separation**: Each platform has its own isolated directory
- **Trait-Based Abstraction**: Common `Platform` trait for all platforms
- **Compile-Time Optimization**: Only target platform code is compiled
- **Type Safety**: Rust's type system ensures consistency
- **Extensibility**: Easy to add new platform-specific features

## Tech Stack

- **Backend**: Rust + Tauri 2.0
- **Frontend**: HTML5, CSS3, Vanilla JavaScript
- **Build System**: Cargo
- **Platform Support**: Windows, macOS, Linux

## Prerequisites

- [Rust](https://www.rust-lang.org/) (1.70 or later)
- [Node.js](https://nodejs.org/) (optional, for frontend development)
- Platform-specific dependencies for Tauri (see [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

## Building

### Development Build

```bash
cargo build
```

### Production Build

```bash
cargo build --release
```

### Run the Application

```bash
cargo tauri dev
```

## Testing

Run the test suite:

```bash
cargo test
```

All tests include:
- Platform detection tests
- Platform initialization tests
- Core functionality tests
- Application integration tests

## Cross-Platform Builds

### Windows
```bash
cargo build --target x86_64-pc-windows-msvc
```

### macOS
```bash
cargo build --target x86_64-apple-darwin
```

### Linux
```bash
cargo build --target x86_64-unknown-linux-gnu
```

## Project Structure

- `src/` - Rust backend code with platform-specific modules
- `ui/` - Frontend HTML, CSS, and JavaScript
- `icons/` - Application icons
- `tauri.conf.json` - Tauri configuration
- `Cargo.toml` - Rust dependencies

## Usage

1. Launch the application
2. Click on the colored area to register clicks
3. Watch your real-time CPS displayed in the center
4. Track your maximum CPS in the stats section
5. Click "Reset Counter" to start over

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the MIT License.

## Acknowledgments

This project was developed with the assistance of AI technology.

Built with [Tauri](https://tauri.app/) - Build smaller, faster, and more secure desktop applications with a web frontend.

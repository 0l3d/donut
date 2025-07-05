# Donut 🍩

A classic spinning donut animation implemented in Rust using no_std, inspired by Andy Sloane's famous donut.c.

## Features

- Pure Rust implementation with no_std
- ASCII art 3D donut animation
- Smooth rotation in terminal
- Uses custom C library bindings (crlib)

## Requirements

- Rust compiler
- Unix-like system (Linux, macOS, etc.)
- Terminal that supports ANSI escape sequences

## Installation

1. Clone this repository:
```bash
git clone https://github.com/0l3d/donut.git
cd donut
```

2. Make sure you have the required files:
   - `donut.rs` - Main donut code
   - `crlib.rs` - C library bindings (must be in same directory)

## Building

```bash
make
```

## Running

```bash
./donut
```

Press `Ctrl+C` to exit.

## Project Structure

```
donut/
├── donut.rs         # Main donut implementation
├── crlib.rs         # C library bindings
├── Makefile         # Build configuration
└── README.md        # This file
```

## How it Works

This implementation uses:
- **3D Mathematics**: Parametric equations for torus generation
- **Projection**: 3D to 2D coordinate transformation
- **Z-buffering**: Depth testing for proper rendering
- **ASCII Shading**: Different characters represent lighting intensity

The donut rotates around two axes, creating a mesmerizing 3D effect in your terminal.

## Technical Details

- **Language**: Rust (no_std)
- **Dependencies**: Custom crlib for C function bindings
- **Screen Size**: 80x24 characters
- **Frame Rate**: ~25 FPS
- **Shading Characters**: `.,-~:;=!*#$@`

## Credits

- Original concept by Andy Sloane
- Rust implementation by [@0l3d](https://github.com/0l3d)
- Based on the mathematical principles of torus rendering

## License

This project is open source. Feel free to use and modify.

---

*Enjoy the spinning donut! 🍩*

# Part 3

Here we add some external crates and use them to display our game in a window. 

The crates we'll use are softbuffer and winit. 

The code to create the window may seem like it's quite specific to winit but it's actually very similar to working with SDL2.

This renders entirely on the CPU and you'll find that if you scale the window up to fullscreen, even on modern hardware, the simulation will slow down significantly.

## Getting Started

To run in development mode
```
cargo run
```
To build a release version
```
cargo build --release
```
see `targets/release` for output.

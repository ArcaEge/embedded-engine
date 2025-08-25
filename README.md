# Embedded Engine

A Rust-based basic pixel game engine that can compile to both WASM and to bare metal on the RP2040, using a HTML canvas or an SSD1306 for display output. The goal is to support basic 3D graphics in the future.

[Demo and video](https://arcaege.github.io/embedded-engine/)

Features (working):

- Compiling to both WASM and RP2040
- Framebuffer
- Inputs

Features (not yet implemented):

- Sprites
  - Collisions
- 3D??
- Async on the RP2040 side
- Loading games from SD card??

## Pinout

| Pi Pico pin | Connected to |
| ------------|--------------|
| GPIO 4      | SSD1306: SDA |
| GPIO 5      | SSD1306: SCL |
| 3.3V        | SSD1306      |
| GND         | SSD1306      |

## Build/run

### Pico

```bash
cargo rp2040
```

### WASM

```bash
cd ./www/
npm install
npm run start
```

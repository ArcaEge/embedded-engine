# Embedded Engine

A Rust-based basic pixel game engine that can compile to both WASM and to bare metal on the RP2040, using a HTML canvas or an SSD1306 for display output. The goal is to support basic 3D graphics in the future.

[Demo and video](https://arcaege.github.io/embedded-engine/)

Features (working):

- Compiling to both WASM and RP2040
- Framebuffer
- Inputs
- Sprites
- Actor-World system
- Sound
  - Sound player
- Spritesheet support (see [ArcaEge/embedded-encoder](https://github.com/ArcaEge/embedded-encoder), this is the accompanying spritesheet encoder)

Features (not yet implemented):

- Sound from encoded file
- Collisions
- Demo game
- 3D??
- Async on the RP2040 side?
- Loading games from SD card??
- Doom port (probably only for WASM because RAM limitations, but pico could work with a little optimising perhaps) - game engine in a game engine???

? = Maybe, I'll do it if I find time

## Pinout

| Pi Pico pin | Connected to |
| ------------|--------------|
| GPIO 4      | SSD1306: SDA |
| GPIO 5      | SSD1306: SCL |
| 3.3V        | SSD1306      |
| GND         | SSD1306      |
| GPIO 6      | Up           |
| GPIO 7      | Down         |
| GPIO 8      | Left         |
| GPIO 9      | Right        |
| GPIO 10     | Jump         |
| GPIO 18     | Sprint       |
| GPIO 19     | Crouch       |
| GPIO 20     | Piezo buzzer |

Connect the other leg of the buttons to ground, no need for pullup resistors as we're using the pico's internal pullups.

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

## Assets to use

- <https://emerald-fish.itch.io/1bit-slime-platformer>
- <https://pixelhole.itch.io/8x8dungeontilemap>
- 16x16, might work less well: <https://kenney-assets.itch.io/1-bit-platformer-pack>
- <https://thebuffed.itch.io/tiny-8x8-dungeon>

## Demo spritesheet credits

- Thanks to Emerald Fish on itch.io for this great tileset: <https://emerald-fish.itch.io/1bit-slime-platformer>

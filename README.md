# Embedded engine

A Rust-based basic pixel game engine that can compile to both WASM and to bare metal on the RP2040, using a HTML canvas or an SSD1306 for display output. The goal is to support basic 3D graphics in the future.

## Build/run

### WASM

```bash
cargo run --target wasm32-unknown-unknown
```

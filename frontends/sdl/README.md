# CHIP-Ahoyto SDL

Utilizes the [`chip-ahoyto`](../../) crate and the [SDL crate](https://github.com/Rust-SDL2/rust-sdl2) to build a desktop application.

## Running

```bash
cargo install cargo-vcpkg
cargo vcpkg build
cargo build --release
cargo run --release
```

Drag and drop your ROM to play.

## Controls

**Keys available:**

`1 2 3 4`
`Q W E R`
`A S D F`
`Z X C V`

**Controls available:**

* `+` Increments the logical (CPU) frequency
* `-` Decrements the logical (CPU) frequency
* `T` Toggles the display of the diagnostics information
* `O` Resets the machine
* `P` Changes the pixel color of the system
* `M` Takes a RAM and VRAM snapshot and saves it in `.sv8`

## ROMs

You can find some good quality game and tools roms here:

* [https://github.com/kripod/chip8-roms](https://github.com/kripod/chip8-roms)
* [https://github.com/loktar00/chip8](https://github.com/loktar00/chip8)

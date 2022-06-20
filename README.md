# CHIP-Ahoyto 🍪

CHIP-8 emulator written in rust 🦀.

The goal of this project is purely experimental and a learning tool for rust.

The work of this emulator was inspired/started by [jc-chip8](https://github.com/joao-conde/jc-chip8).

**You can check a working version of the emulator at [chip-aoyto.joao.me](https://chip-aoyto.joao.me)**

## Goals

* Performance 🏎
* Separation of concerns 🖖
* Simplicity 😀
* Compatibility 🪛

## Features

* Drag and drop support for ROMs
* Pallet switching
* Visual diagnostics
* Variable CPU frequency
* Multiple engine implementations (classic and neo)
* Full compliant with test CHIP-8 ROMs
* RAM snapshot saving and loading
* [WebAssembly](https://webassembly.org) support 🌐

## Build

### WASM for Node.js

```bash
cargo install wasm-pack
wasm-pack build --release --target=nodejs -- --features wasm
```

### WASM for Web

```bash
cargo install wasm-pack
wasm-pack build --release --target=web --out-dir=examples/web/lib -- --features wasm
cd examples/web
npm install && npm run build
python3 -m http.server
```

## Reason

And... yes this is the real inspiration behind the emulator's name:

<img src="res/chips-ahoy.jpeg" alt="Chips Ahoy" width="200" />

## Inspiration

Many articles and websites helped me in this quest to build the emulator these are some of them:

* [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator)
* [Writing a CHIP-8 emulator with Rust and WebAssembly](https://blog.scottlogic.com/2017/12/13/chip8-emulator-webassembly-rust.html)
* [Wikipedia - CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
* [itch.io - OctoJam 6](https://itch.io/jam/octojam-6)
* [CHIP-8 Archive](https://johnearnest.github.io/chip8Archive)

## License

CHIP-Ahoyto is currently licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/).

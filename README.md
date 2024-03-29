# [CHIP-Ahoyto](https://chip-ahoyto.joao.me) 🍪

CHIP-8 emulator written in Rust 🦀.

The goal of this project is purely experimental and a learning tool for rust.

The work of this emulator was inspired/started by [jc-chip8](https://github.com/joao-conde/jc-chip8).

You can check a working version of the emulator at **[chip-ahoyto.joao.me](https://chip-ahoyto.joao.me)**

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

## Deployments

| Provider   | Stable  | URL                                                                  |
| ---------- | ------- | -------------------------------------------------------------------- |
| Cloudflare | `True`  | [chip-ahoyto.joao.me](https://chip-ahoyto.joao.me)                   |
| Netlify    | `True`  | [chip-ahoyto.netlify.app](https://chip-ahoyto.netlify.app)           |
| Cloudflare | `True`  | [chip-ahoyto.joao.me](https://chip-ahoyto.joao.me)                   |
| Cloudflare | `True`  | [chip-ahoyto.pages.dev](https://chip-ahoyto.pages.dev)               |
| Cloudflare | `True`  | [prod.chip-ahoyto.pages.dev](https://prod.chip-ahoyto.pages.dev)     |
| Cloudflare | `True`  | [stable.chip-ahoyto.pages.dev](https://stable.chip-ahoyto.pages.dev) |
| Cloudflare | `False` | [master.chip-ahoyto.pages.dev](https://master.chip-ahoyto.pages.dev) |

## Crate Features

| Name     | Description                                                              |
| -------- | ------------------------------------------------------------------------ |
| `quirks` | Allows CHIP-8 quirks runtime evaluation, comes at a performance penalty. |

## Build

### Library

```bash
cargo build
```

### WASM for Node.js

```bash
cargo install wasm-pack
wasm-pack build --release --target=nodejs -- --features wasm
```

### WASM for Web

```bash
cargo install wasm-pack
wasm-pack build --release --target=web --out-dir=frontends/web/lib -- --features wasm
cd frontends/web
npm install && npm run build
cd dist && python3 -m http.server
```

## Reason

And... yes this is the real inspiration behind the emulator's name:

<img src="https://gitlab.stage.hive.pt/joamag/chip-ahoyto/raw/master/res/chips-ahoy.jpeg" alt="Chips Ahoy" width="200" />

## Inspiration

Many articles and websites helped me in this quest to build the emulator these are some of them.

### Documentation

* [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator)
* [Writing a CHIP-8 emulator with Rust and WebAssembly](https://blog.scottlogic.com/2017/12/13/chip8-emulator-webassembly-rust.html)
* [Wikipedia - CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
* [itch.io - OctoJam 6](https://itch.io/jam/octojam-6)
* [CHIP-8 Archive](https://johnearnest.github.io/chip8Archive)

### Articles

* [Revisiting Chip-8](https://faizilham.github.io/revisiting-chip8)

## License

CHIP-Ahoyto is currently licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/).

## Build Automation

[![Build Status](https://github.com/joamag/chip-ahoyto/workflows/Main%20Workflow/badge.svg)](https://github.com/joamag/chip-ahoyto/actions)
[![crates Status](https://img.shields.io/crates/v/chip-ahoyto)](https://crates.io/crates/chip-ahoyto)
[![npm Status](https://img.shields.io/npm/v/chip-ahoyto.svg)](https://www.npmjs.com/package/chip-ahoyto)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/)

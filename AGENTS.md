# AGENTS.md file

This document describes how to work with the project. Follow these notes when writing code or submitting pull requests.

## Highlights

- CHIP-8 emulator written in Rust with a focus on performance, simplicity and separation of concerns
- Release builds use LTO, `opt-level = 3`, and disabled debug symbols
- Three frontends: Web (WASM), SDL (desktop with diagnostics overlay), and Benchmark
- Web frontend built with React/TypeScript and powered by [EmuKit](https://github.com/joamag/emukit) for emulator UI infrastructure
- Core compiles to WebAssembly via `wasm-pack`, sharing the same emulation logic across all targets
- Two interchangeable engine implementations behind the `Chip8` trait: `classic` (straightforward fetch and decode) and `neo` (faster, quirk aware and save state capable)
- Fully compliant with the standard CHIP-8 test ROMs, which are bundled under `res/roms`
- Optional `quirks` feature enables runtime evaluation of the six CHIP-8 quirks, at a performance cost, otherwise the canonical behaviour is compiled in branch free through the macros in `src/macros.rs`
- Supports RAM and VRAM snapshot save states, palette switching, variable CPU frequency, and visual diagnostics
- Published to crates.io and npm; deployed to Cloudflare Pages and Netlify
- Uses `cargo` for Rust and `npm` for the Web frontend; licensed under Apache 2.0

## Setup

Install the Rust toolchain and the build helpers:

```bash
rustup default nightly
rustup component add rustfmt
rustup component add clippy
cargo install wasm-pack
cargo install cargo-vcpkg
```

The nightly toolchain is required because `rustfmt.toml` uses the nightly only `imports_granularity` and `group_imports` options.

## Formatting

Format all code before committing:

```bash
cargo fmt --all
cargo clippy --fix --allow-dirty --allow-staged --all-targets
npm --prefix frontends/web run pretty
```

## Testing

Run the full test suite:

```bash
cargo test --all-targets --features quirks
```

To run the tests across every workspace member (including the SDL frontend) use `cargo test --all`, which requires the SDL dependencies described below to be already built.

## Web Frontend

To build the Web frontend, first compile the core to WebAssembly and then bundle the app:

```bash
wasm-pack build --release --target=web --out-dir=frontends/web/lib -- --features wasm
cd frontends/web && npm install && npm run build
```

The `frontends/web/lib` directory is a generated artifact and is not tracked, so it must be rebuilt whenever the Rust API surface changes.

## SDL Frontend

To build the SDL frontend (if required) use:

```bash
cd frontends/sdl && cargo vcpkg build && cargo build
```

## Style Guide

- Always update `CHANGELOG.md` according to semantic versioning, mentioning your changes in the unreleased section.
- Write commit messages using [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
- Never bump the internal package version in `Cargo.toml` or `package.json`. This is handled automatically by the release process.
- Rust files use LF line endings, while TypeScript files use CRLF.
- Inline comments should be in the format `// <comment>` and start with lowercase.

## New Release

To create a new release follow the following steps:

- Make sure that both the tests pass and the code formatting are valid.
- Increment (look at `CHANGELOG.md` for semver changes) the `version` value in all the `Cargo.toml` files across the repo (the root package, the root `[workspace] package` table, `frontends/benchmark` and `frontends/sdl`), the `version` value in `frontends/web/package.json`, and the hardcoded version returned by `Info::version()` in `src/info.rs`.
- Move all the `CHANGELOG.md` Unreleased items that have at least one non empty item the into a new section with the new version number and date, and then create new empty sub-sections (Added, Changed and Fixed) for the Unreleased section with a single empty item.
- Create a commit with the following message `version: $VERSION_NUMBER`.
- Push the commit.
- Create a new tag with the value fo the new version number `$VERSION_NUMBER`.
- Create a new release on the GitHub repo using the Markdown from the corresponding version entry in `CHANGELOG.md` as the description of the release and the version number as the title. Do not include the title of the release (version and date) in the description.
- Promote the release to the stable channel by merging `master` into the `stable` branch and pushing it (this triggers the stable and production deployments).
- Sync the tag and the affected branches (`master` and `stable`) to both the GitLab and GitHub remotes, since the deployment and publishing pipelines run across both.

## License

CHIP-Ahoyto is licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/).

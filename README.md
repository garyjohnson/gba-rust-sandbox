# gba-rust-sandbox

Sandbox to play with GBA development and rust

Templated from https://github.com/agbrs/agb

Current docs assume macOS.

## Requirements

Install rustup (https://rustup.rs)

```sh
rustup install nightly
rustup +nightly component add rust-src

brew install --cask gcc-arm-embedded

cargo install cargo-make
cargo install gbafix
```

## Build
Run `cargo build --release`

### Build && Package && Run
Run `./build`

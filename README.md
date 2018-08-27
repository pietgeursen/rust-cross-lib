# Rust lib for embedded projects 

> Cross compile a library in Rust that can be linked into a C project (that's also cross compiled). 

## Dev setup

- Rustup to get rust
- Follow the instructions for [cross](https://github.com/japaric/cross)
  - You need a very recent version on the rust nightly. Otherwise you might get an error about panic_fmt or panic implementation.

## Build

For nrf52:

`$ cross build --target thumbv7em-none-eabihf --release`

## Test

`$ cargo test`

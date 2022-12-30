
# Welcome to the future

This is a native application running in your browser.
Powered by Rust and WebAssembly.

### Nightly

This project requires nightly features.

Install nightly toolchain using:
`cargo toolchain install nightly`
and enable nightly features within the project with
`rustup override set nightly`.

Enjoy the nightly features!

### Inspiration

After watching these 2 videos:

* [Web-Native Rust](https://www.youtube.com/watch?v=y10jJX35shE&t=441s)
* [Ascii Image Converter](https://www.youtube.com/watch?v=uSYd8HjY6GE)

I decided to combine them and created this app.

### Run locally (native native)

`cargo run`

### Run locally (browser native)

`cargo install --locked trunk && trunk serve`

### Run from static site hosting

[from Github](github.io/maixnor/ascii)

### Deployment Status

[![CI](https://github.com/maixnor/ascii/actions/workflows/rust.yml/badge.svg)](https://github.com/maixnor/ascii/actions/workflows/rust.yml)
[![Github Pages](https://github.com/maixnor/ascii/actions/workflows/pages.yml/badge.svg)](https://github.com/maixnor/ascii/actions/workflows/pages.yml)

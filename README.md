# Mythmellow

A top-down arena shooter roguelite in which you're a mythical marshmallow god fighting against peasant munchies such as chocolates, jellies, or candies!

## Goal

Mythmellow is created after playing excessive amount of [Brotato](https://store.steampowered.com/app/1942280/Brotato/) as a Rust developer. The purpose is to learn how to develop a game from start to end with [Bevy](https://bevyengine.org/) using the best practices, and share it with the community as an example.

## Installation

Mythmellow is free and open source! It'll be released on Steam and Epic Games Store in the future for those who want to donate or want to have the convenience of having the game on a gaming platform.

At the time being, the only way to play is to build the game yourself. The game is still in its infancy, so the release process is non-existent.

### Steam

Coming soon...

### Epic Games

Coming soon...

### Pre-built

Coming soon...

### Browser

Coming soon...

### Crates.io

Coming soon...

## Building

You need [Nightly Rust Toolchain](https://www.rust-lang.org/tools/install), [Bevy OS Dependencies](https://bevyengine.org/learn/book/getting-started/setup/#install-os-dependencies) and [LLD or Mold Linker](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional) to build the game.

### Native (Debug)

To build the game for development in your native platform, you can run:

```shell
cargo build --features native,development,bevy/dynamic_linking
```

### Native (Release)

To build the game for release in your native platform, you can run:

```shell
cargo build --features native --release
```

### WebAssembly (Debug)

To build the game for development in WebAssembly, you can run:

```shell
cargo build --target wasm32-unknown-unknown --features wasm,development
```

### WebAssembly (Release)

To build the game for release in WebAssembly, you can run:

```shell
cargo build --target wasm32-unknown-unknown --features wasm --release
```

## Usage

### CLI Options

#### \-\-configuration \<PATH>

Sets the configuration directory.

- In native:
  - defaults to current platforms config directory (see [dirs::config_dir](https://docs.rs/dirs/latest/dirs/fn.config_dir.html))
- In wasm:
  - defaults to `session/configuration` which means browsers session storage will be used

#### \-\-data \<PATH>

Sets the data directory.

- In native:
  - defaults to current platforms data directory (see [dirs::data_dir](https://docs.rs/dirs/latest/dirs/fn.data_dir.html))
- In wasm:
  - defaults to `session/configuration` which means browsers session storage will be used

#### \-\-game

Starts the application in the game, skipping menus.

## Documentation

### API Documentation

To view the API documentation, you can run:

```shell
cargo doc --features native,development --open
```

### Design Documentation

To view the design documentation, you can run:

```shell
mdbook serve --open
```

(requires [mdbook](https://rust-lang.github.io/mdBook/guide/installation.html)).

## License

[Mythmellow](https://github.com/umut-sahin/mythmellow/) is free, open source and permissively licensed!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](https://github.com/umut-sahin/mythmellow/blob/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE]((https://github.com/umut-sahin/mythmellow/blob/main/LICENSE-APACHE)) or <https://www.apache.org/licenses/LICENSE-2.0>)

This means you can select the license you prefer!

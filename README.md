# Mythmallow

A top-down arena shooter roguelite in which you're a mythical marshmallow fighting against peasant munchies such as chocolates, jellies, or candies!

## Goal

Mythmallow is created after playing excessive amount of [Brotato](https://store.steampowered.com/app/1942280/Brotato/) as a Rust enthusiast. The purpose is to learn how to develop a game from start to end with [Bevy](https://bevyengine.org/) using best practices, and share it with the community as an example.

## Installation

Mythmallow is free and open source! It'll be released on Steam and Epic Games Store for a small price in the future for those who want to donate or want to have the convenience of having the game on a gaming platform.

For the time being, the only way to play is to build the game yourself. The game is still in its infancy, so the release process is non-existent.

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
cargo build
```

### Native (Release)

To build the game for release in your native platform, you can run:

```shell
cargo build --release --no-default-features --features native-release
```

### WebAssembly (Debug)

To build the game for development in WebAssembly, you can run:

```shell
cargo build --target wasm32-unknown-unknown --no-default-features --features wasm-development
```

### WebAssembly (Release)

To build the game for release in WebAssembly, you can run:

```shell
cargo build --release --target wasm32-unknown-unknown --no-default-features --features wasm-release
```

## Usage

### Arguments

- In Native:
  - Arguments are parsed from the command line arguments.
    ```
    mythmallow --seed 42 --game
    ```
- In WebAssembly:
  - Arguments are parsed from the URL.
    ```
    https://mythmallow.io/?seed=42&game
    ```

#### \-\-configuration \<PATH>

Specifies the configuration directory.

- In Native:
  - defaults to current platforms configuration directory (see [dirs::config_dir](https://docs.rs/dirs/latest/dirs/fn.config_dir.html))
- In WebAssembly:
  - defaults to `session/configuration` which means browsers session storage will be used

#### \-\-data \<PATH>

Specifies the data directory.

- In Native:
  - defaults to current platforms data directory (see [dirs::data_dir](https://docs.rs/dirs/latest/dirs/fn.data_dir.html))
- In WebAssembly:
  - defaults to `session/data` which means browsers session storage will be used

#### \-\-seed \<SEED>

Specifies the seed for random number generation in the game.

If not set manually, a random seed will be used.

**Warning:** This argument is only for debugging purposes. Same seed can result in a different game, in different versions of the game.

#### \-\-game

Starts the application directly in-game, bypassing menus.

#### \-\-mode \<MODE>

Specifies the game mode when starting the application in-game.

Available Game Modes:
- `survival`

If not set manually, a random game mode will be selected.

#### \-\-player \<PLAYER>

Specifies the player when starting the application in-game.

Available Players:
- From Greek Mythology:
  - `artemis`
  - `hades`

If not set manually, a random player will be selected.

#### \-\-enemies \<ENEMIES>

Specifies the enemies when starting the application in-game.

Available Enemies:
- `sweet`

If not set manually, random enemies will be selected.

## Documentation

### API Documentation

To view the API documentation, you can run:

```shell
cargo doc --open
```

### Design Documentation

To view the design documentation, you can run:

```shell
mdbook serve --open
```

(requires [mdbook](https://rust-lang.github.io/mdBook/guide/installation.html)).

## License

[Mythmallow](https://github.com/umut-sahin/mythmallow/) is free, open source and permissively licensed!

All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](https://github.com/umut-sahin/mythmallow/blob/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE]((https://github.com/umut-sahin/mythmallow/blob/main/LICENSE-APACHE)) or <https://www.apache.org/licenses/LICENSE-2.0>)

This means you can select the license you prefer!

# kill-them-all

**Kill Them All** is a [POE](https://www.pathofexile.com/) lite game.

- [Affixes](#affixes)
- [How to build](#build)

# How to play

Use mouse to move player. Take items dropped by monsters you killed. 
Use [I] to open the inventory.
Try to use the best items for the player.

## Player affixes

TODO

## How to build

You need [Rust](https://www.rust-lang.org/) to build this game.

### run 
```shell
cargo run --release
```

### run with debug features
```shell
cargo run --features=dev
```

### run with tracy features

see https://github.com/bevyengine/bevy/blob/main/docs/profiling.md

In a terminal run
```shell
c:\apps\tracy\tracy-capture.exe -o my_capture.tracy
```
In an other terminal run
```shell
cargo run --release --features bevy/trace_tracy
```

# WEB

see [Bevy + WebGPU](https://bevyengine.org/news/bevy-webgpu/)

## Tools

```shell
rustup target install wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install wasm-opt --locked
cargo install wasm-server-runner
cargo install simple-http-server
```

## Run dev

```shell
cargo run --release --target wasm32-unknown-unknown
```

## Produce a web site

```shell
cargo build --profile wasm-release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./website/ --out-name "kill-them-all" ./target/wasm32-unknown-unknown/release/kill-them-all.wasm
wasm-opt -Oz --output optimized.wasm ./website/kill-them-all_bg.wasm
```
Move the `optimized.wasm` file to `./website/kill-them-all_bg.wasm`, overiding the existing file.

Copy the `assets` folder in the `website` folder.

### run localy

```shell
simple-http-server ./website
```

and open a browser with the url : `http://localhost:8000/index.html`

## Produce a web site

# Assets

- TODO
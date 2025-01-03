# kill-them-all

**Kill Them All** is a [POE](https://www.pathofexile.com/) lite game.

- [Affixes](#affixes)
- [How to build](#build)

## Player affixes


## How to build {#build}

You need [Rust](https://www.rust-lang.org/) to build this game.

### run 
```shell
cargo run --release
```

### run with debug features
```shell
cargo run --features=dev
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

# Assets

- 
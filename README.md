# kill-them-all

# Native

## run 
```shell
cargo run --release
```

## run with debug features
```shell
cargo run --features=dev
```

# WEB

## Tools

```shell
rustup target install wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install wasm-server-runner
cargo install simple-http-server
```

## Run dev

```shell
cargo run --release --target wasm32-unknown-unknown
```

## Produce a web site

```shell
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "kill-them-all" ./target/wasm32-unknown-unknown/release/kill-them-all.wasm
```
Copy the `assets` folder in the `out` folder.

run localy

```shell
simple-http-server ./out
```

and open a browser with the url : `http://localhost:8000/index.html`
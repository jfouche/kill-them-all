# kill-them-all

# Native

## run 
```shell
cargo run --release
```

## run with debug features
```shell
cargo run --features=debug
```

# WEB

## Tools

```shell
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
```

## Run

```shell
cargo run --target wasm32-unknown-unknown
```
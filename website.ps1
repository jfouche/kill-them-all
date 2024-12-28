# Compile WASM
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./website/ --out-name "kill-them-all" ./target/wasm32-unknown-unknown/release/kill-them-all.wasm

# Copy the assets
Copy-Item -Path "assets" -Destination "website" -Recurse -Force
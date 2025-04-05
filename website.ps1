$env:RUSTFLAGS='--cfg getrandom_backend="wasm_js"'

# Compile WASM
cargo build --profile wasm-release --target wasm32-unknown-unknown

#RESET env var
$env:RUSTFLAGS=''

# Bindgen to create javascript file
wasm-bindgen --no-typescript --target web --out-dir ./website/ --out-name "kill-them-all" ./target/wasm32-unknown-unknown/wasm-release/kill-them-all.wasm

# Optimize the wasm file
wasm-opt -Oz --output "optimized.wasm" ./website/kill-them-all_bg.wasm

# Copy the optimized wasm file to its original path
Move-Item -Path "optimized.wasm" -Destination "website/kill-them-all_bg.wasm" -Force

# Copy the assets
Copy-Item -Path "assets" -Destination "website" -Recurse -Force
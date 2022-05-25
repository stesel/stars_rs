[![Rust](https://github.com/stesel/stars_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/stesel/stars_rs/actions/workflows/rust.yml)
[![Deploy](https://github.com/stesel/stars_rs/actions/workflows/deploy.yml/badge.svg)](https://github.com/stesel/stars_rs/actions/workflows/deploy.yml)
# stars_rs
Star shooter with bevy/wgpu


## Run on WEB

Add web assembly target:
```
rustup target add wasm32-unknown-unknown
```

Add wasm-bindgen-cli:
```
cargo install -f wasm-bindgen-cli
```

Build project:
```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/stars_rs.wasm
```

cargo build --target wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/debug/biubiu.wasm --out-dir .
npm run serve

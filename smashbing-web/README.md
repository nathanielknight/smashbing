This crate implements a program for playing SmashBing in the browser using
WebAssembly.

Building it is more complicated than `smashbing-native`. It requires
[`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen), so you'll have
to install that first.

    cargo install wasm-bindgen-cli

You'll also have to add the `wasm32-unknown-unknown` target to cargo.

    rustup target add wasm32-unknown-unknown --toolchain nightly

The build has a few steps:

```bash
# build the webassembly code
cargo build --target wasm32-unknown-unknown

# Bind the built wasm code to a javascript module
wasm-bindgen target/wasm32-unknown-unknown/debug/smashbing_web.wasm --out-dir . --no-modules
```

You may also need to copy the HTML, JavaScript, and audio source files into
a common directory.

If `wasm-bindgen` complains that it's out of sync with the rust version, it
can be updated with

    cargo install --force wasm-bindgen-cli

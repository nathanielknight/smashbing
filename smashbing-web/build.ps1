# Clean up old builds
get-childitem release | remove-item

# Build the WASM files
cargo +nightly build --release --target wasm32-unknown-unknown
wasm-bindgen --no-modules target/wasm32-unknown-unknown/release/smashbing_web.wasm --out-dir ./release
# Remove unnecessary things
Remove-Item -Path "release/smashbing_web.d.ts"

# Deploy index
Copy-Item -Path "index.html" -Destination "./release/"

# Skip sound for now
# foreach ($sound in (gci ../sounds)) {
#     Copy-Item -Path "../sounds/$sound" -Destination "./sounds/"
# }
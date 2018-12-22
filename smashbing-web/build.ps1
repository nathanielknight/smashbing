# Clean up old builds
get-childitem release | remove-item -Recurse

# Build the WASM files
cargo +nightly build --release --target wasm32-unknown-unknown
wasm-bindgen --no-modules target/wasm32-unknown-unknown/release/smashbing_web.wasm --out-dir ./release
# Remove unnecessary things
Remove-Item -Path "release/smashbing_web.d.ts"

# Deploy index
Copy-Item -Path "index.html" -Destination "./release/"
Copy-Item -Path "audioplayer.js" -Destination "./release/"
New-Item -ItemType Directory -Path "./release/sounds"
foreach ($sound in (Get-ChildItem ../sounds)) {
    Copy-Item -Path "../sounds/$sound" -Destination "./release/sounds/"
}
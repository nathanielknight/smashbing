This crate implements a downloadable program for playing SmashBing.

This crate uses [`ggez`](http://ggez.rs/) for graphics and sound; `ggez`, in
turn, uses [SDL](https://www.libsdl.org/), which can be tricky to install. The
best instructions for installing it are in the
[Rust SDL2 crate](https://github.com/Rust-SDL2/rust-sdl2#user-content-requirements).

Once you've installed SDL, you can build the game with

    cargo build

or

    cargo build --release

Tests can be run with

    cargo test

In theory, it can be built for Windows, Linux, or MacOS (though it's only
been tested on Windows and Ubuntu). See
[GGEZ's documentation](https://github.com/ggez/ggez/blob/master/docs/BuildingForEveryPlatform.md)
for details.

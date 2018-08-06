# Smashbing

The pixel critters have been separated form their pixel parent.
Smash pixels to re-unite them!

This is my code for [LOWREZJAM 2018](https://itch.io/jam/lowrezjam-2018).
I've done a [Pong clone](https://nknight.itch.io/wondrousbingball) in Rust,
so I figure it's time to do a breakout clone.

I also want to cross-build for the web
to make distribution easier
and to be able to say I've done something with WASM.
To that end,
I've broken out the game logic into a library
and put the [ggez](https://github.com/icefoxen/ggez) client in a separate crate.
They're tied together with a [Cargo workspace][cargo-wsp].
If I have time, I'll build a WASM version
of the game library
and build a Typescript client as well.

The game library is responsible for:

- defining game state and moves (including animation states)
- updating the game state in response to moves

The game client is responsible for:

- drawing the game state
- the game loop clock (because I can't rely on the hardware clock in the WASM library)
- handling effects (e.g. user input✓, sounds)

[cargo-wsp]: https://doc.rust-lang.org/book/second-edition/ch14-03-cargo-workspaces.html

I want to have sub-pixel physics,✓
with gravity and collisions.✓
I want destroying blocks✓
to randomly perturb the ball's path
to emulate chaotic collisions.

I want chunky low-rez sounds.

I want to do touch/mouse only controls✓
to make mobile-web clients easier.

I'll record my build process with [Just](https://github.com/casey/just).

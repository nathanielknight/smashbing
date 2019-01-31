# Ballistic SmashBing

This is a game written in Rust that can be built as a native executable or as
a web page using WebAssembly (and some supporting JavaScript).

The game is divided into two parts: the game library is responsible for defining
game states (including physics and animation) and updating it in response to
player input. It also emits output events (like playing sounds or quitting the
game). THe game client runs the main game loop, draws the game state, collects
player input, and handles output events from the game library. The game library
is in `libsmashbing`. There are two game clients (one native and one web) in
`smashbing-native` and `smashbing-web`.

See the individual crates for build and test instructions.

This is my code for [LOWREZJAM 2018](https://itch.io/jam/lowrezjam-2018).
I've done a [Pong clone](https://nknight.itch.io/wondrousbingball) in Rust,
so I figure it's time to do a breakout clone.

This crate holds the game code (physics, collisions, etc.). It's used by
`smashbing-native` and `smashbing-web`.

Tests can be run with

    cargo test

Because it's referenced directly by the crates that use it, there's no
special build step for this crate.

Documentation (for reference while implementing programs that use this crate)
can be built with

    cargo doc

or

    cargo doc --document-private-items

to get documentation on internal items as well.

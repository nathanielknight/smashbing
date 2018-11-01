extern crate libsmashbing;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn embed() {
    let mut game = libsmashbing::Game::default();
}

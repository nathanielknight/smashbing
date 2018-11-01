extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate libsmashbing;

#[wasm_bindgen]
pub struct Game {
    game: libsmashbing::Game,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            game: libsmashbing::Game::default(),
        }
    }
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen]
    pub fn get_ball_pos(&self) -> String {
        let ball = &self.game.ball;
        let x: f32 = ball.pos.x;
        let y: f32 = ball.pos.y;
        format!("Ball & ({}, {})", x, y)
    }
}

#[wasm_bindgen]
pub fn new_game() -> Game {
    Game::default()
}

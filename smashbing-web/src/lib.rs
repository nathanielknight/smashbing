/// This module contains the interface between `libsmashbing` and the browser.
///
/// It imports `libshasmbing` sets up and controls the game, imports some
/// functions from JavaScript, and exports an API that can be kicked off when
/// the webpage loads.
extern crate libsmashbing;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// These functions are imported from JavaScript, and are implemented in a
// `<script>` tag in `index.js`.
#[wasm_bindgen]
extern "C" {
    fn draw_rect(x: f32, y: f32, w: f32, h: f32, c: String);
    fn play_sound(sound_id: &str);
    fn exit();
}

// This struct and its methods are exported to JavaScript, where they're called
// to start and control the game.
#[wasm_bindgen]
pub struct EmbeddedGame {
    game: libsmashbing::Game,
    commands: Vec<libsmashbing::Command>,
}

impl Default for EmbeddedGame {
    fn default() -> Self {
        let game = libsmashbing::Game::default();
        EmbeddedGame {
            game,
            commands: vec![],
        }
    }
}

#[wasm_bindgen]
impl EmbeddedGame {
    /// This is the function that's called when we do `new EmbeddedGame()` in
    /// JavaScript.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        EmbeddedGame::default()
    }

    /// The game loop is implemented in JavaScript; this function is called
    /// once per game loop.
    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32) {
        // Pass commands through to the game.
        let effects = self.game.update(dt, &self.commands);
        // Handle effects sent from the game engine.
        for effect in &effects {
            use libsmashbing::{Effect, SoundId};
            match effect {
                Effect::Exit => exit(),
                Effect::Sound(sid) => match sid {
                    SoundId::Bounce => play_sound("bounce"),
                    SoundId::BounceCharge => play_sound("bounce_charge"),
                    SoundId::Impulse => play_sound("impulse"),
                    SoundId::ImpulseExhaust => play_sound("impulse_exhaust"),
                    SoundId::Break1 => play_sound("break1"),
                    SoundId::Break2 => play_sound("break2"),
                    SoundId::Break3 => play_sound("break3"),
                    SoundId::Break4 => play_sound("break4"),
                    SoundId::Win => play_sound("win"),
                },
            }
        }

        self.render();
        self.commands.clear();
    }

    /// This function uses functions imported from JavaScript to draw the game.
    #[wasm_bindgen]
    #[allow(clippy::many_single_char_names)]
    pub fn render(&self) {
        // Background
        draw_rect(0.0, 0.0, 64.0, 64.0, "black".to_owned());
        // Blocks
        for block in &self.game.blocks {
            let x = block.rect.left;
            let y = 64.0 - block.rect.top;
            let w = block.rect.right - block.rect.left;
            let h = block.rect.top - block.rect.bottom;
            let c = Color::from_game_color(&block.color);
            draw_rect(x, y, w, h, c.as_style());
        }
        // Ball
        {
            let ball_pos = &self.game.ball.pos;
            const BALL_SIZE: f32 = 2.0;
            draw_rect(
                ball_pos.x - BALL_SIZE / 2.0,
                64.0 - (ball_pos.y - BALL_SIZE / 2.0),
                BALL_SIZE,
                BALL_SIZE,
                "red".to_owned(),
            );
        }
        // "Critters"
        const CRITTER_COLOR: &str = "blue";
        draw_rect(6.0, 59.0, 3.0, 3.0, CRITTER_COLOR.to_owned());
        for idx in 0..self.game.freed_critters() {
            draw_rect(
                10.0 + f32::from(idx) * 3.0,
                60.0,
                2.0,
                2.0,
                CRITTER_COLOR.to_owned(),
            );
        }
    }

    /// This function (which gets called in an `onClick` handler in JavaScript)
    /// translates user input into a command the game can handle.
    #[wasm_bindgen]
    pub fn fire_at(&mut self, x: f32, y: f32) {
        let cmd = libsmashbing::Command::Fire(x, 64.0 - y);
        self.commands.push(cmd);
    }
}

/// This struct converts from colors as they're represented in `libsmashbing`
/// to a format that's useful for the browser.
#[derive(Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    /// Createa a `Color` from a `libsmashbing` color.
    fn from_game_color(game_color: &libsmashbing::draw::Color) -> Self {
        let (r, g, b, a) = game_color;
        Color {
            r: float_to_html_color(*r),
            g: float_to_html_color(*g),
            b: float_to_html_color(*b),
            a: float_to_html_color(*a),
        }
    }

    /// Convert to a string suitable for use with `draw_rect`.
    fn as_style(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

/// Make sure color representations are between 0 and 1, and convert them to
/// the range that the browser uses (0-255).
fn float_to_html_color(x: f32) -> u8 {
    assert!(0.0 <= x && x <= 1.0, "Color out of bounds: {:?}", x);
    (x * 255.0) as u8
}

#[test]
fn test_float_to_html_color() {
    for i in 0..=100 {
        let x: f32 = i as f32 / 100.0;
        float_to_html_color(x);
    }
}

#[test]
fn test_float_to_html_on_game() {
    let game = EmbeddedGame::default();
}

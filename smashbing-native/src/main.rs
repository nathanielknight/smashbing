extern crate ggez;
extern crate libsmashbing;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::mouse;
use ggez::timer;
use ggez::{Context, GameResult};

use libsmashbing::draw;
use libsmashbing::{Effect, Game, SoundId};

mod sounds;

// TODO: Pixelating Shader
fn setup_graphics(ctx: &mut Context) -> GameResult<()> {
    graphics::set_fullscreen(ctx, false)?;
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 64.0, 64.0))?;
    graphics::set_background_color(ctx, graphics::BLACK);
    Ok(())
}

fn convert_color(inp: &draw::Color) -> graphics::Color {
    let (r, g, b, a) = *inp;
    graphics::Color::new(r, g, b, a)
}

struct CoordConverter {
    screen_size: (f32, f32),
}

impl CoordConverter {
    fn convert_from_pixels(&self, px: f32, py: f32) -> (f32, f32) {
        const DOMAIN_SIZE: f32 = 64.0;
        let (sx, sy) = self.screen_size;
        let x = px / sx * DOMAIN_SIZE;
        let vy = py / sy * DOMAIN_SIZE;
        let y = 64.0 - vy;
        (x, y)
    }

    fn new(ctx: &mut Context) -> CoordConverter {
        let (sx, sy) = graphics::get_size(ctx);
        CoordConverter {
            screen_size: (sx as f32, sy as f32),
        }
    }
}

struct NativeGame {
    game: Game,
    fire: bool,
    coords: CoordConverter,
    sound_repo: sounds::SoundRepo,
}

fn convert_rect(inp: &libsmashbing::rect::Rect) -> graphics::Rect {
    graphics::Rect::new(
        inp.left,
        64.0 - inp.top,
        inp.right - inp.left,
        inp.top - inp.bottom,
    )
}

impl event::EventHandler for NativeGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let cmds = if self.fire {
            self.fire = false; // Reset listener.
            let pos = mouse::get_position(ctx).expect("Error getting mouse position");
            let (x, y) = self.coords.convert_from_pixels(pos.x, pos.y);
            vec![libsmashbing::Command::Fire(x, y)]
        } else {
            Vec::new()
        };
        let delta = timer::get_delta(ctx);
        let dt = timer::duration_to_f64(delta);
        let effects = self.game.update(dt as f32, &cmds);
        self.do_effects(&effects)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Ball
        graphics::set_color(ctx, convert_color(&draw::BALL_COLOR))?;
        let ball_rect =
            graphics::Rect::new(self.game.ball.pos.x, 64.0 - self.game.ball.pos.y, 1.0, 1.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, ball_rect)?;

        // Blocks
        for block in &self.game.blocks {
            graphics::set_color(ctx, convert_color(&block.color))?;
            let block_rect = convert_rect(&block.rect);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, block_rect)?;
        }

        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _: &mut Context, _: event::MouseButton, _: i32, _: i32) {
        if !self.fire {
            self.fire = true;
        }
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Ballistic Smashbing", "Nathaniel Knight", c)
        .expect("Error creating context");
    setup_graphics(ctx).expect("Error setting up graphics");
    let converter = CoordConverter::new(ctx);
    let sounds = sounds::SoundRepo::new(ctx).expect("Error loading sounds");
    let mut game = NativeGame {
        game: libsmashbing::Game::default(),
        fire: false,
        coords: converter,
        sound_repo: sounds,
    };
    event::run(ctx, &mut game).expect("Error running game");
}

impl NativeGame {
    fn do_effects(&mut self, effects: &[Effect]) -> GameResult<()> {
        for effect in effects {
            match effect {
                Effect::Sound(sound_id) => self.play_sound(sound_id)?,
            }
        }
        Ok(())
    }

    fn play_sound(&mut self, sound_id: &SoundId) -> GameResult<()> {
        self.sound_repo.play(sound_id)?;
        Ok(())
    }
}

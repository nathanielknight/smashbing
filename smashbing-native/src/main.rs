extern crate ggez;
extern crate libsmashbing;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use libsmashbing::Game;

fn setup_graphics(ctx: &mut Context) -> GameResult<()> {
    graphics::set_resolution(ctx, 64, 64)?;
    graphics::set_fullscreen(ctx, true)?;
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 64.0, 64.0))?;
    graphics::set_background_color(ctx, graphics::BLACK);
    Ok(())
}

struct NativeGame {
    game: Game,
}

impl event::EventHandler for NativeGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // TODO: this
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Ballistic Smashbing", "Nathaniel Knight", c)
        .expect("Error creating context");
    setup_graphics(ctx).expect("Error setting up graphics");
}

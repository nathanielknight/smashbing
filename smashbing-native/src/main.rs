extern crate ggez;
extern crate libsmashbing;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use libsmashbing::draw;
use libsmashbing::rect;
use libsmashbing::Game;

fn setup_graphics(ctx: &mut Context) -> GameResult<()> {
    graphics::set_resolution(ctx, 64, 64)?;
    graphics::set_fullscreen(ctx, true)?;
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 64.0, 64.0))?;
    graphics::set_background_color(ctx, graphics::BLACK);
    Ok(())
}

fn convert_color(inp: &draw::Color) -> graphics::Color {
    let (r, g, b, a) = *inp;
    graphics::Color::new(r, g, b, a)
}

fn convert_rect(inp: &rect::Rect) -> graphics::Rect {
    graphics::Rect::new(
        inp.left,
        64.0 - inp.top,
        (inp.right - inp.left),
        (inp.top - inp.bottom),
    )
}

struct NativeGame {
    game: Game,
}

impl event::EventHandler for NativeGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const TARGET_FPS: u32 = 60;
        while timer::check_update_time(ctx, TARGET_FPS) {
            let delta = timer::get_delta(ctx);
            let dt = timer::duration_to_f64(delta);
            let cmds = [];
            self.game.update(dt as f32, &cmds);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        // Ball
        graphics::set_color(ctx, convert_color(&draw::BALL_COLOR))?;
        let ball_rect =
            graphics::Rect::new(self.game.ball.pos.x, 64.0 - self.game.ball.pos.y, 1.0, 1.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, ball_rect)?;

        graphics::present(ctx);
        println!("fps: {}", timer::get_fps(ctx));
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("Ballistic Smashbing", "Nathaniel Knight", c)
        .expect("Error creating context");
    setup_graphics(ctx).expect("Error setting up graphics");
    let mut game = NativeGame {
        game: libsmashbing::Game::default(),
    };
    event::run(ctx, &mut game).expect("Error running game");
}

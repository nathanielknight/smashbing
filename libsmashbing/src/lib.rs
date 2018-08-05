mod ball;
mod draw;
mod paddle;
mod rect;
mod vec;

/// Any object that should move during the dynamics step based **only** on
/// its internal state (e.g. ball, particles).
trait DynamicObject {
    /// Update dynamics for a timestep dt
    fn dyn_update(&mut self, dt: f32);
}

/// Point with velocity

pub struct Game {
    ball: ball::Ball,
    paddle: paddle::Paddle,
}

/// User input (read by implementations)
pub enum Command {
    MoveTowards(f32),
    None,
    Fire(f32, f32),
}

impl Game {
    pub fn update(&mut self, dt: f32, commands: &[Command]) {
        // Dynamics
        self.ball.update(dt);
        // Handle User Input
        for cmd in commands {
            match cmd {
                &Command::None => (),
                &Command::MoveTowards(x_pos) => self.paddle.move_towards(&x_pos),
                &Command::Fire(_, _) => unimplemented!(),
            }
        }
        // Collisions
        // Particles
    }

    pub fn screen(&self) -> draw::Screen {
        let mut scrn = draw::blank_screen();
        self.ball.draw_into(&mut scrn);
        scrn
    }
}

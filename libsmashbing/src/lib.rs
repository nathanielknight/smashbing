pub mod ball;
pub mod draw;
mod paddle;
pub mod rect;
pub mod vec;

/// Any object that should move during the dynamics step based **only** on
/// its internal state (e.g. ball, particles).
trait DynamicObject {
    /// Update dynamics for a timestep dt
    fn dyn_update(&mut self, dt: f32);
}

/// Point with velocity

pub struct Game {
    pub ball: ball::Ball,
    pub paddle: paddle::Paddle,
}

impl Default for Game {
    fn default() -> Game {
        let paddle = paddle::Paddle::default();
        Game {
            ball: ball::Ball::new(10.0, 10.0, 300.0, 300.0),
            paddle: paddle,
        }
    }
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
}

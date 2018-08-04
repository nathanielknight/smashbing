mod draw;
mod rect;
mod vec;

/// Any object that should move during the dynamics step based **only** on
/// its internal state (e.g. ball, particles).
trait DynamicObject {
    /// Update dynamics for a timestep dt
    fn dyn_update(&mut self, dt: f32);
}

/// Point with velocity
#[derive(Debug, PartialEq, Clone, Copy)]
struct PointInMotion {
    pos: vec::Vec2,
    vel: vec::Vec2,
}

impl PointInMotion {
    fn new(x: f32, y: f32, dx: f32, dy: f32) -> PointInMotion {
        PointInMotion {
            pos: vec::Vec2::new(x, y),
            vel: vec::Vec2::new(dx, dy),
        }
    }
}

impl DynamicObject for PointInMotion {
    fn dyn_update(&mut self, dt: f32) {
        self.pos += self.vel.scaled(dt);
    }
}

pub struct Game {
    ball: PointInMotion,
}

/// User input (read by implementations)
pub enum Command {
    Left(f32),
    None,
    Right(f32),
    Fire(f32, f32),
}

impl Game {
    pub fn update(&mut self, dt: f32, commands: &[Command]) {
        // Dynamics
        self.ball.dyn_update(dt);
        // User Input
        // Collisions
    }
}

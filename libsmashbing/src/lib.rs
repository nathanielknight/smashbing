use std::collections;

pub mod ball;
pub mod block;
pub mod draw;
pub mod rect;
pub mod vec;

/// Any object that should move during the dynamics step based **only** on
/// its internal state (e.g. ball, particles).
trait DynamicObject {
    /// Update dynamics for a timestep dt
    fn dyn_update(&mut self, dt: f32);
}

pub struct Game {
    pub ball: ball::Ball,
    pub blocks: collections::HashSet<block::Block>,
}

impl Default for Game {
    fn default() -> Game {
        Game {
            ball: ball::Ball::new(10.0, 10.0, 300.0, 300.0),
            blocks: block::new_blockset(),
        }
    }
}

/// User input (read by implementations)
#[derive(Debug)]
pub enum Command {
    None,
    Fire(f32, f32),
    // TODO: Implement a Reset command.
}

impl Game {
    pub fn update(&mut self, dt: f32, commands: &[Command]) {
        // Handle User Input
        for cmd in commands {
            match cmd {
                &Command::None => (),
                &Command::Fire(x, y) => self.ball.fire_at(x, y),
            }
        }
        // Collisions
        let mut colliding = false;
        for block in &self.blocks {
            if block.rect.contains(&self.ball.pos) {
                colliding = true;
            }
        }
        if colliding {
            self.ball.block_collide();
            let ball_pos = self.ball.pos;
            self.blocks.retain(|b| !b.rect.contains(&ball_pos));
        }

        // Dynamics
        self.ball.update(dt);
        // Particles (todo)
    }
}

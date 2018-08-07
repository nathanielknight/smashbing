extern crate rand;

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

#[derive(Debug, Clone)]
pub enum SoundId {
    Bounce,
    BounceCharge,
    Impulse,
    ImpulseExhaust,
    Break1,
    Break2,
    Break3,
    Break4,
    Win,
}

#[derive(Debug)]
pub enum Effect {
    Sound(SoundId),
}

impl Game {
    pub fn update(&mut self, dt: f32, commands: &[Command]) -> Vec<Effect> {
        let mut effects: Vec<Effect> = Vec::new();
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
            let collision_effects = self.ball.block_collide();
            effects.extend(collision_effects);
            let ball_pos = self.ball.pos;
            self.blocks.retain(|b| !b.rect.contains(&ball_pos));
        }

        // Dynamics
        let ball_effects = self.ball.update(dt);
        effects.extend(ball_effects);

        // Particles (todo)
        effects
    }
}

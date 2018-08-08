extern crate rand;

use std::collections;

pub mod ball;
pub mod block;
pub mod draw;
pub mod rect;
pub mod vec;

// TODO: Add Restart and Exit blocks when the field is cleared.

pub struct Game {
    pub ball: ball::Ball,
    pub blocks: collections::HashSet<block::Block>,
}

/// Minimum ball velocity for particle spawn.

impl Default for Game {
    fn default() -> Game {
        Game {
            ball: ball::Ball::default(),
            blocks: block::new_blockset(),
        }
    }
}

/// User input (read by implementations)
#[derive(Debug)]
pub enum Command {
    None,
    Fire(f32, f32),
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
                &Command::Fire(x, y) => {
                    let fire_effects = self.ball.fire_at(x, y);
                    effects.extend(fire_effects);
                }
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

        if self.blocks.iter().count() == 0 {
            self.reset();
        }

        effects
    }

    pub fn freed_critters(&self) -> u8 {
        block::freed_critters(&self.blocks)
    }

    fn reset(&mut self) {
        self.ball = ball::Ball::default();
        self.blocks = block::new_blockset();
    }
}

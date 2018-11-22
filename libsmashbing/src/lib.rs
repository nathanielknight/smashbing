/// This crate contains the game logic for SmashBing.

/// By providing a main loop, audio, rendering, and player input, handling
/// audio and rendering, programs on different platforms can make a program
/// that plays SmashBing without having to re-implement the game logic and
/// physics.
extern crate rand;

use std::collections;

pub mod ball;
pub mod block;
pub mod draw;
pub mod rect;
pub mod vec;

/// This struct contains all of the data for a running game of SmashBing, and
/// is the interface for an program implementing the game.
pub struct Game {
    pub ball: ball::Ball,
    pub blocks: collections::HashSet<block::Block>,
}

impl Default for Game {
    /// Generate a new game with the ball (almost) at rest and a random
    /// arrangement of blocks.
    fn default() -> Game {
        Game {
            ball: ball::Ball::default(),
            blocks: block::new_blockset(),
        }
    }
}

/// `Command` represents user input, which the implementing program has to
/// provide. A vector of commands should be passed to the game when calling
/// `Game::update`.
#[derive(Debug)]
pub enum Command {
    None,
    Fire(f32, f32),
}

/// An effect that the implementing program should handle. This includes sound
/// effects or exiting the game.
///
/// A vector of these will be returned from a Smashbing game's `update` method
/// for the implementing program to handle.
#[derive(Debug)]
pub enum Effect {
    Sound(SoundId),
    Exit,
}

/// `SoundId` enumerates all of the sounds that a SmashBing game might want to
/// make. This is used by `Effect::Sound` to indicate which sound effect to
/// play.
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

impl Game {
    /// Advances the game by `dt` seconds.
    ///
    /// `commands` should be a slice of `Commands`, for indicating player
    /// input.
    ///
    /// Returns a vector of `Effects` that the implementing program should
    /// handle.
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
        let mut reset = false;
        for block in &self.blocks {
            if block.rect.contains(&self.ball.pos) {
                colliding = true;
                match block.effect {
                    block::BlockEffect::None => (),
                    block::BlockEffect::Reset => reset = true,
                    block::BlockEffect::Exit => effects.push(Effect::Exit),
                };
            }
        }
        if reset {
            self.reset();
            return effects;
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
            effects.push(Effect::Sound(SoundId::Win));
            self.set_menu();
        }

        effects
    }

    /// How many pixle critters have been freed? Used to draw the family
    /// of pixel critters at the bottom of the screen.
    pub fn freed_critters(&self) -> u8 {
        block::freed_critters(&self.blocks)
    }

    /// Resets the game to a fresh initial state.
    fn reset(&mut self) {
        self.ball = ball::Ball::default();
        self.blocks = block::new_blockset();
    }

    /// Adds two special blocks, one that signals the game to exit and one that
    /// signals it to reset.
    ///
    /// This gets called when the final block is cleared.
    fn set_menu(&mut self) {
        self.blocks.insert(block::Block::new(
            0,
            8.0,
            26.0,
            (0.9, 0.1, 0.1, 0.1),
            false,
            block::BlockEffect::Exit,
        ));
        self.blocks.insert(block::Block::new(
            1,
            48.0,
            26.0,
            (0.1, 0.9, 0.1, 0.1),
            false,
            block::BlockEffect::Reset,
        ));
    }
}

use std::f32::consts::PI;

extern crate rand;
use rand::{thread_rng, Rng};

use vec::Vec2;

// The ball is constrained to within a few pixels of the edge. This assumes a
// 64 x 64 pixel play area.
const MIN_X: f32 = 3.0;
const MAX_X: f32 = 60.999;
const MIN_Y: f32 = 7.0;
const MAX_Y: f32 = 60.999;

/// Ball doesn't bounce if it hits the ground with less than this velocity.
const BOUNCE_THRESHOLD: f32 = 3.0;
/// Y-Velocity is scaled by this much with each bounce.
const BOUNCE_FACTOR: f32 = 0.55;
/// Magnitude of velocity to add when firing. (This is equivalent to an
/// impulse assuming the ball's mass is constant. Which it is. So there you
/// are.)
const FIRE_IMPULSE: f32 = 70.0;
/// Velocity is scaled by this much when colliding with a block
const BLOCK_DAMPING: f32 = 0.4;

/// Amount the ball's y velocity decreases each second.
const GRAVITATIONAL_ACCELERATION: f32 = -45.0;
/// Ball doesn't accelerate if less than this far from MIN_Y.
const NORMAL_THRESHOLD: f32 = 0.2;

/// Generate a random crash SoundId.
fn random_crash_sound() -> ::SoundId {
    const CRASH_SOUNDS: [::SoundId; 4] = [
        ::SoundId::Break1,
        ::SoundId::Break2,
        ::SoundId::Break3,
        ::SoundId::Break4,
    ];
    let rng = &mut rand::thread_rng();
    let sample = rand::seq::sample_iter(rng, CRASH_SOUNDS.iter(), 1).unwrap();
    sample[0].clone()
}

/// The player's ball
#[derive(Debug)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    /// The player can only impart impulse to the ball if it has charges. The
    /// ball starts with two charges when it leaves the ground.
    pub charges: u8,
    /// The random number generator that provides the ball's randomness.
    dist: rand::distributions::Uniform<f32>,
}

impl Ball {
    pub fn new(x: f32, y: f32, dx: f32, dy: f32) -> Ball {
        let dist = rand::distributions::Uniform::new(-PI / 6.0, PI / 6.0);
        Ball {
            pos: Vec2::new(x, y),
            vel: Vec2::new(dx, dy),
            dist,
            charges: 2,
        }
    }

    #[allow(clippy::useless_let_if_seq)]
    /// Advance the ball's simulation by `dt` seconds, yielding `Effect`s for
    /// any necessary sounds.
    pub fn update(&mut self, dt: f32) -> Vec<::Effect> {
        let mut effects = Vec::new();
        // Move to the next position (p' = p + v*dt)
        self.pos += self.vel.scaled(dt);
        // Don't fall if "resting" on the ground.
        if self.pos.y > MIN_Y + NORMAL_THRESHOLD {
            self.vel.y += GRAVITATIONAL_ACCELERATION * dt;
        }
        // Bounce elastically off side and top walls (no speed lost).
        let mut bounced = false;
        if self.pos.x < MIN_X {
            self.pos.x = MIN_X;
            self.vel.x *= -1.0;
            bounced = true;
        }
        if self.pos.x > MAX_X {
            self.pos.x = MAX_X;
            self.vel.x *= -1.0;
            bounced = true;
        }
        if self.pos.y > MAX_Y {
            self.pos.y = MAX_Y;
            self.vel.y *= -1.0;
            bounced = true;
        }
        // If a bounce happened, play a bounce sound effect.
        if bounced && self.vel.magnitude() > 0.7 {
            effects.push(::Effect::Sound(::SoundId::Bounce));
        }
        // Collide inelasticall with the ground (lose some speed).
        if self.pos.y < MIN_Y {
            if self.vel.magnitude() > 0.7 {
                if self.charges < 2 {
                    effects.push(::Effect::Sound(::SoundId::BounceCharge));
                    self.charges = 2;
                } else {
                    effects.push(::Effect::Sound(::SoundId::Bounce));
                }
            }
            if self.vel.magnitude() < BOUNCE_THRESHOLD {
                self.pos.y = MIN_Y;
                self.vel = Vec2::new(0.0, 0.0);
            } else {
                self.pos.y = MIN_Y;
                self.vel.y *= -BOUNCE_FACTOR;
                self.vel.x *= BOUNCE_FACTOR;
            }
        }
        effects
    }

    pub fn block_collide(&mut self) -> Vec<::Effect> {
        let mut effects = Vec::new();
        // Lose some speed and randomly rotate velocity a little.
        self.vel.scale(BLOCK_DAMPING);
        let mut rng = thread_rng();
        let rot = rng.sample(self.dist);
        self.vel.rotate(rot);
        // Play a crash sound.
        effects.push(::Effect::Sound(random_crash_sound()));
        effects
    }

    /// This is what should be run when the player clicks (or taps, or
    /// whatever). Imparts speed to the ball (if it has charges) and decreases
    /// the number of remaining charges.
    pub fn fire_at(&mut self, x: f32, y: f32) -> Vec<::Effect> {
        // If charges have been depleted, do nothing.
        if self.charges < 1 {
            return vec![];
        }
        // Reduce charges and continue.
        self.charges -= 1;
        // Calculate the vector between the ball and the player's click.
        let mut dv = Vec2::new(x - self.pos.x, y - self.pos.y);
        // Normalise; clicking near the ball or far away makes no difference.
        dv.normalise();
        // Give the ball a standard amount of speed in that direction.
        dv.scale(FIRE_IMPULSE);
        self.vel += dv;
        // Play a sound indicating if there are charges left or if the charges
        // have been exhausted.
        match self.charges {
            0 => vec![::Effect::Sound(::SoundId::ImpulseExhaust)],
            1 => vec![::Effect::Sound(::SoundId::Impulse)],
            _ => vec![],
        }
    }
}

impl Default for Ball {
    fn default() -> Ball {
        Ball::new(3.0, 3.0, 4.0, 4.0)
    }
}

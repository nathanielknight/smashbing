extern crate rand;
use rand::{thread_rng, Rng};

use vec::Vec2;

const PI: f32 = 3.1415927535;

const MIN_X: f32 = 3.0;
const MAX_X: f32 = 60.999;
const MIN_Y: f32 = 7.0;
const MAX_Y: f32 = 60.999;

/// Ball doesn't bounce if it hits the ground with less than this velocity.
const BOUNCE_THRESHOLD: f32 = 3.0;
/// Y-Velocity is scaled by this much with each bounce.
const BOUNCE_FACTOR: f32 = 0.55;

/// Magnitude of velocity to add when firing
const FIRE_IMPULSE: f32 = 70.0;

/// Velocity is scaled by this much when colliding with a block
const BLOCK_DAMPING: f32 = 0.3;

/// Amount the ball's y velocity decreases each second.
const GRAVITATIONAL_ACCELERATION: f32 = -60.0;
/// Ball doesn't accelerate if less than this far from MIN_Y.
const NORMAL_THRESHOLD: f32 = 0.2;

// TODO Add impulse charges that reset on the ground?

/// The player's ball
#[derive(Debug)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    dist: rand::distributions::Uniform<f32>,
}

impl Ball {
    pub fn new(x: f32, y: f32, dx: f32, dy: f32) -> Ball {
        let dist = rand::distributions::Uniform::new(-PI / 6.0, PI / 6.0);
        Ball {
            pos: Vec2::new(x, y),
            vel: Vec2::new(dx, dy),
            dist: dist,
        }
    }

    pub fn update(&mut self, dt: f32) -> Vec<::Effect> {
        let mut effects = Vec::new();
        self.pos += self.vel.scaled(dt);
        if self.pos.y > MIN_Y + NORMAL_THRESHOLD {
            // Don't fall if "resting" on the ground.
            self.vel.y += GRAVITATIONAL_ACCELERATION * dt;
        }
        // Collide elastically off side and top walls
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
        if bounced && self.vel.magnitude() > 0.7 {
            effects.push(::Effect::Sound(::SoundId::Bounce));
        }
        // Collide inelasticall with the ground
        if self.pos.y < MIN_Y {
            if self.vel.magnitude() > 0.7 {
                effects.push(::Effect::Sound(::SoundId::BounceCharge));
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

    pub fn block_collide(&mut self) {
        self.vel.scale(BLOCK_DAMPING);
        let mut rng = thread_rng();
        let rot = rng.sample(self.dist);
        self.vel.rotate(rot);
    }

    pub fn fire_at(&mut self, x: f32, y: f32) {
        let mut dv = Vec2::new(x - self.pos.x, y - self.pos.y);
        dv.normalise();
        dv.scale(FIRE_IMPULSE);
        self.vel += dv;
    }
}

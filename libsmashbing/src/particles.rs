use std::collections;

use rand;
use rand::{thread_rng, Rng};

use draw;
use vec;

const PI: f32 = 3.1415926535;

const PARTICLE_MAX_AGE: f32 = 1.8;

pub struct Particle {
    pub color: draw::Color,
    pub pos: vec::Vec2,
    vel: vec::Vec2,
    age: f32,
}

impl Particle {
    fn new_at(pos: &vec::Vec2, vel: &vec::Vec2) -> Particle {
        let dist = rand::distributions::Uniform::new(-PI / 3.0, PI / 3.0);
        let mut vel = vel.normalised();
        vel.scale(-1.0);
        let mut rng = thread_rng();
        let rot = rng.sample(dist);
        vel.rotate(rot);
        Particle {
            color: (1.0, 1.0, 1.0, 1.0),
            pos: pos.clone(),
            vel: vel,
            age: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.pos += self.vel.scaled(dt); // Move
        self.vel.scale(0.4 / dt); // Slow down
        self.color.3 *= 0.4 / dt; // Fade
        self.age += dt;
    }
}

pub struct ParticleSet {
    counter: u8,
    particles: collections::HashMap<u8, Particle>,
}

impl ParticleSet {
    pub fn empty() -> ParticleSet {
        ParticleSet {
            counter: 0,
            particles: collections::HashMap::new(),
        }
    }

    pub fn spawn(&mut self, at: &vec::Vec2, going: &vec::Vec2) {
        let new_particle = Particle::new_at(at, going);
        self.particles.insert(self.counter, new_particle);
        self.counter += 1;
    }

    pub fn update(&mut self, dt: f32) {
        self.particles.retain(|_, v| v.age <= PARTICLE_MAX_AGE);
        for (_, particle) in self.particles.iter_mut() {
            particle.update(dt);
        }
    }

    pub fn iter_particles(&self) -> impl Iterator<Item = &Particle> {
        self.particles.iter().map(|(_, v)| v)
    }
}

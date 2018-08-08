use std::cmp;
use std::collections;
use std::hash;

extern crate rand;
use rand::{thread_rng, Rng};

use draw;
use rect;

pub struct Block {
    id: u32,
    pub rect: rect::Rect,
    pub color: draw::Color,
    is_critter: bool,
}

const BLOCK_WIDTH: f32 = 8.0;
const BLOCK_HEIGHT: f32 = 5.0;

const CRITTER_BLOCKS: usize = 7;

impl Block {
    fn new(id: u32, x: f32, y: f32, c: draw::Color, critter: bool) -> Block {
        let r = rect::Rect::new(x, x + BLOCK_WIDTH, y, y + BLOCK_HEIGHT);
        Block {
            id: id,
            rect: r,
            color: c,
            is_critter: critter,
        }
    }
}

impl cmp::PartialEq for Block {
    /// Comparison is by identity, not by value.
    fn eq(&self, other: &Block) -> bool {
        self.id == other.id
    }
}

// Flag trait indicating that block equality is reflexive, symetric, and transitive
impl cmp::Eq for Block {}

impl hash::Hash for Block {
    /// Hash is based on identity, not value.
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

const BLOCK_COLS: usize = 6;
const BLOCK_ROWS: usize = 8;

fn random_color(i: &usize, j: &usize) -> draw::Color {
    let dist = rand::distributions::Uniform::new(-0.3, 0.3);
    let mut rng = thread_rng();
    let jitter: f32 = rng.sample(dist);

    let scale = ((i + j) as f32 / (BLOCK_ROWS - 1 + BLOCK_COLS - 1) as f32) + jitter;

    // This is a linear interpolation between two greens.
    // i.e: color = light - (dark - light) * scale
    println!("scale: {}", scale);
    let r = 0.392 + (0.049 - 0.392) * scale;
    let g = 0.875 + (0.456 - 0.875) * scale;
    let b = 0.129 + (0.133 - 0.129) * scale;
    (r, g, b, 1.0)
}

fn random_grey() -> draw::Color {
    let dist = rand::distributions::Uniform::new(-0.2, 0.2);
    let rng = &mut rand::thread_rng();
    let jitter: f32 = rng.sample(dist);
    let r = 0.3 + jitter;
    let g = 0.3 + jitter;
    let b = 0.3 + jitter;
    (r, g, b, 1.0)
}

fn random_critter_locations<'a>() -> [(u8, u8); CRITTER_BLOCKS] {
    let mut locs = [(0, 0); CRITTER_BLOCKS];
    let rng = &mut thread_rng();
    let mut points = Vec::new();
    for i in 0..BLOCK_COLS {
        for j in 0..BLOCK_ROWS {
            points.push((i, j));
        }
    }
    let mut idx = 0;
    for (i, j) in rand::seq::sample_iter(rng, points, CRITTER_BLOCKS).unwrap() {
        locs[idx] = (i as u8, j as u8);
        idx += 1;
    }
    locs
}

#[test]
fn test_criter_locations() {
    let locs = random_critter_locations();
    assert_eq!(locs.len(), CRITTER_BLOCKS);
    for l in locs.iter() {
        println!("loc: {}, {}", l.0, l.1);
    }
}

pub fn new_blockset() -> collections::HashSet<Block> {
    const BLOCKS_START_X: f32 = 8.0;
    const BLOCKS_START_Y: f32 = 16.0;
    let mut blocks = collections::HashSet::new();
    let mut id = 0;
    let critter_locations = random_critter_locations();
    for i in 0..BLOCK_COLS {
        for j in 0..BLOCK_ROWS {
            let critter = critter_locations.contains(&(i as u8, j as u8));
            let c = if critter {
                random_grey()
            } else {
                random_color(&i, &j)
            };
            let x = BLOCKS_START_X + (i as f32) * BLOCK_WIDTH;
            let y = BLOCKS_START_Y + (j as f32) * BLOCK_HEIGHT;
            let block = Block::new(id, x, y, c, critter);
            blocks.insert(block);
            id += 1;
        }
    }
    blocks
}

pub fn freed_critters(blocks: &collections::HashSet<Block>) -> u8 {
    (CRITTER_BLOCKS as u8) - blocks.iter().filter(|b| b.is_critter).count() as u8
}

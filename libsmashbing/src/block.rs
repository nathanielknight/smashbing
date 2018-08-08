use std::cmp;
use std::collections;
use std::hash;

extern crate rand;
use rand::{thread_rng, Rng};

use draw;
use rect;

// TODO: Add little pixel critters

pub struct Block {
    id: u32,
    pub rect: rect::Rect,
    pub color: draw::Color,
}

const BLOCK_WIDTH: f32 = 8.0;
const BLOCK_HEIGHT: f32 = 5.0;

impl Block {
    fn new(id: u32, x: f32, y: f32, c: draw::Color) -> Block {
        let r = rect::Rect::new(x, x + BLOCK_WIDTH, y, y + BLOCK_HEIGHT);
        Block {
            id: id,
            rect: r,
            color: c,
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

pub fn new_blockset() -> collections::HashSet<Block> {
    const BLOCKS_START_X: f32 = 8.0;
    const BLOCKS_START_Y: f32 = 16.0;
    let mut blocks = collections::HashSet::new();
    let mut id = 0;
    for i in 0..BLOCK_COLS {
        for j in 0..BLOCK_ROWS {
            let c = random_color(&i, &j);
            let x = BLOCKS_START_X + (i as f32) * BLOCK_WIDTH;
            let y = BLOCKS_START_Y + (j as f32) * BLOCK_HEIGHT;
            let block = Block::new(id, x, y, c);
            blocks.insert(block);
            id += 1;
        }
    }
    blocks
}

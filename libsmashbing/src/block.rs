use std::cmp;
use std::collections;

use std::hash;

use draw;
use rect;

pub struct Block {
    id: u32,
    pub rect: rect::Rect,
    pub color: draw::Color,
}

const BLOCK_WIDTH: f32 = 8.0;
const BLOCK_HEIGHT: f32 = 5.0;

impl Block {
    fn new(id: u32, x: f32, y: f32) -> Block {
        let r = rect::Rect::new(x, x + BLOCK_WIDTH, y, y + BLOCK_HEIGHT);
        let c = (1.0, 1.0, 1.0, 1.0);
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

pub fn new_blockset() -> collections::HashSet<Block> {
    // TODO: Colors (gradient? Random? Random Gradient?)
    const BLOCKS_START_X: f32 = 8.0;
    const BLOCKS_START_Y: f32 = 16.0;
    let mut blocks = collections::HashSet::new();
    let mut id = 0;
    for i in 0..6 {
        for j in 0..8 {
            let x = BLOCKS_START_X + (i as f32) * BLOCK_WIDTH;
            let y = BLOCKS_START_Y + (j as f32) * BLOCK_HEIGHT;
            let block = Block::new(id, x, y);
            blocks.insert(block);
            id += 1;
        }
    }
    blocks
}

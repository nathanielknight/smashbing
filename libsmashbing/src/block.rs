use std::cmp;
use std::collections;
use std::hash;

extern crate rand;
use rand::{thread_rng, Rng};

use draw;
use rect;

/// The red and green blocks and the end of the game can have special effects
/// (i.e. restarting the game or exiting the program). Most of the blocks will
/// have no effect.
pub enum BlockEffect {
    None,
    Reset,
    Exit,
}

pub struct Block {
    id: u32,
    pub rect: rect::Rect,
    pub color: draw::Color,
    is_critter: bool, // Critter blocks are grey instead of green.
    pub effect: BlockEffect,
}

const BLOCK_WIDTH: f32 = 8.0;
const BLOCK_HEIGHT: f32 = 5.0;

// Number of critter blocks to spawn.
const CRITTER_BLOCKS: usize = 7;

impl Block {
    pub fn new(
        id: u32,
        x: f32,
        y: f32,
        c: draw::Color,
        critter: bool,
        effect: BlockEffect,
    ) -> Block {
        let r = rect::Rect::new(x, x + BLOCK_WIDTH, y, y + BLOCK_HEIGHT);
        Block {
            id: id,
            rect: r,
            color: c,
            is_critter: critter,
            effect: effect,
        }
    }
}

impl cmp::PartialEq for Block {
    /// Comparison is by identity, not by value.
    fn eq(&self, other: &Block) -> bool {
        self.id == other.id
    }
}

// Flag trait indicating that block equality is reflexive, symetric, and
// transitive. Since block comparison is by identity, this just means we're
// assuming that block equality works like equality on regular whole numbers.
impl cmp::Eq for Block {}

/// This trait is implemented so that blocks can be put in a `HashSet`.
impl hash::Hash for Block {
    /// Hash is based on identity, not value.
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// How many rows and columns of blocks to render.
const BLOCK_COLS: usize = 6;
const BLOCK_ROWS: usize = 8;

/// Make sure `x` is between `floor` and `ceil`.
fn clamp(x: f32, floor: f32, ceil: f32) -> f32 {
    if x < floor {
        return floor;
    }
    if x > ceil {
        return ceil;
    }
    x
}

/// Get a slightly randomized color based on the block location.
fn block_color(i: &usize, j: &usize) -> draw::Color {
    let dist = rand::distributions::Uniform::new(-0.3, 0.3);
    let mut rng = thread_rng();
    let jitter: f32 = rng.sample(dist);

    let scale = ((i + j) as f32 / (BLOCK_ROWS - 1 + BLOCK_COLS - 1) as f32) + jitter;

    // This is a linear interpolation between two greens. When `scale` is -0.3,
    // we'd get the dark green. When `scale` is 0.3, we get the light green.
    let r = clamp(0.392 + (0.049 - 0.392) * scale, 0.0, 1.0);
    let g = clamp(0.875 + (0.456 - 0.875) * scale, 0.0, 1.0);
    let b = clamp(0.129 + (0.133 - 0.129) * scale, 0.0, 1.0);
    (r, g, b, 1.0)
}

// For critter blocks, a random grey color.
fn random_grey() -> draw::Color {
    let dist = rand::distributions::Uniform::new(-0.2, 0.2);
    let rng = &mut rand::thread_rng();
    let jitter: f32 = rng.sample(dist);
    let r = 0.3 + jitter;
    let g = 0.3 + jitter;
    let b = 0.3 + jitter;
    (r, g, b, 1.0)
}

// Generate random, non-repeating locations for the critters in terms of block
// indices.
fn random_critter_locations<'a>() -> [(u8, u8); CRITTER_BLOCKS] {
    let mut locs = [(0, 0); CRITTER_BLOCKS];
    let rng = &mut thread_rng();
    let mut points = Vec::new();
    // This could be implemented more efficiently, but it's only run once at
    // the beginning of each game, so we can use this more straightforward method.
    for i in 0..BLOCK_COLS {
        for j in 0..BLOCK_ROWS {
            points.push((i, j));
        }
    }
    let mut idx = 0;
    // Get unique block locations using `rand::seq::sample_iter`.
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
}

/// Construct a new blockset with randomized colors and critter locations.
pub fn new_blockset() -> collections::HashSet<Block> {
    // Location to start placing blocks, in pixels (assumes a 64x64 pixel play area).
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
                block_color(&i, &j)
            };
            let x = BLOCKS_START_X + (i as f32) * BLOCK_WIDTH;
            let y = BLOCKS_START_Y + (j as f32) * BLOCK_HEIGHT;
            let block = Block::new(id, x, y, c, critter, BlockEffect::None);
            blocks.insert(block);
            id += 1;
        }
    }
    blocks
}

// Count how many critters have been freed.
pub fn freed_critters(blocks: &collections::HashSet<Block>) -> u8 {
    (CRITTER_BLOCKS as u8) - blocks.iter().filter(|b| b.is_critter).count() as u8
}

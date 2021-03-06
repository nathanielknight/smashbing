/// Rectangular areas, for implementing graphics and collisions.

use vec::Vec2;

#[derive(PartialEq, Debug)]
pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Rect {
        Rect {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Check if this `Rect` contains a point (as a `Vec2`).
    pub fn contains(&self, point: Vec2) -> bool {
        let in_x = self.left <= point.x && point.x <= self.right;
        let in_y = self.bottom <= point.y && point.y <= self.top;
        in_x && in_y
    }

    /// Move this rect `dx` units horizontally and `dy` units vertically.
    /// As per the usual convention, left and down are negative.
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.left += dx;
        self.right += dx;
        self.bottom += dy;
        self.top += dy;
    }

    /// Create a new rect offset from this one by `dx` units horizontally
    /// and `dy` units vertically.
    pub fn translated(&self, dx: f32, dy: f32) -> Rect {
        Rect::new(
            self.left + dx,
            self.right + dx,
            self.bottom + dy,
            self.top + dy,
        )
    }

    /// Find the centre of this `Rect` (approximate, insofar as math on 
    /// `f32`s is approximate).
    pub fn center(&self) -> Vec2 {
        Vec2::new(
            (self.right + self.left) / 2.0,
            (self.bottom + self.top) / 2.0,
        )
    }
}

#[test]
fn test_contains() {
    let r = Rect::new(1.0, 2.0, 1.0, 2.0);
    assert!(r.contains(Vec2::new(1.5, 1.5)));
}

#[test]
fn test_translate() {
    let mut r = Rect::new(1.0, 2.0, 1.0, 2.0);
    r.translate(1.0, 0.0);
    assert_eq!(r, Rect::new(2.0, 3.0, 1.0, 2.0));
    r.translate(0.0, 1.0);
    assert_eq!(r, Rect::new(2.0, 3.0, 2.0, 3.0));
    r.translate(-1.0, -1.0);
    assert_eq!(r, Rect::new(1.0, 2.0, 1.0, 2.0));
}

#[test]
fn test_translated() {
    #[allow(unused_mut)]
    let mut orig = Rect::new(1.0, 2.0, 1.0, 2.0);
    let translated = orig.translated(1.0, 1.0);
    assert_eq!(translated, Rect::new(2.0, 3.0, 2.0, 3.0));
    assert_eq!(orig, Rect::new(1.0, 2.0, 1.0, 2.0));
}

#[test]
fn test_center_x() {
    let r1 = Rect::new(0.0, 2.0, 0.0, 2.0);
    assert_eq!(r1.center(), Vec2::new(1.0, 1.0));
}

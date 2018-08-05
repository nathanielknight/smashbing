/// Rectangular areas in space.
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
            left: left,
            right: right,
            top: top,
            bottom: bottom,
        }
    }

    pub fn contains(&self, point: &Vec2) -> bool {
        let in_x = self.left <= point.x && point.x <= self.right;
        let in_y = self.bottom <= point.y && point.y <= self.top;
        in_x && in_y
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.left += dx;
        self.right += dx;
        self.bottom += dy;
        self.top += dy;
    }

    pub fn translated(&self, dx: f32, dy: f32) -> Rect {
        Rect::new(
            self.left + dx,
            self.right + dx,
            self.bottom + dy,
            self.top + dy,
        )
    }
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
    assert!(r.contains(&Vec2::new(1.5, 1.5)));
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

impl Rect {
    fn distance_to(&self, p: &Vec2) -> Vec2 {
        let dx = if p.x < self.left {
            p.x - self.left
        } else if p.x > self.right {
            p.x - self.right
        } else {
            0.0
        };
        let dy = if p.y > self.top {
            p.y - self.top
        } else if p.y < self.bottom {
            p.y - self.bottom
        } else {
            0.0
        };
        Vec2::new(dx, dy)
    }
}

#[test]
fn test_rect_distance_to() {
    let r = Rect::new(1.0, 2.0, 1.0, 2.0);
    // let sqrt2 = (2.0 as f32).sqrt();
    let cases = vec![
        (Vec2::new(1.5, 1.5), Vec2::new(0.0, 0.0)),
        (Vec2::new(1.5, 3.0), Vec2::new(0.0, 1.0)),
        (Vec2::new(3.0, 1.5), Vec2::new(1.0, 0.0)),
        (Vec2::new(3.0, 3.0), Vec2::new(1.0, 1.0)),
        (Vec2::new(0.0, 0.0), Vec2::new(-1.0, -1.0)),
        (Vec2::new(1.5, 0.0), Vec2::new(0.0, -1.0)),
    ];
    for (pt, dist) in cases {
        assert_eq!(r.distance_to(&pt), dist);
    }
}

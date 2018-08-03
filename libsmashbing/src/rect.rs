/// Rectangular areas in space.
use vec::Vec2;

struct Rect {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Rect {
    fn new(left: f32, right: f32, bottom: f32, top: f32) -> Rect {
        Rect {
            left: left,
            right: right,
            top: top,
            bottom: bottom,
        }
    }

    fn contains(&self, point: Vec2) -> bool {
        let in_x = self.left <= point.x && point.x <= self.right;
        let in_y = self.bottom <= point.y && point.y <= self.top;
        in_x && in_y
    }
}

#[test]
fn test_contains() {
    let r = Rect::new(1.0, 2.0, 1.0, 2.0);
    assert!(r.contains(Vec2::new(1.5, 1.5)));
}

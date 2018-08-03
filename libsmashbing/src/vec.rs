/// Points and directions in space.
use std::f32::consts::PI;
use std::ops;

/// Point in 2D Space (modelled as a continuous field)
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    /// Get the vector's length.
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Rotate by a number of radians.
    pub fn rotate(&mut self, rads: f32) {
        let newx = self.x * rads.cos() - self.y * rads.sin();
        let newy = self.x * rads.sin() + self.y * rads.cos();
        self.x = newx;
        self.y = newy;
    }

    /// Scale the vector's length.
    pub fn scale(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }

    /// A new copy of the vector, scaled by the given scalar.
    pub fn scaled(&self, scalar: f32) -> Vec2 {
        let mut newvec = self.clone();
        newvec.scale(scalar);
        newvec
    }
}

#[test]
fn test_vec_magnitude() {
    assert_eq!(Vec2::new(1.0, 0.0).magnitude(), 1.0);
    assert_eq!(Vec2::new(0.0, 1.0).magnitude(), 1.0);
    assert_eq!(Vec2::new(1.0, 2.0).magnitude(), (5.0 as f32).sqrt());
}

#[test]
fn test_vec_rotate() {
    let mut v = Vec2::new(1.0, 0.0);
    v.rotate(PI / 2.0);
    assert!((v - Vec2::new(0.0, 1.0)).magnitude() < 1e-7);
    v.rotate(-PI / 2.0);
    assert!((v - Vec2::new(1.0, 0.0)).magnitude() < 1e-6);
    v.rotate(PI / 4.0);
    let sqrt2 = (2.0 as f32).sqrt() * 0.5;
    assert!(
        (v - Vec2::new(sqrt2, sqrt2)).magnitude() < 1e-7,
        "got: {:?}, want: {:?}",
        v,
        Vec2::new(sqrt2, sqrt2)
    );
}

impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[test]
fn test_moving() {
    let mut p = Vec2::new(0.0, 0.0);
    p += Vec2::new(1.0, 1.0);
    assert_eq!(p, Vec2::new(1.0, 1.0));
    p += Vec2::new(1.0, 1.0);
    assert_eq!(p, Vec2::new(2.0, 2.0));
    p += Vec2::new(-2.0, 0.0);
    assert_eq!(p, Vec2::new(0.0, 2.0));
    p -= Vec2::new(1.0, 1.0);
    assert_eq!(p, Vec2::new(-1.0, 1.0));
}

#[test]
fn test_vec_scale() {
    let mut v = Vec2::new(1.0, 2.0);
    v.scale(1.0);
    assert_eq!(v, Vec2::new(1.0, 2.0));
    v.scale(2.0);
    assert_eq!(v, Vec2::new(2.0, 4.0));
    v.scale(-3.0);
    assert_eq!(v, Vec2::new(-6.0, -12.0));
}

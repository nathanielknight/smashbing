/// RGBA Color. This type is mostly here to provide a common way to send colors
/// to implementors (e.g. SDL based native libraries, WASM based webpages).
pub type Color = (f32, f32, f32, f32);

pub const BALL_COLOR: Color = (1.0, 0.0, 0.0, 1.0);

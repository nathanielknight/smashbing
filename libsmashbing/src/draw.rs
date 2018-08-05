/// Helper functions for drawing objects into a Screen

/// RGBA Color
pub type Color = (f32, f32, f32, f32);

fn blank_color() -> Color {
    (0.0, 0.0, 0.0, 0.0)
}

/// Field to draw pixels on
pub type Screen = [[Color; 64]; 64];

pub fn blank_screen() -> Screen {
    [[(0.0, 0.0, 0.0, 0.0); 64]; 64]
}

pub trait Drawable {
    /// Draw this object into the given screen.
    fn draw_into(&self, scrn: &mut Screen);
}

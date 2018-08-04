/// Helper functions for drawing objects into a Screen

/// RGBA Color
pub type Color = (f32, f32, f32, f32);

/// Field to draw pixels on
pub type Screen = [[Color; 64]; 64];

trait DrawInto {
    /// Draw this object into the given screen.
    fn draw_into(&self, scrn: &mut Screen);
}

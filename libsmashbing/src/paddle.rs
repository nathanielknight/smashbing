/// The player's paddle.
use rect;

// Size of the play area
const MIN_X: f32 = 0.0;
const MAX_X: f32 = 100.0;

// Parameters for movement.
const MOVE_THRESHOLD: f32 = 2.0;
const MOVE_SCALE: f32 = 0.7;

pub struct Paddle {
    rect: rect::Rect,
    rebounding: bool,
}

// Paddle size
const PADDLE_BOTTOM: f32 = 4.0;
const PADDLE_WIDTH: f32 = 8.0;
const PADDLE_HEIGHT: f32 = 3.0;

impl Default for Paddle {
    fn default() -> Paddle {
        let cntr = (MAX_X - MIN_X) / 2.0;
        Paddle {
            rect: rect::Rect {
                left: cntr - PADDLE_WIDTH / 2.0,
                right: cntr + PADDLE_WIDTH / 2.0,
                bottom: PADDLE_BOTTOM,
                top: PADDLE_BOTTOM + PADDLE_HEIGHT,
            },
            rebounding: false,
        }
    }
}

impl Paddle {
    fn stay_in_bounds(&mut self) {
        if self.rect.left < MIN_X {
            let dx = MIN_X - self.rect.left;
            self.rect.translate(dx, 0.0);
        }
        if self.rect.right > MAX_X {
            let dx = MAX_X - self.rect.right;
            self.rect.translate(dx, 0.0);
        }
    }

    pub fn move_towards(&mut self, x: &f32) {
        // TODO(nknight): This
        self.stay_in_bounds();
    }
}

#[test]
fn test_paddle_move_towards() {
    // Test moving a short distance
    let mut p = Paddle::default();
    let x1 = p.rect.left - 1.0;
    p.move_towards(&x1);
    assert_eq!(p.rect.left, x1);
}

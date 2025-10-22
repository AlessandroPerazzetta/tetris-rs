/// Movement and game configuration parameters for Tetris.

/// Width of the Tetris grid (number of columns).
pub const GRID_WIDTH: usize = 10;

/// Height of the Tetris grid (number of rows).
pub const GRID_HEIGHT: usize = 20;

/// Width of the score panel.
pub const SCORE_WIDTH: f32 = 120.0;

/// Size of a single Tetris block (in pixels).
pub const BLOCK_SIZE: f32 = 30.0;

/// Delay (in seconds) between soft drop moves when holding the down key.
pub const SOFT_DROP_DELAY_VERTICAL: f32 = 0.05;

/// Delay (in seconds) between left/right moves when holding the left or right key.
pub const SOFT_DROP_DELAY_HORIZONTAL: f32 = 0.15;

/// Struct to group all movement timers for soft drop logic.
#[derive(Default)]
pub struct Timers {
    pub soft_drop_down: f32,
    pub soft_drop_left: f32,
    pub soft_drop_right: f32,
}
#[allow(dead_code)]
impl Timers {
    /// Resets all timers to zero.
    pub fn reset(&mut self) {
        self.soft_drop_down = 0.0;
        self.soft_drop_left = 0.0;
        self.soft_drop_right = 0.0;
    }
}

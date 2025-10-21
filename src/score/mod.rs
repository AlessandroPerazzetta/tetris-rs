use crate::GRID_WIDTH;
use crate::tetromino::BLOCK_SIZE;
use macroquad::prelude::*;

pub const SCORE_WIDTH: f32 = 120.0;

/// Draws the Tetris score
///
/// # Arguments
/// * `score` - The score actually reached
pub fn draw_score(score: u32) {
    let x = GRID_WIDTH as f32 * BLOCK_SIZE + 20.0;
    let y = 40.0;
    let font_size = 32u16;

    draw_text("Score:", x, y, font_size as f32, YELLOW);
    draw_text(
        &format!("{}", score),
        x,
        y + font_size as f32 + 8.0,
        font_size as f32,
        YELLOW,
    );
}

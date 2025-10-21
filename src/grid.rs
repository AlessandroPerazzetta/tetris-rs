use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use crate::tetromino::BLOCK_SIZE;
use macroquad::prelude::*;

/// Draws the Tetris grid using lines.
///
/// # Arguments
/// * `color` - The color of the grid lines.
pub fn draw_grid(color: Color) {
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            draw_rectangle_lines(
                x as f32 * BLOCK_SIZE,
                y as f32 * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                1.0,
                color,
            );
        }
    }
}

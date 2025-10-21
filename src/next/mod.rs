use crate::game::GRID_WIDTH;
use crate::score::SCORE_WIDTH;
use crate::tetromino::{BLOCK_SIZE, TetrominoShape};
use macroquad::prelude::*;

/// Draws the next tetromino block under the score panel.
///
/// # Arguments
/// * `next_shape` - The TetrominoShape of the next block
/// * `color` - The color to use for the next block
pub fn draw_next_tetromino(next_shape: &TetrominoShape, color: Color) {
    // Position to draw the next block (under the score)
    let panel_x = GRID_WIDTH as f32 * BLOCK_SIZE;
    let panel_width = SCORE_WIDTH;
    let next_label_y = 120.0;
    let block_area_y = next_label_y + 32.0;

    // Draw the "Next" label
    draw_text("Next:", panel_x + 20.0, next_label_y, 32.0, WHITE);

    // Find the bounding box of the tetromino shape
    let mut min_row = 4;
    let mut max_row = 0;
    let mut min_col = 4;
    let mut max_col = 0;
    for row in 0..4 {
        for col in 0..4 {
            if next_shape[row][col] != 0 {
                if row < min_row {
                    min_row = row;
                }
                if row > max_row {
                    max_row = row;
                }
                if col < min_col {
                    min_col = col;
                }
                if col > max_col {
                    max_col = col;
                }
            }
        }
    }
    let shape_width = (max_col - min_col + 1) as f32 * BLOCK_SIZE;
    let shape_height = (max_row - min_row + 1) as f32 * BLOCK_SIZE;

    // Center the tetromino in the panel area
    let center_x = panel_x + (panel_width - shape_width) / 2.0;
    let center_y = block_area_y + 32.0 - shape_height / 2.0;

    // Draw the tetromino shape (4x4 grid)
    for row in 0..4 {
        for col in 0..4 {
            if next_shape[row][col] != 0 {
                let x = center_x + (col - min_col) as f32 * BLOCK_SIZE;
                let y = center_y + (row - min_row) as f32 * BLOCK_SIZE;
                draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, color);
                // Optionally, add a border for better visibility
                draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 2.0, BLACK);
            }
        }
    }
}

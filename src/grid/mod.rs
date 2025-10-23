// This module provides the logic for rendering the Tetris playfield grid.
//
// Principles and design choices:
//
// - Contains functions to draw the grid lines that visually separate each cell in the playfield.
// - Uses configurable parameters for grid size and block size to ensure consistency with the rest of the game.
// - Designed for clarity and easy integration with the main rendering loop.

use macroquad::prelude::*;

use crate::parameters::{BLOCK_SIZE, GRID_HEIGHT, GRID_WIDTH};

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

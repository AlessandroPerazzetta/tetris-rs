// This module provides the core grid and gameplay logic for Tetris.
//
// Principles and design choices:
//
// - Defines the Grid type as a 2D array representing the playfield, where each cell is either empty or holds a color.
// - Implements collision detection for tetromino placement, ensuring pieces do not overlap existing blocks or boundaries.
// - Handles stacking of tetrominoes into the grid, updating the playfield state with the correct color.
// - Provides efficient line clearing logic, removing filled rows and shifting above rows down.
// - Includes a function to draw all stacked blocks in the grid for rendering.
// - All logic is designed for clarity, efficiency, and compatibility with the standard Tetris rules.

use macroquad::prelude::*;

use crate::parameters::{BLOCK_SIZE, GRID_HEIGHT, GRID_WIDTH};
use crate::tetromino::TetrominoShape;

/// Type alias for the Tetris grid, a 2D array of optional Colors.
/// Each cell is either None (empty) or Some(Color) (filled with a block of that color).
pub type Grid = [[Option<Color>; GRID_WIDTH]; GRID_HEIGHT];

/// Checks if placing the tetromino at (x, y) would result in a collision with the grid or boundaries.
///
/// # Arguments
/// * `grid` - The current state of the Tetris grid.
/// * `shape` - The shape of the tetromino to check.
/// * `x` - The x position to check (in grid coordinates).
/// * `y` - The y position to check (in grid coordinates).
/// # Returns
/// * `true` if there is a collision, `false` otherwise.
pub fn check_collision(grid: &Grid, shape: &TetrominoShape, x: i32, y: i32) -> bool {
    for row in 0..4 {
        for col in 0..4 {
            if shape[row][col] != 0 {
                let nx = x + col as i32;
                let ny = y + row as i32;
                if nx < 0 || nx >= GRID_WIDTH as i32 || ny >= GRID_HEIGHT as i32 {
                    return true;
                }
                if ny >= 0 && grid[ny as usize][nx as usize].is_some() {
                    return true;
                }
            }
        }
    }
    false
}

/// Stacks the tetromino into the grid at position (x, y) with the given color.
///
/// # Arguments
/// * `grid` - The current state of the Tetris grid (mutable).
/// * `shape` - The shape of the tetromino to stack.
/// * `x` - The x position to stack at (in grid coordinates).
/// * `y` - The y position to stack at (in grid coordinates).
/// * `color` - The color of the tetromino.
pub fn stack_tetromino(grid: &mut Grid, shape: &TetrominoShape, x: i32, y: i32, color: Color) {
    for row in 0..4 {
        for col in 0..4 {
            if shape[row][col] != 0 {
                let nx = x + col as i32;
                let ny = y + row as i32;
                if ny >= 0 && ny < GRID_HEIGHT as i32 && nx >= 0 && nx < GRID_WIDTH as i32 {
                    grid[ny as usize][nx as usize] = Some(color);
                }
            }
        }
    }
}

/// Draws the stacked blocks in the grid.
///
/// # Arguments
/// * `grid` - The current state of the Tetris grid.
pub fn draw_grid_blocks(grid: &Grid) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if let Some(color) = grid[y][x] {
                draw_rectangle(
                    x as f32 * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    BLOCK_SIZE - 2.0,
                    BLOCK_SIZE - 2.0,
                    color,
                );
            }
        }
    }
}

/// Clears filled lines in the grid.
///
/// # Arguments
/// * `grid` - The current state of the Tetris grid (mutable).
/// # Returns
/// * The number of lines cleared.
pub fn clear_lines(grid: &mut Grid) -> usize {
    let mut cleared = 0;
    let mut y = GRID_HEIGHT as i32 - 1;
    while y >= 0 {
        if grid[y as usize].iter().all(|cell| cell.is_some()) {
            // Remove the line and insert an empty one at the top
            for row in (1..=y as usize).rev() {
                grid[row] = grid[row - 1];
            }
            grid[0] = [None; GRID_WIDTH];
            cleared += 1;
            // Stay at the same y to check the new row that fell down
        } else {
            y -= 1;
        }
    }
    cleared
}

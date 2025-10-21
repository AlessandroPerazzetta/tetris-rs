use crate::tetromino::{BLOCK_SIZE, TetrominoShape};
use macroquad::prelude::*;

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;

pub type Grid = [[Option<Color>; GRID_WIDTH]; GRID_HEIGHT];

/// Checks if placing the tetromino at (x, y) would result in a collision with the grid or boundaries.
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
/// Returns the number of lines cleared.
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

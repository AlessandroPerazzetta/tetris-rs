// This module defines the core logic for Tetromino shapes and their manipulation in Tetris.
//
// Principles and design choices:
//
// - TetrominoShape is defined as a 4x4 grid ([[u8; 4]; 4]) because all standard Tetris pieces (I, O, S, Z, J, L, T)
//   can fit within a 4x4 matrix in any rotation. This uniform size simplifies rotation, collision detection, and drawing logic.
// - The SHAPES constant lists all 7 standard tetrominoes using this 4x4 representation.
// - The rotate() function rotates a tetromino 90 degrees clockwise within its 4x4 grid. This is done by transposing
//   and then reversing rows, which is efficient and works for all pieces due to the fixed size.
// - draw_tetromino() draws any tetromino on the grid by iterating over its 4x4 matrix and drawing a block for each nonzero cell.
// - TetrominoBag implements the "bag of 7" system: each cycle, all 7 tetrominoes are shuffled and dealt out one by one,
//   ensuring fair and modern piece distribution (no repeats until all have appeared).
//
// This approach is standard in modern Tetris implementations for simplicity, fairness, and code maintainability.

use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

use crate::parameters::BLOCK_SIZE;

/// Type alias for a Tetromino shape represented as a 4x4 grid
pub type TetrominoShape = [[u8; 4]; 4];

/// All 7 standard Tetromino shapes in their initial orientation
/// Each shape is a 4x4 grid where 1s represent filled blocks and 0s represent empty space.
/// The shapes are defined as follows:
/// I: Straight line
/// O: Square
/// S: Skewed Shape
/// Z: Skewed Z Shape
/// J: Left L Shape
/// L: Right L Shape
/// T: T Shape
pub const SHAPES: [TetrominoShape; 7] = [
    // I
    [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
    // O
    [[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    // S
    [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    // Z
    [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    // J
    [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    // L
    [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    // T
    [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
];

/// Rotate the tetromino
///
/// # Arguments
/// * `shape` - The TetrominoShape to rotate
pub fn rotate(shape: &TetrominoShape) -> TetrominoShape {
    let mut new_shape = [[0u8; 4]; 4];
    for y in 0..4 {
        for x in 0..4 {
            new_shape[x][3 - y] = shape[y][x];
        }
    }
    new_shape
}

/// Drwaw the tetromino
///
/// # Arguments
/// * `shape` - The TetrominoShape to draw
/// * `grid_x` - The x position on the grid to draw at (in grid coordinates)
/// * `grid_y` - The y position on the grid to draw at (in grid coordinates)
/// * `color` - The color to use for drawing the tetromino
pub fn draw_tetromino(shape: &TetrominoShape, grid_x: i32, grid_y: i32, color: Color) {
    for row in 0..4 {
        for col in 0..4 {
            if shape[row][col] != 0 {
                draw_rectangle(
                    (grid_x + col as i32) as f32 * BLOCK_SIZE,
                    (grid_y + row as i32) as f32 * BLOCK_SIZE,
                    BLOCK_SIZE - 2.0,
                    BLOCK_SIZE - 2.0,
                    color,
                );
            }
        }
    }
}

/// Bag of 7 system for tetromino generation
pub struct TetrominoBag {
    bag: Vec<usize>,
    index: usize,
}

/// Implementation of TetrominoBag
impl TetrominoBag {
    pub fn new() -> Self {
        let mut bag: Vec<usize> = (0..7).collect();
        bag.shuffle();
        Self { bag, index: 0 }
    }

    /// Get the next tetromino index from the bag
    pub fn next(&mut self) -> usize {
        let idx = self.bag[self.index];
        self.index += 1;
        if self.index >= self.bag.len() {
            self.bag = (0..7).collect();
            self.bag.shuffle();
            self.index = 0;
        }
        idx
    }

    /// Peek at the next tetromino index without advancing the bag
    pub fn peek(&self) -> usize {
        self.bag[self.index]
    }
}

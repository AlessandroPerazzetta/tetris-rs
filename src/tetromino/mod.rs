use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

use crate::parameters::BLOCK_SIZE;

pub type TetrominoShape = [[u8; 4]; 4];

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

impl TetrominoBag {
    pub fn new() -> Self {
        let mut bag: Vec<usize> = (0..7).collect();
        bag.shuffle();
        Self { bag, index: 0 }
    }

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

    pub fn peek(&self) -> usize {
        self.bag[self.index]
    }
}

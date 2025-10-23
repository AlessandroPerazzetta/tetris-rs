// This module defines the GameInfo panel, which displays the player's score, lines cleared,
// current level, and a preview of the next tetromino. It provides a unified and consistently
// spaced side panel for all essential game information.
//
// Principles and design choices:
//
// - The GameInfo struct tracks score, lines cleared, next tetromino shape/color, and level.
// - The draw() method renders all info in a visually grouped and spaced layout, making it easy
//   for the player to see their progress and upcoming piece at a glance.
// - The next tetromino preview is centered in its area, regardless of shape, for a polished look.
// - All layout constants (panel position, font size, spacing, etc.) are defined at the top for
//   easy adjustment and consistent appearance.
// - The module is designed for clarity, extensibility, and a modern Tetris UI experience.

use macroquad::prelude::*;

use crate::parameters::{BLOCK_SIZE, GRID_WIDTH, SCORE_WIDTH};
use crate::tetromino::TetrominoShape;

/// Panel layout constants for consistent spacing and positioning of game information.
const PANEL_X: f32 = GRID_WIDTH as f32 * BLOCK_SIZE + 20.0;
const PANEL_Y: f32 = 40.0;
const FONT_SIZE: f32 = 32.0;
const SECTION_SPACING: f32 = 24.0;
const SECTION_HEIGHT: f32 = FONT_SIZE + 16.0;
const NEXT_PREVIEW_WIDTH: f32 = SCORE_WIDTH - 40.0;
const NEXT_PREVIEW_HEIGHT: f32 = 100.0; // Adjust as needed for your layout

/// Struct to hold and manage game information for the side panel.
/// Tracks score, lines cleared, next tetromino, and level.
pub struct GameInfo {
    pub score: u32,
    pub lines_cleared: u32,
    pub next_shape: TetrominoShape,
    pub next_color: Color,
    pub level: usize,
}

/// Implementation of GameInfo methods.
/// Provides functionality to update and draw the game info panel.
/// Methods include adding score, setting next tetromino, setting level, and drawing the panel.
impl GameInfo {
    pub fn new(next_shape: TetrominoShape, next_color: Color) -> Self {
        Self {
            score: 0,
            lines_cleared: 0,
            next_shape,
            next_color,
            level: 1,
        }
    }

    /// Adds score and updates lines cleared based on the number of lines just cleared.
    /// 
    /// # Arguments
    /// * `lines` - The number of lines cleared in the last move.
    pub fn add_score(&mut self, lines: u32) {
        self.score += 100 * lines * lines;
        self.lines_cleared += lines;
    }

    /// Sets the next tetromino's shape and color.
    /// 
    /// # Arguments
    /// * `shape` - The TetrominoShape of the next block
    /// * `color` - The color to use for the next block
    pub fn set_next(&mut self, shape: TetrominoShape, color: Color) {
        self.next_shape = shape;
        self.next_color = color;
    }

    /// Sets the current level.
    /// 
    /// # Arguments
    /// * `level` - The new level to set
    pub fn set_level(&mut self, level: usize) {
        self.level = level;
    }

    /// Draws the score, lines cleared, and next tetromino preview.
    pub fn draw(&self) {
        let mut y = PANEL_Y;

        // Draw score
        draw_text("Score:", PANEL_X, y, FONT_SIZE, YELLOW);
        draw_text(
            &format!("{}", self.score),
            PANEL_X,
            y + FONT_SIZE + 8.0,
            FONT_SIZE,
            YELLOW,
        );
        y += SECTION_HEIGHT + SECTION_SPACING;

        // Draw lines cleared
        draw_text("Lines:", PANEL_X, y, FONT_SIZE, GREEN);
        draw_text(
            &format!("{}", self.lines_cleared),
            PANEL_X,
            y + FONT_SIZE + 8.0,
            FONT_SIZE,
            GREEN,
        );
        y += SECTION_HEIGHT + SECTION_SPACING;

        // Draw next tetromino label and preview
        draw_text("Next:", PANEL_X, y, FONT_SIZE, WHITE);
        // Draw the tetromino below the label, with some extra space
        draw_next_tetromino(
            &self.next_shape,
            self.next_color,
            PANEL_X,
            y + FONT_SIZE + 12.0,
            NEXT_PREVIEW_WIDTH,
            NEXT_PREVIEW_HEIGHT,
        );

        // Draw current level below the next tetromino preview, with spacing
        let level_text = format!("Level: {}", self.level);
        let level_y = y + FONT_SIZE + NEXT_PREVIEW_HEIGHT + 32.0; // 32.0 for extra spacing
        draw_text(
            &level_text,
            PANEL_X,
            level_y,
            FONT_SIZE,
            YELLOW,
        );
    }
}

/// Draws the next tetromino block under the score panel.
///
/// # Arguments
/// * `next_shape` - The TetrominoShape of the next block
/// * `color` - The color to use for the next block
fn draw_next_tetromino(
    next_shape: &TetrominoShape,
    color: Color,
    area_x: f32,
    area_y: f32,
    area_width: f32,
    area_height: f32,
) {
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

    // Center the tetromino in the preview area
    let center_x = area_x + (area_width - shape_width) / 2.0;
    let center_y = area_y + (area_height - shape_height) / 2.0;

    // Draw the tetromino shape (4x4 grid)
    for row in 0..4 {
        for col in 0..4 {
            if next_shape[row][col] != 0 {
                let x = center_x + (col - min_col) as f32 * BLOCK_SIZE;
                let y = center_y + (row - min_row) as f32 * BLOCK_SIZE;
                draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, color);
                draw_rectangle_lines(x, y, BLOCK_SIZE, BLOCK_SIZE, 2.0, BLACK);
            }
        }
    }
}
use macroquad::prelude::*;

mod game;
mod grid;
mod next;
mod parameters;
mod score;
mod state;
mod tetromino;
mod ui;
use game::{
    GRID_HEIGHT, GRID_WIDTH, Grid, check_collision, clear_lines, draw_grid_blocks, stack_tetromino,
};
use grid::draw_grid;
use next::draw_next_tetromino;
use parameters::{SOFT_DROP_DELAY_HORIZONTAL, SOFT_DROP_DELAY_VERTICAL, Timers};
use score::{SCORE_WIDTH, draw_score};
use state::GameState;
use tetromino::{BLOCK_SIZE, SHAPES, TetrominoBag, draw_tetromino, rotate};
use ui::draw_centered_text;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetromino Shapes, Rotation, and Grid".to_owned(),
        window_width: (GRID_WIDTH as f32 * BLOCK_SIZE + SCORE_WIDTH) as i32,
        window_height: (GRID_HEIGHT as f32 * BLOCK_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Running;
    let mut grid: Grid = [[None; GRID_WIDTH]; GRID_HEIGHT];

    let tetromino_colors = [ORANGE, YELLOW, GREEN, RED, BLUE, PURPLE, PINK];

    // Use TetrominoBag for bag of 7 system
    let mut bag = TetrominoBag::new();
    let mut bag_index = 0;
    let mut shape_idx = bag.next();
    let mut shape = SHAPES[shape_idx];
    let mut color = tetromino_colors[shape_idx];
    let mut next_idx = bag.peek();
    let mut next_shape = SHAPES[next_idx];
    let mut next_color = tetromino_colors[next_idx];

    let mut grid_x = 3;
    let mut grid_y = 0;
    let mut score = 0;
    let mut fall_timer = 0.0;
    let fall_delay = 0.5;
    let mut timers = Timers::default();

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Running => {
                // ---------------------------------------------
                // Falling logic
                // ---------------------------------------------
                fall_timer += get_frame_time();
                if fall_timer > fall_delay {
                    fall_timer = 0.0;
                    if !check_collision(&grid, &shape, grid_x, grid_y + 1) {
                        grid_y += 1;
                    } else {
                        stack_tetromino(&mut grid, &shape, grid_x, grid_y, color);
                        // Line clearing and scoring
                        let lines_cleared = clear_lines(&mut grid);
                        if lines_cleared > 0 {
                            score += 100 * lines_cleared * lines_cleared;
                        }
                        // Spawn new tetromino using TetrominoBag
                        shape_idx = bag.next();
                        shape = SHAPES[shape_idx];
                        color = tetromino_colors[shape_idx];
                        next_idx = bag.peek();
                        next_shape = SHAPES[next_idx];
                        next_color = tetromino_colors[next_idx];
                        grid_x = 3;
                        grid_y = 0;
                        // Game over if new tetromino collides immediately
                        if check_collision(&grid, &shape, grid_x, grid_y) {
                            game_state = GameState::GameOver;
                        }
                    }
                }

                // ---------------------------------------------
                // KEY UP LOGIC
                // ---------------------------------------------
                // Controls: rotate, move
                if is_key_pressed(KeyCode::Up) {
                    let rotated = rotate(&shape);
                    if !check_collision(&grid, &rotated, grid_x, grid_y) {
                        shape = rotated;
                    }
                }
                // This modified version allow rotation when near border moving block to allow rotation
                // TODO: test and decide which one is better
                //
                // if is_key_pressed(KeyCode::Up) {
                //     let rotated = rotate(&shape);

                //     // Try rotating in place
                //     if !check_collision(&grid, &rotated, grid_x, grid_y) {
                //         shape = rotated;
                //     } else {
                //         // Try wall kicks: left/right by 1 or 2 cells
                //         let kicks = [-1, 1, -2, 2];
                //         let mut kicked = false;
                //         for &dx in &kicks {
                //             if !check_collision(&grid, &rotated, grid_x + dx, grid_y) {
                //                 shape = rotated;
                //                 grid_x += dx;
                //                 kicked = true;
                //                 break;
                //             }
                //         }
                //         // If none of the kicks work, do not rotate
                //     }
                // }

                // ---------------------------------------------
                // KEY RIGHT LOGIC
                // ---------------------------------------------
                // Soft drop logic for right movement
                if is_key_down(KeyCode::Right) {
                    timers.soft_drop_right += get_frame_time();
                    if timers.soft_drop_right > SOFT_DROP_DELAY_HORIZONTAL {
                        timers.soft_drop_right = 0.0;
                        if !check_collision(&grid, &shape, grid_x + 1, grid_y) {
                            grid_x += 1;
                        }
                    }
                } else {
                    timers.soft_drop_right = 0.0;
                }
                // Also allow instant move right on key press
                if is_key_pressed(KeyCode::Right) {
                    if !check_collision(&grid, &shape, grid_x + 1, grid_y) {
                        grid_x += 1;
                    }
                }

                // ---------------------------------------------
                // KEY LEFT LOGIC
                // ---------------------------------------------
                // Soft drop logic for left movement
                if is_key_down(KeyCode::Left) {
                    timers.soft_drop_left += get_frame_time();
                    if timers.soft_drop_left > SOFT_DROP_DELAY_HORIZONTAL {
                        timers.soft_drop_left = 0.0;
                        if !check_collision(&grid, &shape, grid_x - 1, grid_y) {
                            grid_x -= 1;
                        }
                    }
                } else {
                    timers.soft_drop_left = 0.0;
                }
                // Also allow instant move left on key press
                if is_key_pressed(KeyCode::Left) {
                    if !check_collision(&grid, &shape, grid_x - 1, grid_y) {
                        grid_x -= 1;
                    }
                }

                // ---------------------------------------------
                // KEY DOWN LOGIC
                // ---------------------------------------------
                // Soft drop logic for down movement
                if is_key_down(KeyCode::Down) {
                    timers.soft_drop_down += get_frame_time();
                    if timers.soft_drop_down > SOFT_DROP_DELAY_VERTICAL {
                        timers.soft_drop_down = 0.0;
                        if !check_collision(&grid, &shape, grid_x, grid_y + 1) {
                            grid_y += 1;
                        }
                    }
                } else {
                    timers.soft_drop_down = 0.0;
                }
                // Also allow instant drop on key press for responsiveness
                if is_key_pressed(KeyCode::Down) {
                    if !check_collision(&grid, &shape, grid_x, grid_y + 1) {
                        grid_y += 1;
                    }
                }

                // ---------------------------------------------
                // KEY SPACE LOGIC
                // ---------------------------------------------
                if is_key_pressed(KeyCode::Space) {
                    // Hard drop
                    while !check_collision(&grid, &shape, grid_x, grid_y + 1) {
                        grid_y += 1;
                    }
                }

                // Draw grid lines
                draw_grid(GRAY);
                // Draw stacked blocks
                draw_grid_blocks(&grid);
                // Draw active tetromino
                draw_tetromino(&shape, grid_x, grid_y, color);
                // Draw score
                draw_score(score.try_into().unwrap());
                // Draw next tetromino preview
                draw_next_tetromino(&next_shape, next_color);
            }
            GameState::Paused => {
                draw_centered_text("Paused", 60.0, YELLOW);
            }
            GameState::GameOver => {
                draw_centered_text("Game Over", 60.0, RED);
            }
        }

        // toggle pause with Enter key
        if is_key_pressed(KeyCode::Enter) {
            game_state = if game_state == GameState::Running {
                GameState::Paused
            } else if game_state == GameState::Paused {
                GameState::Running
            } else {
                game_state
            };
        }
        next_frame().await
    }
}

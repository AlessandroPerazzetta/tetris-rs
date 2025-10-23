use macroquad::prelude::*;

mod game;
mod game_info;
mod grid;
mod parameters;
mod state;
mod tetromino;
mod ui;

use crate::parameters::{
    BLOCK_SIZE, GRID_HEIGHT, GRID_WIDTH, SCORE_WIDTH, SOFT_DROP_DELAY_HORIZONTAL,
    SOFT_DROP_DELAY_VERTICAL, Timers, Difficulty
};
use game::{Grid, check_collision, clear_lines, draw_grid_blocks, stack_tetromino};
use game_info::GameInfo;
use grid::draw_grid;
use state::GameState;
use tetromino::{SHAPES, TetrominoBag, draw_tetromino, rotate};
use ui::{draw_centered_text, draw_bottom_centered_text, draw_difficulty_menu};

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris RS".to_owned(),
        window_width: (GRID_WIDTH as f32 * BLOCK_SIZE + SCORE_WIDTH) as i32,
        window_height: (GRID_HEIGHT as f32 * BLOCK_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Waiting;
    let mut grid: Grid = [[None; GRID_WIDTH]; GRID_HEIGHT];

    let tetromino_colors = [ORANGE, YELLOW, GREEN, RED, BLUE, PURPLE, PINK];

    // Use TetrominoBag for bag of 7 system
    let mut bag = TetrominoBag::new();
    let mut shape_idx = bag.next();
    let mut shape = SHAPES[shape_idx];
    let mut color = tetromino_colors[shape_idx];
    let mut next_idx = bag.peek();
    let mut next_shape = SHAPES[next_idx];
    let mut next_color = tetromino_colors[next_idx];

    let mut game_info = GameInfo::new(next_shape, next_color);

    let mut grid_x = 3;
    let mut grid_y = 0;
    let mut fall_timer = 0.0;
    let mut timers = Timers::default();

    let mut selected_menu = 1; // 0: Easy, 1: Medium, 2: Hard
    let difficulties = [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];
    let mut difficulty = Difficulty::Medium;
    let mut fall_delay = difficulty.fall_delay();
    let mut level_timer = 0.0;
    let mut level = 1;

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Waiting => {
                // Draw difficulty selection menu
                draw_difficulty_menu(selected_menu);
                
                // Draw instruction to start
                draw_bottom_centered_text("Press Enter to start", 48.0, YELLOW);
                
                // Menu navigation logic
                if is_key_pressed(KeyCode::Up) && selected_menu > 0 {
                    selected_menu -= 1;
                }
                if is_key_pressed(KeyCode::Down) && selected_menu < 2 {
                    selected_menu += 1;
                }
                if is_key_pressed(KeyCode::Enter) {
                    difficulty = difficulties[selected_menu];
                    fall_delay = difficulty.fall_delay();
                    game_state = GameState::Running;
                    level = 1;
                    level_timer = 0.0;
                    game_info.set_level(level); // reset level in game info
                }
            }
            GameState::Running => {
                // ---------------------------------------------
                // Level progression logic
                // ---------------------------------------------
                level_timer += get_frame_time();
                if level_timer > 30.0 {
                    level_timer = 0.0;
                    level += 1;
                    fall_delay = (fall_delay * 0.9).max(0.1);
                    game_info.set_level(level); // update level in game info
                }

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
                            game_info.add_score(lines_cleared.try_into().unwrap());
                        }
                        // Spawn new tetromino using TetrominoBag
                        shape_idx = bag.next();
                        shape = SHAPES[shape_idx];
                        color = tetromino_colors[shape_idx];
                        next_idx = bag.peek();
                        next_shape = SHAPES[next_idx];
                        next_color = tetromino_colors[next_idx];
                        game_info.set_next(next_shape, next_color);
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
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::X) {
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

                // Pause logic moved here
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::P) {
                    game_state = GameState::Paused;
                }

                // Draw grid lines
                draw_grid(GRAY);
                // Draw stacked blocks
                draw_grid_blocks(&grid);
                // Draw active tetromino
                draw_tetromino(&shape, grid_x, grid_y, color);
                // Draw game info panel (score, lines, next)
                game_info.draw();
            }
            GameState::Paused => {
                // Draw the grid and stacked blocks as usual
                draw_grid(GRAY);
                draw_grid_blocks(&grid);

                // Draw a semi-transparent overlay to "blur" or dim the grid
                draw_rectangle(
                    0.0,
                    0.0,
                    GRID_WIDTH as f32 * BLOCK_SIZE,
                    GRID_HEIGHT as f32 * BLOCK_SIZE,
                    Color::new(0.2, 0.2, 0.2, 0.7),
                );

                // Draw the info panel as usual
                game_info.draw();

                // Draw "Paused" text in the center
                draw_centered_text("Paused", 60.0, YELLOW);
                
                // Unpause logic moved here
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::P) {
                    game_state = GameState::Running;
                }
            }
            GameState::GameOver => {
                // Draw the grid and stacked blocks as usual
                draw_grid(GRAY);
                draw_grid_blocks(&grid);

                // Draw a semi-transparent overlay to "blur" or dim the grid
                draw_rectangle(
                    0.0,
                    0.0,
                    GRID_WIDTH as f32 * BLOCK_SIZE,
                    GRID_HEIGHT as f32 * BLOCK_SIZE,
                    Color::new(0.2, 0.2, 0.2, 0.7),
                );

                // Draw the info panel as usual
                game_info.draw();

                // Draw "Game Over" text in the center
                draw_centered_text("Game Over", 60.0, RED);
            }
        }

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }
        next_frame().await
    }
}

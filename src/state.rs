// This module defines the GameState enum and related logic for managing the game's state.
//
// Principles and design choices:
//
// - The GameState enum represents all possible states of the game: Waiting (menu), Running (active play), Paused, and GameOver.
// - Provides convenience methods for checking the current state, improving code readability and maintainability.
// - Designed for clear and robust state management throughout the game loop.

/// Enum representing the different states of the game.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Waiting,
    Running,
    Paused,
    GameOver,
}

/// Implementation of GameState methods.
#[allow(dead_code)]
impl GameState {
    pub fn is_running(&self) -> bool {
        matches!(self, GameState::Running)
    }
    pub fn is_waiting(&self) -> bool {
        matches!(self, GameState::Waiting)
    }
}

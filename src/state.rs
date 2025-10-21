#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Running,
    Paused,
    GameOver,
}

impl GameState {
    pub fn is_running(&self) -> bool {
        matches!(self, GameState::Running)
    }
}

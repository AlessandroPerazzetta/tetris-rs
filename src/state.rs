#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Waiting,
    Running,
    Paused,
    GameOver,
}

#[allow(dead_code)]
impl GameState {
    pub fn is_running(&self) -> bool {
        matches!(self, GameState::Running)
    }
    pub fn is_waiting(&self) -> bool {
        matches!(self, GameState::Waiting)
    }
}

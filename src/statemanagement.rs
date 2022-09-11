#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Loading,
    GamePlaying,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauseState {
    Paused,
    UnPaused,
}

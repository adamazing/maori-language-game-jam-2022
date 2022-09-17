#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Loading,
    GameIntro,
    GamePlaying,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauseState {
    Paused,
    UnPaused,
}

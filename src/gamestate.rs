use std::time::Duration;

pub trait Update {
    fn update(&mut self, dt: Duration, game_state: &mut GameState);
}

pub type Score = u32;

#[derive(Debug, Copy, Clone, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing(Score),
    GameOver,
}

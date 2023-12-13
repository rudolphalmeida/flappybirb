use std::time::Duration;

pub trait Update {
    fn update(&mut self, dt: Duration, game_state: &mut GameState);
}

pub type Score = u32;

#[derive(Debug, Copy, Clone, Default)]
pub enum PlayState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct GameState {
    pub state: PlayState,
    pub score: Score,
    pub viewport_size: (u32, u32),
    pub fly_up: bool,
}

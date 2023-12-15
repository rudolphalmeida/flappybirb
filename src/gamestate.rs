use std::time::Duration;

use nalgebra as na;

pub trait Update {
    fn update(&mut self, dt: Duration, game_state: &mut GameState);
}

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub position: na::Vector2<f32>,
    pub size: na::Vector2<f32>,
}

impl BoundingBox {
    pub fn intersect(&self, other: &BoundingBox) -> bool {
        /// if (X1+W1<X2 or X2+W2<X1 or Y1+H1<Y2 or Y2+H2<Y1):
        //     Intersection = Empty
        // else:
        //     Intersection = Not Empty
        if (self.position.x + self.size.x < other.position.x) ||
            (other.position.x + other.size.x < self.position.x) ||
            (self.position.y + self.size.y < other.position.y) ||
            (other.position.y + other.size.y < self.position.y) {
            false
        } else {
            true
        }
    }
}

pub trait Hittable {
    fn bounding_boxes(&self, game_state: &GameState) -> Vec<BoundingBox>;
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

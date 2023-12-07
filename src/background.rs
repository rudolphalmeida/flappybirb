use std::time::Duration;
use glium::{Display, Frame, Surface};
use glium::glutin::surface::WindowSurface;
use crate::renderer::{Render, SpriteRenderer};
use nalgebra as na;
use crate::gamestate::{GameState, Update};

use crate::texture::Texture;

pub struct Background {
    texture: Texture,
    offset: f32,
    speed: f32,
}

impl Background {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let texture = Texture::from_bytes(include_bytes!("../assets/sprites/background-day.png"), display);
        Self {
            texture,
            offset: 0.0,
            speed: 0.05,
        }
    }
}

impl Render for Background {
    fn render(&self, frame: &mut Frame, renderer: &mut SpriteRenderer, _game_state: &GameState) {
        let size = frame.get_dimensions();
        let pan = na::Vector2::new(self.offset, 0.0);
        renderer.render(frame, self.texture.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest), na::Vector2::new(0.0, 0.0), na::Vector2::new(size.0 as f32, size.1 as f32), 0.0, pan);
    }
}

impl Update for Background {
    fn update(&mut self, dt: Duration, game_state: &mut GameState) {
        if matches!(game_state, GameState::Playing(_)) {
            self.offset += dt.as_secs_f32() * self.speed;
        }
    }
}

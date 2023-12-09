use crate::gamestate::{GameState, PlayState, Update};
use crate::renderer::{Render, RenderOptions, SpriteRenderer};
use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame, Surface};
use nalgebra as na;
use std::time::Duration;

use crate::texture::Texture;

pub struct Background {
    texture: Texture,
    offset: f32,
    speed: f32,
}

impl Background {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let texture = Texture::from_bytes(
            include_bytes!("../assets/sprites/background-night.png"),
            display,
        );
        Self {
            texture,
            offset: 0.0,
            speed: 0.05,
        }
    }
}

impl Render for Background {
    fn render(&self, frame: &mut Frame, renderer: &SpriteRenderer, _game_state: &GameState) {
        let size = frame.get_dimensions();
        let pan = na::Vector2::new(self.offset, 0.0);
        renderer.render(
            frame,
            &self.texture,
            RenderOptions {
                size: na::Vector2::new(size.0 as f32, size.1 as f32),
                pan,
                ..RenderOptions::default()
            },
        );
    }
}

impl Update for Background {
    fn update(&mut self, dt: Duration, game_state: &mut GameState) {
        if matches!(game_state.state, PlayState::Playing) {
            self.offset += dt.as_secs_f32() * self.speed;
        }
    }
}

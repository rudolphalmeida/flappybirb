use crate::gamestate::{GameState, PlayState, Update};
use crate::renderer::{Render, RenderOptions, SpriteRenderer};
use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame, Surface};
use nalgebra as na;
use std::time::Duration;

use crate::texture::Texture;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextureVariant {
    Day,
    Night,
}

pub struct Background {
    night_texture: Texture,
    day_texture: Texture,
    offset: f32,
    speed: f32,
    pub texture_variant: TextureVariant,
}

impl Background {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let night_texture = Texture::from_bytes(
            include_bytes!("../assets/sprites/background-night.png"),
            display,
        );
        let day_texture = Texture::from_bytes(
            include_bytes!("../assets/sprites/background-day.png"),
            display,
        );

        Self {
            night_texture,
            day_texture,
            offset: 0.0,
            speed: 0.085,
            texture_variant: TextureVariant::Night,
        }
    }
}

impl Render for Background {
    fn render(&self, frame: &mut Frame, renderer: &SpriteRenderer, _game_state: &GameState) {
        let size = frame.get_dimensions();
        let pan = na::Vector2::new(self.offset, 0.0);
        renderer.render(
            frame,
            if self.texture_variant == TextureVariant::Night {
                &self.night_texture
            } else {
                &self.day_texture
            },
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

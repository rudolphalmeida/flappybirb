use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame, Surface};
use nalgebra_glm as glm;
use std::time::Duration;

use crate::gamestate::{BoundingBox, GameState, Hittable, PlayState, Update};
use crate::renderer::{Render, RenderOptions, SpriteRenderer};
use crate::texture::Texture;

pub struct Ground {
    texture: Texture,
    offset: f32,
    speed: f32,
}

impl Ground {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let texture = Texture::from_bytes(include_bytes!("../assets/sprites/base.png"), display);
        Self {
            texture,
            offset: 0.0,
            speed: 0.15,
        }
    }
}

impl Render for Ground {
    fn render(&self, frame: &mut Frame, renderer: &SpriteRenderer, game_state: &GameState) {
        let pan = glm::vec2(self.offset, 0.0);
        let BoundingBox { position, size } = self.bounding_boxes(game_state)[0];
        renderer.render(
            frame,
            &self.texture,
            RenderOptions {
                position,
                size,
                pan,
                ..RenderOptions::default()
            },
        );
    }
}

impl Update for Ground {
    fn update(&mut self, dt: Duration, game_state: &mut GameState) {
        if matches!(game_state.state, PlayState::Playing) {
            self.offset += dt.as_secs_f32() * self.speed;
        }
    }
}

impl Hittable for Ground {
    fn bounding_boxes(&self, game_state: &GameState) -> Vec<BoundingBox> {
        // Cover bottom 20% of window
        let viewport_size = game_state.viewport_size;
        let size = glm::vec2(viewport_size.0 as f32, viewport_size.1 as f32 * 0.20);
        let position = glm::vec2(0.0, viewport_size.1 as f32 * 0.80);
        vec![BoundingBox {position, size}]
    }
}

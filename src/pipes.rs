use std::time::Duration;

use glium::{Display, Frame, Surface};
use glium::glutin::surface::WindowSurface;
use nalgebra_glm as glm;

use crate::gamestate::{GameState, PlayState, Update};
use crate::renderer::{Render, SpriteRenderer};
use crate::texture::Texture;

const PIPE_APERTURE_PERCENT: f32 = 0.15;  // px
const PIPE_GAP_PERCENT: f32 = 0.20; // px

pub struct Pipes {
    texture: Texture,
    left_pipe_offset: f32,
    speed: f32,
}

impl Pipes {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let texture = Texture::from_bytes(include_bytes!("../assets/sprites/pipe-green.png"), display);
        let width = display.get_framebuffer_dimensions().0 as f32;
        let left_pipe_offset = width * 0.50;

        Self {
            texture,
            left_pipe_offset,
            speed: 0.1,
        }
    }
}

impl Render for Pipes {
    fn render(&self, frame: &mut Frame, renderer: &SpriteRenderer, game_state: &GameState) {
        if matches!(game_state.state, PlayState::Playing) {
            let (width, height) = frame.get_dimensions();
            let (width, height) = (width as f32, height as f32);
            let mut pipe_offset = self.left_pipe_offset;
            while pipe_offset < width {
                // Top pipe
                let rotation = 180.0;
                let position = glm::vec2(pipe_offset, 0.0);
                let size = glm::vec2(width * 0.10, height * (0.50 - PIPE_APERTURE_PERCENT / 2.0));
                renderer.render(frame, self.texture.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest), position, size, rotation, Default::default());

                // Bottom pipe
                let rotation = 0.0;
                let position = glm::vec2(pipe_offset, height * (0.50 + PIPE_APERTURE_PERCENT / 2.0));
                let size = glm::vec2(width * 0.10, height * (0.50 - PIPE_APERTURE_PERCENT / 2.0));
                renderer.render(frame, self.texture.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest), position, size, rotation, Default::default());

                pipe_offset += width * 0.10 + width * PIPE_GAP_PERCENT;
            }
        }
    }
}

impl Update for Pipes {
    fn update(&mut self, dt: Duration, game_state: &mut GameState) {
        if matches!(game_state.state, PlayState::Playing) {
            let width = game_state.viewport_size.0 as f32;
            self.left_pipe_offset -= (dt.as_secs_f32() * self.speed) * width;
            if self.left_pipe_offset < -(width * 0.10) {
                self.left_pipe_offset += width * 0.10 + width * PIPE_GAP_PERCENT;
            }
        }
    }
}
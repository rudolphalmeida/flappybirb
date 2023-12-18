use std::time::Duration;

use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame, Surface};
use nalgebra_glm as glm;

use crate::gamestate::{BoundingBox, GameState, Hittable, PlayState, Update};
use crate::renderer::{Render, RenderOptions, SpriteRenderer};
use crate::texture::Texture;

const PIPE_APERTURE_PERCENT: f32 = 0.15;
const PIPE_GAP_PERCENT: f32 = 0.20;

pub struct Pipes {
    texture: Texture,
    left_pipe_offset: f32,
    speed: f32,
}

impl Pipes {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let texture =
            Texture::from_bytes(include_bytes!("../assets/sprites/pipe-green.png"), display);
        let width = display.get_framebuffer_dimensions().0 as f32;
        let left_pipe_offset = width * 0.50;

        Self {
            texture,
            left_pipe_offset,
            speed: 0.15,
        }
    }

    pub fn reset(&mut self, game_state: &GameState) {
        self.left_pipe_offset = game_state.viewport_size.0 as f32 * 0.50;
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
                let position = glm::vec2(pipe_offset, 0.0);
                let size = glm::vec2(width * 0.10, height * (0.50 - PIPE_APERTURE_PERCENT / 2.0));
                renderer.render(
                    frame,
                    &self.texture,
                    RenderOptions {
                        position,
                        size,
                        flip_vertical: true,
                        ..RenderOptions::default()
                    },
                );

                // Bottom pipe
                let position =
                    glm::vec2(pipe_offset, height * (0.50 + PIPE_APERTURE_PERCENT / 2.0));
                let size = glm::vec2(width * 0.10, height * (0.50 - PIPE_APERTURE_PERCENT / 2.0));
                renderer.render(
                    frame,
                    &self.texture,
                    RenderOptions {
                        position,
                        size,
                        ..RenderOptions::default()
                    },
                );

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

impl Hittable for Pipes {
    fn bounding_boxes(&self, game_state: &GameState) -> Vec<BoundingBox> {
        let (width, height) = game_state.viewport_size;
        let (width, height) = (width as f32, height as f32);

        let mut bounding_boxes = Vec::new();

        let mut pipe_offset = self.left_pipe_offset;

        while pipe_offset < width {
            // Top pipe
            let position = glm::vec2(pipe_offset, 0.0);
            let size = glm::vec2(width * 0.10, height * (0.50 - PIPE_APERTURE_PERCENT / 2.0));
            bounding_boxes.push(BoundingBox { position, size });

            // Bottom pipe
            let position = glm::vec2(pipe_offset, height * (0.50 + PIPE_APERTURE_PERCENT / 2.0));
            let size = glm::vec2(width * 0.10, height * (0.50 - PIPE_APERTURE_PERCENT / 2.0));
            bounding_boxes.push(BoundingBox { position, size });

            pipe_offset += width * 0.10 + width * PIPE_GAP_PERCENT;
        }

        bounding_boxes
    }
}

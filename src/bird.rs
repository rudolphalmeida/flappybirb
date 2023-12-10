use crate::gamestate::{GameState, PlayState, Update};
use crate::renderer::{Render, RenderOptions, SpriteRenderer};
use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame};
use nalgebra as na;
use std::time::Duration;

use crate::texture::Texture;

const MAX_FLAP_DURATION: f32 = 0.5;

#[derive(Debug, Copy, Clone)]
enum Flap {
    Down = 0,
    Mid = 1,
    Up = 2,
}

const FLAP_CYCLE: [Flap; 4] = [Flap::Down, Flap::Mid, Flap::Up, Flap::Mid];

pub struct Bird {
    textures: [Texture; 3],
    position: na::Vector2<f32>,
    velocity: na::Vector2<f32>,
    flap: usize,
    flap_duration: Duration,
}

impl Bird {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let textures = [
            Texture::from_bytes(
                include_bytes!("../assets/sprites/bluebird-downflap.png"),
                display,
            ),
            Texture::from_bytes(
                include_bytes!("../assets/sprites/bluebird-midflap.png"),
                display,
            ),
            Texture::from_bytes(
                include_bytes!("../assets/sprites/bluebird-upflap.png"),
                display,
            ),
        ];

        let (width, height) = display.get_framebuffer_dimensions();
        let position = na::Vector2::new(width as f32 * 0.25, height as f32 * 0.50);
        let velocity = na::Vector2::new(1.0, 0.0);
        let flap = 0;
        let flap_duration = Duration::from_secs_f32(0.0);

        Self {
            textures,
            position,
            velocity,
            flap,
            flap_duration,
        }
    }
}

impl Render for Bird {
    fn render(&self, frame: &mut Frame, renderer: &SpriteRenderer, game_state: &GameState) {
        if matches!(game_state.state, PlayState::Playing) {
            let (width, height) = self.textures[FLAP_CYCLE[self.flap] as usize].size;
            let size = na::Vector2::new(width as f32, height as f32) * 1.5;
            renderer.render(
                frame,
                &self.textures[FLAP_CYCLE[self.flap] as usize],
                RenderOptions {
                    position: self.position,
                    size,
                    ..RenderOptions::default()
                },
            );
        }
    }
}

impl Update for Bird {
    fn update(&mut self, dt: Duration, game_state: &mut GameState) {
        if matches!(game_state.state, PlayState::Playing) {
            self.flap_duration += dt;
            if self.flap_duration >= Duration::from_secs_f32(MAX_FLAP_DURATION) {
                self.flap = (self.flap + 1) % 4;
                self.flap_duration -= Duration::from_secs_f32(MAX_FLAP_DURATION);
            }
        }
    }
}

use std::time::Duration;

use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame};
use nalgebra as na;
use nalgebra_glm as glm;

use crate::gamestate::{BoundingBox, GameState, Hittable, PlayState, Update};
use crate::renderer::{Render, RenderOptions, SpriteRenderer};
use crate::texture::Texture;

const MAX_FLAP_DURATION: f32 = 0.25;
const DEFAULT_GRAVITY: f32 = 600.0;
const UPWARDS_FORCE: f32 = 300.0;

#[derive(Debug, Copy, Clone)]
enum Flap {
    Down = 0,
    Mid = 1,
    Up = 2,
}

const FLAP_CYCLE: [Flap; 4] = [Flap::Down, Flap::Mid, Flap::Up, Flap::Mid];

pub struct Bird {
    textures: [Texture; 3],
    pub y_position: f32,
    pub y_velocity: f32,
    flap_index: usize,
    flap_duration: Duration,
    rotation: f32,

    pub gravity: f32,
    pub upwards_force: f32,
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

        let (_width, height) = display.get_framebuffer_dimensions();
        let y_position = height as f32 * 0.50;
        let y_velocity = 0.0;
        let flap = 0;
        let flap_duration = Duration::from_secs_f32(0.0);
        let rotation = 0.0;

        let gravity = DEFAULT_GRAVITY;
        let upwards_force = UPWARDS_FORCE;

        Self {
            textures,
            y_position,
            y_velocity,
            flap_index: flap,
            flap_duration,
            rotation,
            gravity,
            upwards_force,
        }
    }

    pub fn reset(&mut self, game_state: &GameState) {
        let height = game_state.viewport_size.1 as f32;
        self.y_position = height * 0.50;
        self.y_velocity = 0.0;
        self.flap_index = 0;
        self.flap_duration = Duration::from_secs_f32(0.0);
        self.rotation = 0.0;
    }
}

impl Render for Bird {
    fn render(&self, frame: &mut Frame, renderer: &SpriteRenderer, game_state: &GameState) {
        if matches!(game_state.state, PlayState::Playing)
            || matches!(game_state.state, PlayState::GameOver)
        {
            let BoundingBox { position, size } = self.bounding_boxes(game_state)[0];

            renderer.render(
                frame,
                &self.textures[FLAP_CYCLE[self.flap_index] as usize],
                RenderOptions {
                    position,
                    size,
                    rotation: self.rotation,
                    ..RenderOptions::default()
                },
            );
        }
    }
}

impl Update for Bird {
    fn update(&mut self, dt: Duration, game_state: &mut GameState) {
        if matches!(game_state.state, PlayState::Playing) {
            if game_state.fly_up {
                self.y_velocity = -self.upwards_force;
                self.rotation = -20.0;
            }

            self.y_velocity += self.gravity * dt.as_secs_f32();
            self.y_position += self.y_velocity * dt.as_secs_f32();
        }

        self.flap_duration += dt;
        if self.flap_duration >= Duration::from_secs_f32(MAX_FLAP_DURATION) {
            self.flap_index = (self.flap_index + 1) % 4;
            self.flap_duration -= Duration::from_secs_f32(MAX_FLAP_DURATION);
        }
    }
}

impl Hittable for Bird {
    fn bounding_boxes(&self, game_state: &GameState) -> Vec<BoundingBox> {
        let (width, height) = self.textures[FLAP_CYCLE[self.flap_index] as usize].size;
        let size = na::Vector2::new(width as f32, height as f32) * 1.5;
        let position = glm::vec2(game_state.viewport_size.0 as f32 * 0.25, self.y_position);

        vec![BoundingBox { position, size }]
    }
}

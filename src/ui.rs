use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame};
use nalgebra_glm as glm;

use crate::gamestate::{GameState, PlayState};
use crate::renderer::{Render, RenderOptions, SpriteRenderer};
use crate::texture::Texture;
use crate::util::{horizontally_centered_position, vertically_centered_position};

pub struct Ui {
    begin_texture: Texture,
    // score_textures: [Texture; 10],
    gameover_texture: Texture,
}

impl Ui {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let begin_texture =
            Texture::from_bytes(include_bytes!("../assets/sprites/message.png"), display);
        let gameover_texture =
            Texture::from_bytes(include_bytes!("../assets/sprites/gameover.png"), display);

        Self {
            begin_texture,
            gameover_texture,
        }
    }
}

impl Render for Ui {
    fn render(&self, frame: &mut Frame, renderer: &SpriteRenderer, game_state: &GameState) {
        let viewport_size = game_state.viewport_size;
        let viewport_size = glm::vec2(viewport_size.0 as f32, viewport_size.1 as f32);
        match game_state.state {
            PlayState::MainMenu => {
                let texture_size = self.begin_texture.size;
                let size = glm::vec2(texture_size.0 as f32, texture_size.1 as f32) * 2.0;
                let position = glm::vec2(
                    vertically_centered_position(viewport_size, size),
                    horizontally_centered_position(viewport_size, size),
                );

                renderer.render(
                    frame,
                    &self.begin_texture,
                    RenderOptions {
                        position,
                        size,
                        ..RenderOptions::default()
                    },
                );
            }
            PlayState::Playing => {}
            PlayState::GameOver => {
                let texture_size = self.gameover_texture.size;
                let size = glm::vec2(texture_size.0 as f32, texture_size.1 as f32) * 2.0;
                let position = glm::vec2(
                    vertically_centered_position(viewport_size, size),
                    horizontally_centered_position(viewport_size, size),
                );

                renderer.render(
                    frame,
                    &self.gameover_texture,
                    RenderOptions {
                        position,
                        size,
                        ..RenderOptions::default()
                    },
                );
            }
        };
    }
}

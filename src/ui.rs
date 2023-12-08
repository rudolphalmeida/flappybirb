use glium::{Display, Frame, Surface};
use glium::glutin::surface::WindowSurface;
use nalgebra_glm as glm;

use crate::gamestate::{GameState, PlayState};
use crate::renderer::{Render, SpriteRenderer};
use crate::texture::Texture;

pub struct Ui {
    begin_texture: Texture,
    // score_textures: [Texture; 10],
    // gameover_texture: Texture,
}

impl Ui {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let begin_texture = Texture::from_bytes(include_bytes!("../assets/sprites/message.png"), display);
        // let gameover_texture = Texture::from_bytes(include_bytes!("../assets/sprites/background-day.png"), display);

        Self {
            begin_texture
        }
    }
}

impl Render for Ui {
    fn render(&self, frame: &mut Frame, renderer: &SpriteRenderer, game_state: &GameState) {
        match game_state.state {
            PlayState::MainMenu => {
                let size = frame.get_dimensions();
                let message_size = (size.0 as f32 / 2.0, size.1 as f32 / 2.0);
                let message_position = (size.0 as f32 / 4.0, size.1 as f32 / 4.0);
                renderer.render(frame, self.begin_texture.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest), glm::vec2(message_position.0, message_position.1), glm::vec2(message_size.0, message_size.1), 0.0, Default::default());
            }
            PlayState::Playing => {}
            PlayState::GameOver => {}
        }
    }
}

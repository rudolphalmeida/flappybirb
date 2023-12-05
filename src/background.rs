use glium::{Display, Frame};
use glium::glutin::surface::WindowSurface;
use crate::renderer::{Renderable, SpriteRenderer};

use crate::texture::Texture;

pub struct Background {
    pub texture: Texture,
}

impl Background {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let texture = Texture::from_bytes(include_bytes!("../assets/sprites/background-day.png"), display);
        Self {
            texture
        }
    }
}

impl Renderable for Background {
    fn render(&self, frame: &mut Frame, renderer: &mut SpriteRenderer) {
        renderer.render(frame, &self.texture, Default::default(), Default::default(), Default::default());
    }
}

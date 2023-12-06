use glium::{Display, Frame, Surface};
use glium::glutin::surface::WindowSurface;
use crate::renderer::{Renderable, SpriteRenderer};
use nalgebra as na;

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
        let size = frame.get_dimensions();
        renderer.render(frame, self.texture.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest), na::Vector2::new(0.0, 0.0), na::Vector2::new(size.0 as f32, size.1 as f32), 0.0);
    }
}

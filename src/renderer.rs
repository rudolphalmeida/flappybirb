use glium::{Display, Frame, Program, Surface, uniform, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use nalgebra as na;
use nalgebra::Rotation2;
use crate::shader::load_shader;

use crate::texture::Texture;
use crate::vertex::Vertex;

pub trait Renderable {
    fn render(&self, frame: &mut Frame, renderer: &mut SpriteRenderer);
}

pub struct SpriteRenderer {
    view: na::Orthographic3<f32>,
    shader_program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
}

impl SpriteRenderer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let shader_program = load_shader(display);
        let (width, height) = display.get_framebuffer_dimensions();
        let view = na::Orthographic3::new(0.0, width as f32, 0.0, height as f32, -1.0, 1.0);

        let shape = Vertex::rectangle();
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        Self {
            shader_program, view, vertex_buffer
        }
    }

    pub fn viewport_resized(&mut self, (width, height): (u32, u32)) {
        self.view = na::Orthographic3::new(0.0, width as f32, 0.0, height as f32, -1.0, 1.0);
    }

    pub fn render(&self, frame: &mut Frame, texture: &Texture, position: na::Vector2<f32>, size: na::Vector2<f32>, rotation: Rotation2<f32>) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let uniforms = uniform! { sprite: texture.texture_id() };
        frame.draw(&self.vertex_buffer, indices, &self.shader_program, &uniforms, &Default::default()).unwrap();
    }
}

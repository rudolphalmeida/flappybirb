use glium::{Display, Frame, Program, Surface, uniform, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::texture::SrgbTexture2d;
use glium::uniforms::Sampler;
use nalgebra as na;
use nalgebra::RealField;
use nalgebra_glm as glm;
use crate::gamestate::GameState;
use crate::shader::load_shader;
use crate::vertex::Vertex;

pub trait Render {
    fn render(&self, frame: &mut Frame, renderer: &SpriteRenderer, game_state: &GameState);
}

pub struct SpriteRenderer {
    view: na::Matrix4<f32>,
    shader_program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
}

impl SpriteRenderer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let shader_program = load_shader(display);
        let (width, height) = display.get_framebuffer_dimensions();
        let view = glm::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

        let shape = Vertex::sprite_rectangle();
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        Self {
            shader_program, view, vertex_buffer
        }
    }

    pub fn viewport_resized(&mut self, (width, height): (u32, u32)) {
        log::debug!("Viewport resized to ({width:}, {height:})");
        self.view = glm::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
    }

    pub fn render(&self, frame: &mut Frame, texture: Sampler<'_, SrgbTexture2d>, position: na::Vector2<f32>, size: na::Vector2<f32>, rotation: f32, pan: na::Vector2<f32>) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let mut model = glm::identity::<f32, 4>();
        model = glm::translate(&model, &glm::vec3(position.x, position.y, 0.0));

        model = glm::translate(&model, &glm::vec3(0.5 * size.x, 0.5 * size.y, 0.0));
        model = glm::rotate(&model, rotation * f32::pi() / 180.0, &glm::vec3(0.0, 0.0, 1.0));
        model = glm::translate(&model, &glm::vec3(-0.5 * size.x, -0.5 * size.y, 0.0));

        let model = glm::scale(&model, &glm::vec3(size.x, size.y, 1.0));
        let model_ref = model.as_ref();

        let projection_ref = self.view.as_ref();

        let uniforms = uniform! { sprite: texture, model: *model_ref, projection: *projection_ref, pan: *pan.as_ref() };
        frame.draw(&self.vertex_buffer, indices, &self.shader_program, &uniforms, &Default::default()).unwrap();
    }
}

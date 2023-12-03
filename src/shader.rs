use glium::Display;
use glium::glutin::surface::WindowSurface;

const VERTEX_SHADER_SOURCE: &str = include_str!("../sprite_vertex.vert");
const FRAGMENT_SHADER_SOURCE: &str = include_str!("../sprite_fragment.frag");

pub fn load_shader(display: &Display<WindowSurface>) -> glium::Program {
    glium::Program::from_source(display, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None).unwrap()
}
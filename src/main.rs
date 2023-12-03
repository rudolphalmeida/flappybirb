use glium::{Surface, uniform};
use glium::uniforms::SamplerWrapFunction;
use winit::event::{Event, WindowEvent};
use crate::background::Background;
use crate::shader::load_shader;
use crate::vertex::Vertex;

mod vertex;
mod shader;
mod background;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_inner_size(500, 500).with_title("Flappy Birb").build(&event_loop);

    let shader_program = load_shader(&display);

    let shape = Vertex::rectangle();
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let background = Background::new(&display);

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::NewEvents(_) => {}
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::KeyboardInput { .. } => {}
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let uniforms = uniform! {tex: background.texture.sampled().wrap_function(SamplerWrapFunction::Repeat)};

                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
                frame.draw(&vertex_buffer, indices, &shader_program, &uniforms, &Default::default()).unwrap();
                frame.finish().unwrap();
            }
            Event::RedrawEventsCleared => window.request_redraw(),
            _ => {}
        }
    });
}

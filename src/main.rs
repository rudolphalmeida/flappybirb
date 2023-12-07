use std::time::Instant;
use glium::Surface;
use winit::event::{Event, WindowEvent};
use crate::background::Background;
use crate::gamestate::Update;
use crate::renderer::{Render, SpriteRenderer};

mod background;
mod gamestate;
mod renderer;
mod shader;
mod texture;
mod vertex;

fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_inner_size(500, 500).with_title("Flappy Birb").build(&event_loop);

    let mut sprite_renderer = SpriteRenderer::new(&display);

    let mut background = Background::new(&display);

    let mut previous_frame_time = Instant::now();

    event_loop.run(move |ev, _, control_flow| {
        let frame_time = Instant::now();
        let dt = frame_time - previous_frame_time;
        previous_frame_time = frame_time;

        background.update(dt);

        match ev {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::KeyboardInput { .. } => {}
                    WindowEvent::Resized(size) => sprite_renderer.viewport_resized((size.width, size.height)),
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
                background.render(&mut frame, &mut sprite_renderer);
                frame.finish().unwrap();
            }
            Event::RedrawEventsCleared => window.request_redraw(),
            _ => {}
        }
    });
}

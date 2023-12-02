use glium::Surface;
use winit::event::{Event, WindowEvent};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_inner_size(500, 500).with_title("Flappy Birb").build(&event_loop);

    let mut surface = display.draw();
    surface.clear_color(0.0, 0.0, 1.0, 1.0);
    surface.finish().unwrap();

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
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => window.request_redraw(),
            _ => {}
        }
    });
}

use std::time::Instant;
use glium::Surface;
use winit::event::{Event, KeyboardInput, ScanCode, VirtualKeyCode, WindowEvent};
use crate::background::Background;
use crate::gamestate::{GameState, Update};
use crate::ground::Ground;
use crate::renderer::{Render, SpriteRenderer};
use crate::ui::Ui;

mod background;
mod gamestate;
mod ground;
mod renderer;
mod shader;
mod texture;
mod ui;
mod vertex;

fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_inner_size(500, 500).with_title("Flappy Birb").build(&event_loop);

    let mut sprite_renderer = SpriteRenderer::new(&display);

    let mut game_state = GameState::default();

    let mut background = Background::new(&display);
    let mut ground = Ground::new(&display);
    let ui = Ui::new(&display);

    let mut previous_frame_time = Instant::now();

    event_loop.run(move |ev, _, control_flow| {
        let frame_time = Instant::now();
        let dt = frame_time - previous_frame_time;
        previous_frame_time = frame_time;

        background.update(dt, &mut game_state);
        ground.update(dt, &mut game_state);

        match ev {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::KeyboardInput { input, .. } => {
                        match game_state {
                            GameState::Playing(_) => {}
                            _ if input.virtual_keycode == Some(VirtualKeyCode::Space) => game_state = GameState::Playing(0),
                            _ => {}
                        }
                    }
                    WindowEvent::Resized(size) => sprite_renderer.viewport_resized((size.width, size.height)),
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 1.0);

                background.render(&mut frame, &mut sprite_renderer, &game_state);
                ground.render(&mut frame, &mut sprite_renderer, &game_state);
                ui.render(&mut frame, &mut sprite_renderer, &game_state);

                frame.finish().unwrap();
            }
            Event::RedrawEventsCleared => window.request_redraw(),
            _ => {}
        }
    });
}

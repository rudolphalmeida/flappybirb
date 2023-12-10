use crate::background::Background;
use crate::bird::Bird;
use crate::gamestate::{GameState, PlayState, Update};
use crate::ground::Ground;
use crate::pipes::Pipes;
use crate::renderer::{Render, SpriteRenderer};
use crate::ui::Ui;
use glium::Surface;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::window::Icon;

mod background;
mod bird;
mod gamestate;
mod ground;
mod pipes;
mod renderer;
mod shader;
mod texture;
mod ui;
mod vertex;

fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new();

    let image = image::load(
        std::io::Cursor::new(include_bytes!("../assets/favicon.ico")),
        image::ImageFormat::Ico,
    )
    .unwrap()
    .to_rgba8();
    let size = image.dimensions();
    let icon = Icon::from_rgba(image.into_raw(), size.0, size.1).ok();
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(700.0, 970.0))
        .with_title("Flappy Birb")
        .with_resizable(false)
        .with_window_icon(icon);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_builder)
        .build(&event_loop);

    let mut sprite_renderer = SpriteRenderer::new(&display);

    let mut game_state = GameState::default();

    let mut background = Background::new(&display);
    let mut pipes = Pipes::new(&display);
    let mut ground = Ground::new(&display);
    let mut bird = Bird::new(&display);
    let ui = Ui::new(&display);

    let mut previous_frame_time = Instant::now();

    event_loop.run(move |ev, _, control_flow| {
        let frame_time = Instant::now();
        let dt = frame_time - previous_frame_time;
        previous_frame_time = frame_time;

        background.update(dt, &mut game_state);
        pipes.update(dt, &mut game_state);
        ground.update(dt, &mut game_state);
        bird.update(dt, &mut game_state);

        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput { input, .. } => match game_state.state {
                    PlayState::Playing => {}
                    _ if input.virtual_keycode == Some(VirtualKeyCode::Space) => {
                        game_state.state = PlayState::Playing
                    }
                    _ => {}
                },
                WindowEvent::Resized(size) => {
                    game_state.viewport_size = (size.width, size.height);
                    sprite_renderer.viewport_resized((size.width, size.height));
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 1.0);

                background.render(&mut frame, &sprite_renderer, &game_state);
                pipes.render(&mut frame, &sprite_renderer, &game_state);
                ground.render(&mut frame, &sprite_renderer, &game_state);
                bird.render(&mut frame, &sprite_renderer, &game_state);
                ui.render(&mut frame, &sprite_renderer, &game_state);

                frame.finish().unwrap();
            }
            Event::RedrawEventsCleared => window.request_redraw(),
            _ => {}
        }
    });
}

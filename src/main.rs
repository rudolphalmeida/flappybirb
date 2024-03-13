use std::time::Instant;

use glium::Surface;
use soloud::{audio, FromExt, Soloud};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::Icon;

use crate::background::{Background, TextureVariant};
use crate::bird::{Bird, BirdColor};
use crate::gamestate::{GameState, Hittable, PlayState, Update};
use crate::ground::Ground;
use crate::pipes::Pipes;
use crate::renderer::{Render, SpriteRenderer};
use crate::ui::Ui;

mod background;
mod bird;
mod gamestate;
mod ground;
mod pipes;
mod renderer;
mod shader;
mod texture;
mod ui;
mod util;
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

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &window, &event_loop);
    let mut show_debug = false;

    let sl = Soloud::default().unwrap();
    let hit = audio::Wav::from_mem(include_bytes!("../assets/audio/hit.wav")).unwrap();
    let _die = audio::Wav::from_mem(include_bytes!("../assets/audio/die.wav")).unwrap();
    let _swoosh = audio::Wav::from_mem(include_bytes!("../assets/audio/swoosh.wav")).unwrap();
    let wing = audio::Wav::from_mem(include_bytes!("../assets/audio/wing.wav")).unwrap();

    let mut sprite_renderer = SpriteRenderer::new(&display);

    let mut game_state = GameState::default();
    let mut hit_detection = true;

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

        let mut redraw = || {
            let repaint_after = egui_glium.run(&window, |ctx| {
                if !show_debug {
                    return;
                }

                egui::Window::new("Options")
                    .resizable(false)
                    .min_width(450.0)
                    .show(ctx, |ui| {
                        egui::ComboBox::from_label("Background style")
                            .selected_text(if background.texture_variant == TextureVariant::Night {
                                "Night"
                            } else {
                                "Day"
                            })
                            .show_ui(ui, |ui| {
                                if ui.button("Night").clicked() {
                                    background.texture_variant = TextureVariant::Night;
                                }
                                if ui.button("Day").clicked() {
                                    background.texture_variant = TextureVariant::Day;
                                }
                            });
                        ui.separator();
                        ui.checkbox(&mut hit_detection, "Hit Detection");

                        ui.separator();
                        ui.label("Gravity");
                        ui.add(egui::DragValue::new(&mut bird.gravity).speed(0.1));

                        ui.label("Upward Force");
                        ui.add(egui::DragValue::new(&mut bird.upwards_force).speed(0.1));

                        egui::ComboBox::from_label("Color")
                            .selected_text(match bird.color {
                                BirdColor::Blue => "Blue",
                                BirdColor::Red => "Red",
                                BirdColor::Yellow => "Yellow",
                            })
                            .show_ui(ui, |ui| {
                                if ui.button("Blue").clicked() {
                                    bird.color = BirdColor::Blue;
                                }
                                if ui.button("Red").clicked() {
                                    bird.color = BirdColor::Red;
                                }
                                if ui.button("Yellow").clicked() {
                                    bird.color = BirdColor::Yellow;
                                }
                            });

                        ui.separator();
                        if ui.button("Reset game").clicked() {
                            game_state.state = PlayState::MainMenu;
                            bird.reset(&game_state);
                            pipes.reset(&game_state);
                        }
                    });
            });

            if repaint_after.is_zero() {
                window.request_redraw();
                *control_flow = ControlFlow::Poll;
            };

            {
                let mut frame = display.draw();

                frame.clear_color(0.0, 0.0, 0.0, 1.0);

                background.render(&mut frame, &sprite_renderer, &game_state);
                pipes.render(&mut frame, &sprite_renderer, &game_state);
                ground.render(&mut frame, &sprite_renderer, &game_state);
                bird.render(&mut frame, &sprite_renderer, &game_state);
                ui.render(&mut frame, &sprite_renderer, &game_state);

                egui_glium.paint(&display, &mut frame);

                frame.finish().unwrap();
            }
        };

        match ev {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => control_flow.set_exit(),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                state,
                                ..
                            },
                        ..
                    } => match game_state.state {
                        PlayState::Playing if virtual_keycode == Some(VirtualKeyCode::Space) => {
                            game_state.fly_up = true;
                            if sl.voice_count() == 0 {
                                sl.play(&wing);
                            }
                        }
                        _ if virtual_keycode == Some(VirtualKeyCode::Space)
                            && state == ElementState::Released =>
                        {
                            bird.reset(&game_state);
                            pipes.reset(&game_state);
                            game_state.state = PlayState::Playing;
                        }
                        _ if virtual_keycode == Some(VirtualKeyCode::F5)
                            && state == ElementState::Released =>
                        {
                            show_debug = !show_debug;
                        }
                        _ => {}
                    },
                    WindowEvent::Resized(size) => {
                        game_state.viewport_size = (size.width, size.height);
                        sprite_renderer.viewport_resized((size.width, size.height));
                    }
                    _ => {}
                };

                let event_response = egui_glium.on_event(&event);
                if event_response.repaint {
                    window.request_redraw();
                }
            }
            Event::RedrawEventsCleared => redraw(),
            Event::RedrawRequested(_) => redraw(),
            _ => {}
        }

        background.update(dt, &mut game_state);
        pipes.update(dt, &mut game_state);
        ground.update(dt, &mut game_state);
        bird.update(dt, &mut game_state);

        if hit_detection {
            let bird_bb = bird.bounding_boxes(&game_state)[0];
            let ground_bb = ground.bounding_boxes(&game_state)[0];
            let pipe_intersect = pipes
                .bounding_boxes(&game_state)
                .iter()
                .any(|bb| bb.intersect(&bird_bb));

            if matches!(game_state.state, PlayState::Playing)
                && (bird_bb.intersect(&ground_bb) || pipe_intersect)
            {
                sl.play(&hit);
                game_state.state = PlayState::GameOver;
            }
        }

        game_state.fly_up = false;
    });
}

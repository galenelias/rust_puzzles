#![feature(c_variadic)]
#![deny(clippy::all)]
// #![forbid(unsafe_code)]

use crate::puzzle_wrap::{Frontend, Input, MouseButton};
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopBuilder;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod puzzle_wrap;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

#[derive(Debug)]
enum PuzzleEvents {
    RedrawRequested,
}

fn main() -> Result<(), Error> {
    env_logger::init();

    // let event_loop = EventLoop::new().unwrap();
    let event_loop = EventLoopBuilder::<PuzzleEvents>::with_user_event()
        .build()
        .unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rusty Mines")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut frontend = Frontend::new(WIDTH, HEIGHT);
    frontend.new_mines();
    frontend.new_game();
    frontend.set_size(WIDTH, HEIGHT);
    frontend.redraw();

    let event_loop_proxy = event_loop.create_proxy();
    frontend.set_end_draw_callback(move || {
        event_loop_proxy
            .send_event(PuzzleEvents::RedrawRequested)
            .unwrap();
    });

    let res = event_loop.run(|event, elwt| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
            }

            if input.mouse_pressed(0) {
                if let Some((x, y)) = input.cursor() {
                    frontend.process_input(&Input::MouseDown((MouseButton::Left, (x, y))));
                }
            }

            if input.mouse_released(0) {
                if let Some((x, y)) = input.cursor() {
                    frontend.process_input(&Input::MouseUp((MouseButton::Left, (x, y))));
                }
            }

            if input.mouse_pressed(1) {
                if let Some((x, y)) = input.cursor() {
                    frontend.process_input(&Input::MouseDown((MouseButton::Right, (x, y))));
                }
            }

            if input.mouse_released(1) {
                if let Some((x, y)) = input.cursor() {
                    frontend.process_input(&Input::MouseUp((MouseButton::Right, (x, y))));
                }
            }
        }

        match event {
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::RedrawRequested => {
                        for (dst, &src) in pixels
                            .frame_mut()
                            .chunks_exact_mut(4)
                            .zip(frontend.frame().iter())
                        {
                            dst[0] = (src >> 16) as u8;
                            dst[1] = (src >> 8) as u8;
                            dst[2] = src as u8;
                            dst[3] = (src >> 24) as u8;
                        }

                        if let Err(err) = pixels.render() {
                            log_error("pixels.render", err);
                            elwt.exit();
                            return;
                        }
                    }
                    _ => {
                        // println!("Other WindowEvent event: {:?}", event)
                    }
                };
            }
            Event::UserEvent(ref event) => match event {
                PuzzleEvents::RedrawRequested => {
                    window.request_redraw();
                }
            },
            _ => {
                // println!("Other event: {:?}", event)
            }
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

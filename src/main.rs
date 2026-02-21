#![feature(c_variadic)]
#![feature(c_size_t)]
#![deny(clippy::all)]
// #![forbid(unsafe_code)]

use crate::puzzle_wrap::{Frontend, Input, MouseButton};
use error_iter::ErrorIter as _;
use log::error;
use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{CheckMenuItem, CheckMenuItemBuilder, Menu, MenuItem, PredefinedMenuItem, Submenu};
use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::Arc;
use std::time;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{
    ElementState, KeyEvent, MouseButton as WinitMouseButton, StartCause, WindowEvent,
};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, KeyCode, PhysicalKey};
use winit::platform::macos::EventLoopBuilderExtMacOS;
use winit::window::{Window, WindowAttributes, WindowId};

mod puzzle_wrap;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

#[derive(Debug)]
enum PuzzleEvents {
    RedrawRequested,
    MenuEvent(muda::MenuEvent),
}

struct PuzzlePreset {
    name: String,
    menu_item: CheckMenuItem,
    id: i32,
}

struct AppMenu {
    menu_bar: Menu,
    _file_menu: Submenu,
    preset_items: Vec<PuzzlePreset>,
}

impl AppMenu {
    fn new(menu_bar: Menu, frontend: &Frontend) -> Self {
        #[cfg(target_os = "macos")]
        {
            let app_menu = Submenu::new("App", true);
            app_menu
                .append_items(&[
                    &PredefinedMenuItem::about(None, None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::services(None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::hide(None),
                    &PredefinedMenuItem::hide_others(None),
                    &PredefinedMenuItem::show_all(None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::quit(None),
                ])
                .unwrap();
            menu_bar.append(&app_menu).unwrap();
        }

        let file_menu = Submenu::new("&File", true);
        let edit_menu = Submenu::new("&Edit", true);
        let window_menu = Submenu::new("&Window", true);

        let type_menu = Submenu::new("&Type", true);
        let current_preset = frontend.which_preset();
        let presets = frontend.get_presets().unwrap();
        let mut preset_items = Vec::new();
        for (index, entry) in presets.entries().enumerate() {
            let preset_menu_item = CheckMenuItemBuilder::new()
                .text(entry.title())
                .enabled(true)
                .checked(Some(index as i32) == current_preset)
                .build();
            type_menu.append(&preset_menu_item).unwrap();
            preset_items.push(PuzzlePreset {
                name: entry.title().to_string(),
                id: entry.id(),
                menu_item: preset_menu_item,
            });
        }

        menu_bar
            .append_items(&[&file_menu, &edit_menu, &type_menu, &window_menu])
            .unwrap();

        Self {
            menu_bar,
            _file_menu: file_menu,
            preset_items,
        }
    }
}

struct App {
    window: Option<Arc<Window>>,
    app_menu: AppMenu,
    pixels: Option<Pixels<'static>>,
    frontend: Box<Frontend>,
    actual_width: u32,
    actual_height: u32,
    cursor_position: Option<(f32, f32)>,
    mouse_state: [bool; 2], // Left, Right
    control_held: bool,
}

impl App {
    fn new() -> Self {
        // Ensure the frontend is in a Box so that the pointers we pass to the midend FFI layer don't shift
        let mut frontend = Box::new(Frontend::new());
        frontend.new_midend();

        frontend.new_game();
        let (actual_width, actual_height) = frontend.set_size(WIDTH, HEIGHT);
        frontend.redraw();

        println!(
            "Actual width: {}, actual height: {}",
            actual_width, actual_height
        );

        let menu_bar = Menu::new();

        Self {
            window: None,
            app_menu: AppMenu::new(menu_bar, &frontend),
            pixels: None,
            frontend,
            actual_width,
            actual_height,
            cursor_position: None,
            mouse_state: [false, false],
            control_held: false,
        }
    }

    fn update_preset_menu(&mut self) {
        let current_preset = self.frontend.which_preset();
        for (index, entry) in &mut self.app_menu.preset_items.iter().enumerate() {
            entry
                .menu_item
                .set_checked(Some(index as i32) == current_preset);
        }
    }
}

impl ApplicationHandler<PuzzleEvents> for App {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        if cause == StartCause::Init {
            #[cfg(target_os = "macos")]
            {
                self.app_menu.menu_bar.init_for_nsapp();
            }
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let size = LogicalSize::new(self.actual_width as f64, self.actual_height as f64);
            let window_attributes = WindowAttributes::default()
                .with_title("Rusty Puzzles")
                .with_inner_size(size)
                .with_min_inner_size(size);

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            let pixels =
                Pixels::new(self.actual_width, self.actual_height, surface_texture).unwrap();

            window.request_redraw();

            self.window = Some(window);
            self.pixels = Some(pixels);
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: PuzzleEvents) {
        match event {
            PuzzleEvents::RedrawRequested => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            PuzzleEvents::MenuEvent(event) => {
                for preset in &self.app_menu.preset_items {
                    if event.id() == preset.menu_item.id() {
                        // TODO: Can we save off the preset easily to avoid refetching them all from the frontend?
                        let presets = self.frontend.get_presets().unwrap();
                        for entry in presets.entries() {
                            if entry.id() == preset.id {
                                self.frontend.load_preset(entry.get_entry());
                                self.frontend.new_game();

                                let (actual_width, actual_height) =
                                    self.frontend.set_size(WIDTH, HEIGHT);

                                let window = self.window.as_mut().unwrap();
                                let _ = window.request_inner_size(LogicalSize::new(
                                    actual_width,
                                    actual_height,
                                ));
                                let window_size = window.inner_size();

                                if let Some(pixels) = &mut self.pixels {
                                    let _ = pixels.resize_buffer(actual_width, actual_height);
                                    let _ = pixels
                                        .resize_surface(window_size.width, window_size.height);
                                }

                                self.frontend.redraw();
                                window.request_redraw();
                            }
                        }
                    }
                }
                self.update_preset_menu();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        physical_key,
                        state,
                        ..
                    },
                ..
            } => {
                // Track shift key state
                if let PhysicalKey::Code(KeyCode::ControlLeft | KeyCode::ControlRight) =
                    physical_key
                {
                    self.control_held = state == ElementState::Pressed;
                }

                if state != ElementState::Pressed {
                    return;
                }

                if let Key::Character(character) = key {
                    self.frontend
                        .process_input(&Input::KeyDown(puzzle_wrap::Key::Character(
                            character.chars().next().unwrap(),
                        )));
                }

                if let PhysicalKey::Code(keycode) = physical_key {
                    match keycode {
                        KeyCode::Escape => {
                            event_loop.exit();
                        }
                        KeyCode::ArrowLeft
                        | KeyCode::ArrowRight
                        | KeyCode::ArrowUp
                        | KeyCode::ArrowDown => {
                            self.frontend
                                .process_input(&Input::KeyDown(puzzle_wrap::Key::Special(keycode)));
                        }
                        _ => {}
                    }
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(pixels) = &mut self.pixels {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = Some((position.x as f32, position.y as f32));
            }
            WindowEvent::MouseInput { state, button, .. } => {
                // If control is held and left button is pressed, treat it as right button
                #[cfg(target_os = "macos")]
                let effective_button = if self.control_held && button == WinitMouseButton::Left {
                    WinitMouseButton::Right
                } else {
                    button
                };

                #[cfg(not(target_os = "macos"))]
                let effective_button = button;

                let button_idx = match effective_button {
                    WinitMouseButton::Left => 0,
                    WinitMouseButton::Right => 1,
                    _ => return,
                };

                let puzzle_button = match effective_button {
                    WinitMouseButton::Left => MouseButton::Left,
                    WinitMouseButton::Right => MouseButton::Right,
                    _ => return,
                };

                if let Some((x, y)) = self.cursor_position {
                    match state {
                        ElementState::Pressed => {
                            self.mouse_state[button_idx] = true;
                            self.frontend
                                .process_input(&Input::MouseDown((puzzle_button, (x, y))));
                        }
                        ElementState::Released => {
                            self.mouse_state[button_idx] = false;
                            self.frontend
                                .process_input(&Input::MouseUp((puzzle_button, (x, y))));
                        }
                    }
                }
            }
            WindowEvent::CursorLeft { .. } => {
                self.cursor_position = None;
            }
            WindowEvent::RedrawRequested => {
                if let Some(ref mut pixels) = self.pixels {
                    for (dst, &src) in pixels
                        .frame_mut()
                        .chunks_exact_mut(4)
                        .zip(self.frontend.frame().iter())
                    {
                        dst[0] = (src >> 16) as u8;
                        dst[1] = (src >> 8) as u8;
                        dst[2] = src as u8;
                        dst[3] = (src >> 24) as u8;
                    }

                    if let Err(err) = pixels.render() {
                        log_error("pixels.render", err);
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }

        // Handle mouse held events
        if let Some((x, y)) = self.cursor_position {
            if self.mouse_state[0] {
                self.frontend
                    .process_input(&Input::MouseHeld((MouseButton::Left, (x, y))));
            }
            if self.mouse_state[1] {
                self.frontend
                    .process_input(&Input::MouseHeld((MouseButton::Right, (x, y))));
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // If we have a timer active then we need to make sure we wake up after some interval
        // so we can drive the timer - otherwise we could get starved waiting for events
        if self.frontend.is_timer_active() {
            self.frontend.tick(); // Give a chance for timers to run.
            const WAIT_TIME: time::Duration = time::Duration::from_millis(10);
            event_loop.set_control_flow(ControlFlow::WaitUntil(time::Instant::now() + WAIT_TIME));
        } else {
            event_loop.set_control_flow(ControlFlow::Wait);
        }
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let event_loop = EventLoop::<PuzzleEvents>::with_user_event()
        .with_default_menu(false)
        .build()
        .unwrap();

    let mut app = App::new();

    let event_loop_proxy = event_loop.create_proxy();
    app.frontend.set_end_draw_callback(move || {
        event_loop_proxy
            .send_event(PuzzleEvents::RedrawRequested)
            .unwrap();
    });

    let proxy = event_loop.create_proxy();
    muda::MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(PuzzleEvents::MenuEvent(event));
    }));

    let res = event_loop.run_app(&mut app);
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

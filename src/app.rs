use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId, WindowLevel};

use crate::listener::GlobalInput;
use crate::sprites::SpriteSet;
use crate::state::{PetState, StateManager};

pub struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    sprites: SpriteSet,
    state: StateManager,
    cursor_inside: bool,
}

impl App {
    pub fn new(sprites: SpriteSet) -> Self {
        Self {
            window: None,
            surface: None,
            sprites,
            state: StateManager::default(),
            cursor_inside: false,
        }
    }
}

impl ApplicationHandler<GlobalInput> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title("desktop-cat")
            .with_inner_size(LogicalSize::new(128.0, 128.0))
            .with_decorations(false)
            .with_transparent(true)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_resizable(false);

        let window = Rc::new(event_loop.create_window(attrs).expect("建立視窗失敗"));
        let context = Context::new(window.clone()).expect("建立 softbuffer Context 失敗");
        let surface = Surface::new(&context, window.clone()).expect("建立 Surface 失敗");

        window.request_redraw();
        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: GlobalInput) {
        if event == GlobalInput::MouseLeft && self.cursor_inside {
            return;
        }

        match event {
            GlobalInput::KeyPress => self.state.trigger(PetState::Typing, 150),
            GlobalInput::MouseLeft => self.state.trigger(PetState::MouseLeft, 150),
            GlobalInput::MouseRight => self.state.trigger(PetState::MouseRight, 150),
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let (Some(window), Some(surface)) = (&self.window, &mut self.surface) else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => event_loop.exit(),

            WindowEvent::CursorEntered { .. } => self.cursor_inside = true,
            WindowEvent::CursorLeft { .. } => self.cursor_inside = false,

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                self.state.trigger(PetState::Petted, 400);
                window.request_redraw();
                let _ = window.drag_window();
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Right,
                ..
            } => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                let size = window.inner_size();
                let (Some(w), Some(h)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                else {
                    return;
                };

                surface.resize(w, h).unwrap();
                let mut buffer = surface.buffer_mut().unwrap();

                let sprite = self.state.current_sprite(&self.sprites);
                let dst_w = size.width as usize;
                let dst_h = size.height as usize;

                // Nearest-neighbor 貼圖，自動適配不同螢幕縮放倍率
                for y in 0..dst_h {
                    let sy = y * sprite.height / dst_h;
                    for x in 0..dst_w {
                        let sx = x * sprite.width / dst_w;
                        buffer[y * dst_w + x] = sprite.pixels[sy * sprite.width + sx];
                    }
                }

                buffer.present().unwrap();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.update(Instant::now()) {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        if let Some(deadline) = self.state.next_deadline() {
            event_loop.set_control_flow(ControlFlow::WaitUntil(deadline));
        } else {
            event_loop.set_control_flow(ControlFlow::Wait);
        }
    }
}

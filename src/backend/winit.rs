use std::{cell::{Cell, RefCell}, ffi::c_void, sync::mpsc::{self, Receiver, Sender}};

use log::*;
use raw_window_handle::HandleError;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::Key,
    event::{MouseButton, MouseScrollDelta},
    window::{Fullscreen, Window, WindowAttributes, WindowId}
};

use crate::backend::{keys::{KeyAction, Modifiers}, windows::WindowModes};

use super::{events::WindowEvents, windows::{BackendWindow, BackendWindows, Dimensions, WindowDetails, WindowHandles}, BResult, Backend, Backends};


#[derive(Debug)]
pub struct WinitBackend {
    message_proxy: EventLoopProxy<WinitMessage>,
    response_receiver: Receiver<WinitResponse>,
    event_receiver: Receiver<WinitEvent>,
    events: RefCell<Vec<Option<WinitEvent>>>
}

impl WinitBackend {
    pub fn create(callback: impl FnOnce(Backends) + Send + 'static) -> BResult<()> {
        let (response_sender, response_receiver) = mpsc::channel();
        let (event_sender, event_receiver) = mpsc::channel();
        let events = RefCell::new(Vec::new());

        let event_loop: EventLoop<WinitMessage> = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let message_proxy = event_loop.create_proxy();

        std::thread::spawn(move || {
            callback(Backends::Winit(WinitBackend { message_proxy, response_receiver, event_receiver, events }));
        });

        let mut app = WinitApp { response_sender, event_sender };
        
        event_loop.run_app(&mut app).unwrap();

        Ok(())
    }

    fn send_message(&self, message: WinitMessage) {
        self.message_proxy.send_event(message).unwrap();
    }

    fn receive_response(&self) -> Option<WinitResponse> {
        self.response_receiver.recv().ok()
    }

    fn receive_events(&self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            self.events.borrow_mut().push(Some(event));
        }
    }
}

impl Backend for WinitBackend {
    fn create_window(&self, info: WindowDetails) -> BackendWindows {
        self.send_message(WinitMessage::CreateWindow(info));
        if let Some(WinitResponse::CreateWindow(window)) = self.receive_response() {
            BackendWindows::WinitWindow(WinitWindow { 
                backend: self,
                scale: Cell::new(window.scale_factor() as f32),
                window
            })
        } else {
            panic!("Could not get created window!")
        }
    }

    fn gl_proc_address(&self, _: &str) -> *const c_void {
        todo!()
    }

    fn exit(&self) {
        self.send_message(WinitMessage::Exit);
    }
}


#[derive(Debug)]
pub struct WinitWindow<'backend> {
    backend: &'backend WinitBackend,
    window: Window,
    scale: Cell<f32>
}

impl BackendWindow for WinitWindow<'_> {
    fn handles(&self) -> Result<WindowHandles, HandleError> {
        WindowHandles::from(&self.window)
    }

    fn dimensions(&self) -> Dimensions {
        let PhysicalSize { width, height} = self.window.inner_size();
        Dimensions::new(width, height)
    }

    fn set_mode(&self, window_mode: WindowModes) {
        match window_mode {
            WindowModes::Fullscreen => {
                let monitor = self.window.primary_monitor()
                .or_else(|| self.window.available_monitors().next())
                .unwrap(); // There should always be at least one monitor available

                let fullscreen =  if let Some(handle) = monitor.video_modes().next() {
                    Fullscreen::Exclusive(handle)
                } else {
                    warn!("Unable to set Window mode to fullscreen");
                    Fullscreen::Borderless(None)
                };

                self.window.set_fullscreen(Some(fullscreen));
            },
            WindowModes::Borderless => self.window.set_fullscreen(Some(Fullscreen::Borderless(None))),
            WindowModes::Maximized => {
                self.window.set_fullscreen(None);
                self.window.set_maximized(true);
            },
            WindowModes::Windowed => {
                self.window.set_fullscreen(None);
                self.window.set_maximized(false);
            },
        }
    }

    fn flush_events(&self) -> Vec<WindowEvents> {
        self.backend.receive_events();

        let events = self.backend.events.borrow_mut()
        .iter_mut()
        .filter(
            |opt| opt.as_ref().is_some_and(
                |event| event.window == self.window.id()
            )
        )
        .map(
            |opt| opt.take().unwrap().event
        )
        .collect();

        self.backend.events.borrow_mut().retain(|opt| opt.is_some());

        events
    }

    fn target_scale(&self) -> f32 {
        self.window.scale_factor() as f32
    }

    fn current_scale(&self) -> f32 {
        self.scale.get()
    }

    fn set_scale(&self, scale: f32) {
        self.scale.set(scale);
    }
}


#[derive(Debug)]
struct WinitApp {
    response_sender: Sender<WinitResponse>,
    event_sender: Sender<WinitEvent>
}

impl WinitApp {
    fn respond(&self, response: WinitResponse) {
        self.response_sender.send(response).unwrap();
    }
}

impl ApplicationHandler<WinitMessage> for WinitApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        //self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        self.event_sender.send(WinitEvent {
            window: window_id,
            event: match event {
                WindowEvent::RedrawRequested => {
                    WindowEvents::Redraw
                },
                WindowEvent::CursorMoved { position, .. } => {
                    let (x, y) = (position.x.round() as _, position.y.round() as _);

                    WindowEvents::CursorPos(x, y)
                },
                WindowEvent::CloseRequested => {
                    WindowEvents::CloseRequested
                },
                WindowEvent::DroppedFile(path) => WindowEvents::FileDropped(path),
                WindowEvent::Focused(focus) => WindowEvents::FocusChange(focus),
                WindowEvent::Resized(size) => {
                    // TODO: Scaling
                    WindowEvents::Resize(size.width, size.height)
                },
                WindowEvent::KeyboardInput { device_id: _, event, is_synthetic } => {
                    if is_synthetic { return; } // I hope this is correct...

                    let state = match event.state {
                        ElementState::Pressed => if event.repeat { KeyAction::Hold } else { KeyAction::Press }
                        ElementState::Released => KeyAction::Release
                    };
                    let text = if let Key::Character(c) = event.logical_key {
                        Some(c)
                    } else { None };

                    // TODO: Modifiers
                    WindowEvents::Key(event.physical_key.into(), text, state, Modifiers::empty())
                },
                WindowEvent::MouseInput { device_id: _, state, button } => {
                    let button_num = match button {
                        MouseButton::Left => 1,
                        MouseButton::Right => 2,
                        MouseButton::Middle => 3,
                        MouseButton::Back => 4,
                        MouseButton::Forward => 5,
                        MouseButton::Other(num) => num as u32,
                    };
                    let state = match state {
                        ElementState::Pressed => KeyAction::Press,
                        ElementState::Released => KeyAction::Release,
                    };

                    WindowEvents::MouseButton(button_num, state)
                },
                WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                    let (x, y) = match delta {
                        MouseScrollDelta::LineDelta(x, y) => (x as i32 * 10, y as i32 * 10), // TODO: Adjust
                        MouseScrollDelta::PixelDelta(pos) => (pos.x as _, pos.y as _),
                    };

                    WindowEvents::MouseScroll(x, y)
                },
                WindowEvent::Touch(_event) => {
                    // TODO: Handle this properly
                    // println!("{event:?}");
                    return;
                },
                WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => {
                    // TODO
                    return
                },
                _event => {
                    // debug!("{:?}", event);
                    return;
                }
            }
        }).ok();
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: WinitMessage) {
        match event {
            WinitMessage::CreateWindow(details) => {
                debug!("Creating window: {details:?}");
                // Monitor for exclusive fullscreen
                let monitor = event_loop.primary_monitor()
                .or_else(|| event_loop.available_monitors().next()) // TODO: Implement selectable monitor
                .unwrap(); // There should always be at least one monitor available

                let mut attributes = WindowAttributes::default()
                .with_inner_size(PhysicalSize::new(details.width, details.height))
                .with_title(details.title)
                .with_transparent(true);

                match details.mode {
                    WindowModes::Fullscreen => attributes.fullscreen = if let Some(handle) = monitor.video_modes().next() {
                        Some(Fullscreen::Exclusive(handle))
                    } else {
                        warn!("Unable to set initial Window mode to fullscreen");
                        Some(Fullscreen::Borderless(None))
                    },
                    WindowModes::Borderless => attributes.fullscreen = Some(Fullscreen::Borderless(None)),
                    WindowModes::Maximized => attributes.maximized = true,
                    WindowModes::Windowed => attributes.maximized = false,
                }

                let window = event_loop.create_window(attributes).unwrap();

                self.respond(WinitResponse::CreateWindow(window));
            },
            WinitMessage::Exit => {
                event_loop.exit();
            }
        }
    }
}

#[derive(Debug)]
struct WinitEvent {
    window: WindowId,
    event: WindowEvents
}

#[derive(Debug)]
enum WinitMessage {
    CreateWindow(WindowDetails),
    Exit
}

#[derive(Debug)]
enum WinitResponse {
    CreateWindow(Window)
}
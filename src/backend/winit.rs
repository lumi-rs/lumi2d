use std::{cell::Cell, ffi::c_void, sync::mpsc::{self, Receiver, Sender}};

use log::*;
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent as WinitEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::Key,
    window::{Fullscreen, WindowAttributes}
};

use crate::backend::{keys::{KeyAction, Modifiers}, windows::WindowModes};

use super::{events::WindowEvent, windows::{BackendEvent, Window, WindowDetails, WindowId}, winit_window::WinitWindow, BResult, Backend, BackendTrait};


#[derive(Debug)]
pub struct WinitBackend {
    message_proxy: EventLoopProxy<WinitMessage>,
    response_receiver: Receiver<WinitResponse>,
    pub event_receiver: Receiver<BackendEvent>,
    pub event_sender: Sender<BackendEvent>,
    exiting: Cell<bool>
}

impl WinitBackend {
    pub fn create(callback: impl FnOnce(Backend) + Send + 'static) -> BResult<()> {
        let (response_sender, response_receiver) = mpsc::channel();
        let (event_sender, event_receiver) = mpsc::channel();
        

        let event_loop: EventLoop<WinitMessage> = EventLoop::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let message_proxy = event_loop.create_proxy();

        let cloned = event_sender.clone();
        std::thread::spawn(move || {
            callback(Backend::Winit(WinitBackend {
                message_proxy, response_receiver, event_receiver, event_sender: cloned,
                exiting: Cell::new(false)
            }));
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
}

impl BackendTrait for WinitBackend {
    fn create_window(&self, info: WindowDetails) -> Window {
        self.send_message(WinitMessage::CreateWindow(info));

        if let Some(WinitResponse::CreateWindow(window)) = self.receive_response() {
            Window::WinitWindow(WinitWindow {
                scale: Cell::new(window.scale_factor() as f32),
                window,
            })
        } else {
            panic!("Could not get created window!")
        }
    }

    fn gl_proc_address(&self, _: &str) -> *const c_void {
        todo!()
    }

    fn exit(&self) {
        self.exiting.set(true);
        self.send_message(WinitMessage::Exit);
    }

    fn subscribe_events(&self, mut callback: impl FnMut(Vec<BackendEvent>)) {
        if crate::polling() {
            while !self.exiting.get() {
                callback(self.flush_events());
            }
        } else {
            while let Ok(event) = self.event_receiver.recv() {
                let mut events = Vec::with_capacity(4);
                events.push(event);
    
                while let Ok(queued_event) = self.event_receiver.try_recv() {
                    events.push(queued_event);
                }
    
                callback(events);
                if self.exiting.get() { break; }
            }
        }
    }

    fn flush_events(&self) -> Vec<BackendEvent> {
        let mut events = Vec::with_capacity(4);
        while let Ok(event) = self.event_receiver.try_recv() {
            events.push(event);
        }
        events
    }
}



#[derive(Debug)]
struct WinitApp {
    response_sender: Sender<WinitResponse>,
    event_sender: Sender<BackendEvent>
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

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, window_id: winit::window::WindowId, event: WinitEvent) {
        if let Some(event) = convert_event(event) {
            self.event_sender.send(BackendEvent {
                event,
                window_id: WindowId::Winit(window_id)
            }).ok();
        }
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
enum WinitMessage {
    CreateWindow(WindowDetails),
    Exit
}

#[derive(Debug)]
enum WinitResponse {
    CreateWindow(winit::window::Window)
}

fn convert_event(event: WinitEvent) -> Option<WindowEvent> {
    Some(match event {
        WinitEvent::RedrawRequested => WindowEvent::Redraw,
        WinitEvent::CloseRequested => WindowEvent::CloseRequested,
        WinitEvent::DroppedFile(path) => WindowEvent::FileDropped(path),
        WinitEvent::Focused(focus) => WindowEvent::FocusChange(focus),
        WinitEvent::CursorMoved { position, .. } => {
            let PhysicalPosition { x, y } = position;
            WindowEvent::CursorPos((x, y).into())
        },
        WinitEvent::Resized(size) => {
            let PhysicalSize { width, height } = size;
            WindowEvent::WindowSize((width, height).into())
        },
        WinitEvent::Moved(position) => {
            let PhysicalPosition { x, y } = position;
            WindowEvent::WindowPos((x, y).into())
        },
        WinitEvent::KeyboardInput { device_id: _, event, is_synthetic } => {
            if is_synthetic { return None; } // I hope this is correct...

            let state = match event.state {
                ElementState::Pressed => if event.repeat { KeyAction::Hold } else { KeyAction::Press }
                ElementState::Released => KeyAction::Release
            };
            let text = if let Key::Character(c) = event.logical_key {
                Some(c)
            } else { None };

            // TODO: Modifiers
            WindowEvent::Key(event.physical_key.into(), text, state, Modifiers::empty())
        },
        WinitEvent::MouseInput { device_id: _, state, button } => {
            let button_num = match button {
                MouseButton::Left => 1,
                MouseButton::Right => 2,
                MouseButton::Middle => 3,
                MouseButton::Back => 4,
                MouseButton::Forward => 5,
                MouseButton::Other(num) => num.into(),
            };
            let state = match state {
                ElementState::Pressed => KeyAction::Press,
                ElementState::Released => KeyAction::Release,
            };

            WindowEvent::MouseButton(button_num, state)
        },
        WinitEvent::MouseWheel { device_id: _, delta, phase: _ } => {
            let (x, y) = match delta {
                MouseScrollDelta::LineDelta(x, y) => (x as i32 * 10, y as i32 * 10), // TODO: Adjust
                MouseScrollDelta::PixelDelta(pos) => (pos.x as _, pos.y as _),
            };

            WindowEvent::MouseScroll(x, y)
        },
        WinitEvent::Touch(_event) => {
            // TODO: Handle this properly
            // println!("{event:?}");
            return None;
        },
        WinitEvent::ScaleFactorChanged { scale_factor, inner_size_writer: _ } => {
            WindowEvent::ScaleFactor(scale_factor as _)
        },
        WinitEvent::ModifiersChanged(_modifiers) => {
            // TODO: Handle this
            return None;
        },
        _event => {
            // debug!("{:?}", event);
            return None;
        }
    })
}
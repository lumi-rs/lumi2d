use std::cell::Cell;

use log::*;
use raw_window_handle::HandleError;
use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::Key,
    window::{Fullscreen, Window}
};

use crate::structs::Dimensions;

use super::{events::WindowEvents, keys::{KeyAction, Modifiers}, windows::{BackendEvent, WindowTrait, WindowHandles, WindowId, WindowModes}, WinitBackend};


#[derive(Debug)]
pub struct WinitWindow<'backend> {
    pub backend: &'backend WinitBackend,
    pub window: Window,
    pub scale: Cell<f32>
}

impl WinitWindow<'_> {
    fn _convert_event(&self, event: WindowEvent) -> Option<WindowEvents> {
        Some(match event {
                WindowEvent::RedrawRequested => WindowEvents::Redraw,
                WindowEvent::CloseRequested => WindowEvents::CloseRequested,
                WindowEvent::DroppedFile(path) => WindowEvents::FileDropped(path),
                WindowEvent::Focused(focus) => WindowEvents::FocusChange(focus),
                WindowEvent::CursorMoved { position, .. } => {
                    let xy: (u32, u32) = position.to_logical::<f64>(self.current_scale() as _).into();

                    WindowEvents::CursorPos(xy.into())
                },
                WindowEvent::Resized(size) => {
                    let LogicalSize { width, height } = size.to_logical::<u32>(self.current_scale() as _);

                    WindowEvents::WindowSize((width, height).into())
                },
                WindowEvent::Moved(position) => {
                    let LogicalPosition { x, y } = position.to_logical::<i32>(self.current_scale() as _);
                    
                    WindowEvents::WindowPos((x, y).into())
                },
                WindowEvent::KeyboardInput { device_id: _, event, is_synthetic } => {
                    if is_synthetic { return None; } // I hope this is correct...

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
                        MouseButton::Other(num) => num.into(),
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
                    return None;
                },
                WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer: _ } => {
                    WindowEvents::ScaleFactor(scale_factor as _)
                },
                WindowEvent::ModifiersChanged(_modifiers) => {
                    // TODO: Handle this
                    return None;
                },
                _event => {
                    // debug!("{:?}", event);
                    return None;
                }
        })
    }
}

impl WindowTrait for WinitWindow<'_> {
    fn handles(&self) -> Result<WindowHandles, HandleError> {
        WindowHandles::from(&self.window)
    }

    fn physical_dimensions(&self) -> Dimensions {
        let PhysicalSize { width, height} = self.window.inner_size();
        Dimensions::new(width, height)
    }

    fn dimensions(&self) -> Dimensions {
        let LogicalSize { width, height} = self.window.inner_size().to_logical(self.current_scale() as _);
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

    fn target_scale(&self) -> f32 {
        self.window.scale_factor() as f32
    }

    fn current_scale(&self) -> f32 {
        self.scale.get()
    }

    fn set_scale(&self, scale: f32) {
        self.scale.set(scale);
    }

    fn send_event(&self, event: WindowEvents) {
        self.backend.event_sender.send(BackendEvent { event, window_id: WindowId::Winit(self.window.id()) }).ok();
    }

    fn id(&self) -> WindowId {
        WindowId::Winit(self.window.id())
    }

    fn close(self) {
        drop(self.window);
    }
}
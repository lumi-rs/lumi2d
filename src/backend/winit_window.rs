use std::cell::Cell;

use log::*;
use raw_window_handle::HandleError;
use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent as WinitEvent},
    keyboard::Key,
    window::{Fullscreen, Window}
};

use crate::structs::Dimensions;

use super::{events::WindowEvent, keys::{KeyAction, Modifiers}, windows::{BackendEvent, WindowTrait, WindowHandles, WindowId, WindowModes}, WinitBackend};


#[derive(Debug)]
pub struct WinitWindow<'backend> {
    pub backend: &'backend WinitBackend,
    pub window: Window,
    pub scale: Cell<f32>
}

impl WinitWindow<'_> {
    fn _convert_event(&self, event: WinitEvent) -> Option<WindowEvent> {
        Some(match event {
                WinitEvent::RedrawRequested => WindowEvent::Redraw,
                WinitEvent::CloseRequested => WindowEvent::CloseRequested,
                WinitEvent::DroppedFile(path) => WindowEvent::FileDropped(path),
                WinitEvent::Focused(focus) => WindowEvent::FocusChange(focus),
                WinitEvent::CursorMoved { position, .. } => {
                    let xy: (u32, u32) = position.to_logical::<f64>(self.current_scale() as _).into();

                    WindowEvent::CursorPos(xy.into())
                },
                WinitEvent::Resized(size) => {
                    let LogicalSize { width, height } = size.to_logical::<u32>(self.current_scale() as _);

                    WindowEvent::WindowSize((width, height).into())
                },
                WinitEvent::Moved(position) => {
                    let LogicalPosition { x, y } = position.to_logical::<i32>(self.current_scale() as _);
                    
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

    fn send_event(&self, event: WindowEvent) {
        self.backend.event_sender.send(BackendEvent { event, window_id: WindowId::Winit(self.window.id()) }).ok();
    }

    fn id(&self) -> WindowId {
        WindowId::Winit(self.window.id())
    }

    fn close(self) {
        // self is dropped, closing the window.
    }
}
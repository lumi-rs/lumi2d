use std::path::PathBuf;

use smol_str::SmolStr;

use crate::structs::{Dimensions, Position};

use super::{keys::*, windowing::window::WindowId};


/// An enum of all the possible events a window can emit.  
/// All `Position`s and `Dimension`s already take the window's scale factor into account.
#[derive(Debug, PartialEq, Clone)]
pub enum WindowEvent {
    Redraw,
    CloseRequested,
    /// Will not be emitted on Wayland, mobile and web!
    WindowPos(Position<i32>),
    WindowSize(Dimensions<u32>),
    CursorPos(Position<f64>),
    /// Button number, KeyAction
    MouseButton(u32, KeyAction),
    /// x offset, y offset  
    /// +x is content moving right (scroll left), +y is content moving down (scroll up)
    MouseScroll(i32, i32),
    /// Key, Text, KeyAction, KeyModifiers
    Key(PhysicalKey, Option<SmolStr>, KeyAction, Modifiers),
    /// true == focused
    FocusChange(bool),
    FileDropped(PathBuf),
    /// f32 = new scale
    ScaleFactor(f32)
}


impl WindowEvent {
    /// Adjusts the events that are dependant on scale, like CursorPos.
    pub fn scale_with(self, scale: f32) -> Self {
        match self {
            WindowEvent::WindowSize(dim) => WindowEvent::WindowSize(dim / scale),
            WindowEvent::CursorPos(pos) => WindowEvent::CursorPos(pos / scale),
            other => other
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct BackendEvent {
    pub event: WindowEvent,
    pub window_id: WindowId
}


#[derive(Debug, PartialEq, Clone)]
pub enum Event<T> {
    Backend(BackendEvent),
    Custom(T)
}
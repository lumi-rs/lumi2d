use std::path::PathBuf;

use smol_str::SmolStr;

use super::keys::*;


#[derive(Debug, PartialEq)]
pub enum WindowEvents {
    Redraw,
    CloseRequested,
    /// x, y
    CursorPos(u32, u32),
    /// x, y
    WindowPos(u32, u32),
    // Button number, KeyAction
    MouseButton(u32, KeyAction),
    /// Key, Text, KeyAction, KeyModifiers
    Key(PhysicalKey, Option<SmolStr>, KeyAction, Modifiers),
    /// x offset, y offset  
    /// +x is content moving right (scroll left), +y is content moving down (scroll up)
    MouseScroll(i32, i32),
    /// width, height
    Resize(u32, u32),
    /// true == focused
    FocusChange(bool),
    FileDropped(PathBuf),
    /// new scale
    ScaleFactor(f32)
}

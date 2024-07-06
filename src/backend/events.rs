
#[derive(Debug, PartialEq)]
pub enum WindowEvents {
    Redraw,
    CloseRequested,
    CursorPos(u32, u32), // x, y
    WindowPos(u32, u32), // x, y
    MouseButton(u32, KeyAction),
    Key(u32, u32, KeyAction, Modifiers), // KeyType, Scancode, KeyAction, KeyModifiers
    Char(char, Modifiers),
    Scroll(u32, u32), // x offset, y offset
    Resize(u32, u32), // width, height
    FocusChange(bool), // true == focused
}

#[derive(Debug, PartialEq)]
pub enum KeyAction {
    Press,
    Hold,
    Release
}


bitflags::bitflags! {
    #[derive(Debug, PartialEq)]
    pub struct Modifiers: u8 {
        const Shift = 1;
        const Control = 1 << 1;
        const Alt = 1 << 2;
        const Super = 1 << 3;
        const CapsLock = 1 << 4;
        const NumLock = 1 << 5;
    }
}
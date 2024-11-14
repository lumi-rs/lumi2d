use log::warn;
use winit::keyboard::KeyCode;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyAction {
    Press,
    Hold,
    Release
}

bitflags::bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Modifiers: u8 {
        const Shift = 1;
        const Control = 1 << 1;
        const Alt = 1 << 2;
        const Super = 1 << 3;
        const CapsLock = 1 << 4;
        const NumLock = 1 << 5;
    }
}

/// An Enum of keyboard keys.  
/// Taken from `winit` (https://github.com/rust-windowing/winit/blob/73c01fff96266a6a7d9159e9bd26acc00df0bbc9/src/keyboard.rs#L296)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PhysicalKey {
    /// <kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
    /// This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
    /// (hankaku/zenkaku/kanji) key on Japanese keyboards
    Backquote,
    /// Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key
    /// located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labeled <kbd>#</kbd> on a UK (102) keyboard.
    Backslash,
    /// <kbd>[</kbd> on a US keyboard.
    BracketLeft,
    /// <kbd>]</kbd> on a US keyboard.
    BracketRight,
    /// <kbd>,</kbd> on a US keyboard.
    Comma,
    /// <kbd>0</kbd> on a US keyboard.
    Digit0,
    /// <kbd>1</kbd> on a US keyboard.
    Digit1,
    /// <kbd>2</kbd> on a US keyboard.
    Digit2,
    /// <kbd>3</kbd> on a US keyboard.
    Digit3,
    /// <kbd>4</kbd> on a US keyboard.
    Digit4,
    /// <kbd>5</kbd> on a US keyboard.
    Digit5,
    /// <kbd>6</kbd> on a US keyboard.
    Digit6,
    /// <kbd>7</kbd> on a US keyboard.
    Digit7,
    /// <kbd>8</kbd> on a US keyboard.
    Digit8,
    /// <kbd>9</kbd> on a US keyboard.
    Digit9,
    /// <kbd>=</kbd> on a US keyboard.
    Equal,
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
    /// Labeled <kbd>\\</kbd> on a UK keyboard.
    IntlBackslash,
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
    /// Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard.
    IntlRo,
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
    /// Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a
    /// Russian keyboard.
    IntlYen,
    /// <kbd>a</kbd> on a US keyboard.
    /// Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    KeyA,
    /// <kbd>b</kbd> on a US keyboard.
    KeyB,
    /// <kbd>c</kbd> on a US keyboard.
    KeyC,
    /// <kbd>d</kbd> on a US keyboard.
    KeyD,
    /// <kbd>e</kbd> on a US keyboard.
    KeyE,
    /// <kbd>f</kbd> on a US keyboard.
    KeyF,
    /// <kbd>g</kbd> on a US keyboard.
    KeyG,
    /// <kbd>h</kbd> on a US keyboard.
    KeyH,
    /// <kbd>i</kbd> on a US keyboard.
    KeyI,
    /// <kbd>j</kbd> on a US keyboard.
    KeyJ,
    /// <kbd>k</kbd> on a US keyboard.
    KeyK,
    /// <kbd>l</kbd> on a US keyboard.
    KeyL,
    /// <kbd>m</kbd> on a US keyboard.
    KeyM,
    /// <kbd>n</kbd> on a US keyboard.
    KeyN,
    /// <kbd>o</kbd> on a US keyboard.
    KeyO,
    /// <kbd>p</kbd> on a US keyboard.
    KeyP,
    /// <kbd>q</kbd> on a US keyboard.
    /// Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    KeyQ,
    /// <kbd>r</kbd> on a US keyboard.
    KeyR,
    /// <kbd>s</kbd> on a US keyboard.
    KeyS,
    /// <kbd>t</kbd> on a US keyboard.
    KeyT,
    /// <kbd>u</kbd> on a US keyboard.
    KeyU,
    /// <kbd>v</kbd> on a US keyboard.
    KeyV,
    /// <kbd>w</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    KeyW,
    /// <kbd>x</kbd> on a US keyboard.
    KeyX,
    /// <kbd>y</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    KeyY,
    /// <kbd>z</kbd> on a US keyboard.
    /// Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
    /// QWERTZ (e.g., German) keyboard.
    KeyZ,
    /// <kbd>-</kbd> on a US keyboard.
    Minus,
    /// <kbd>.</kbd> on a US keyboard.
    Period,
    /// <kbd>'</kbd> on a US keyboard.
    Quote,
    /// <kbd>;</kbd> on a US keyboard.
    Semicolon,
    /// <kbd>/</kbd> on a US keyboard.
    Slash,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    AltLeft,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    /// This is labeled <kbd>AltGr</kbd> on many keyboard layouts.
    AltRight,
    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
    /// Labeled <kbd>Delete</kbd> on Apple keyboards.
    Backspace,
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    CapsLock,
    /// The application context menu key, which is typically found between the right
    /// <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.
    ContextMenu,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
    Enter,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperLeft,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperRight,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftLeft,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftRight,
    /// <kbd> </kbd> (space)
    Space,
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    Tab,
    /// Japanese: <kbd>変</kbd> (henkan)
    Convert,
    /// Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd>
    /// (katakana/hiragana/romaji)
    KanaMode,
    /// Korean: HangulMode <kbd>한/영</kbd> (han/yeong)
    ///
    /// Japanese (Mac keyboard): <kbd>か</kbd> (kana)
    Lang1,
    /// Korean: Hanja <kbd>한</kbd> (hanja)
    ///
    /// Japanese (Mac keyboard): <kbd>英</kbd> (eisu)
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    Lang5,
    /// Japanese: <kbd>無変換</kbd> (muhenkan)
    NonConvert,
    /// <kbd>⌦</kbd>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
    /// the keyboard is encoded as [`Backspace`].
    ///
    /// [`Backspace`]: Self::Backspace
    Delete,
    /// <kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd>
    End,
    /// <kbd>Help</kbd>. Not present on standard PC keyboards.
    Help,
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    Home,
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    Insert,
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd>
    PageDown,
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd>
    PageUp,
    /// <kbd>↓</kbd>
    ArrowDown,
    /// <kbd>←</kbd>
    ArrowLeft,
    /// <kbd>→</kbd>
    ArrowRight,
    /// <kbd>↑</kbd>
    ArrowUp,
    /// On the Mac, this is used for the numpad <kbd>Clear</kbd> key.
    NumLock,
    /// <kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control
    Numpad0,
    /// <kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote
    /// control
    Numpad1,
    /// <kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control
    Numpad2,
    /// <kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control
    Numpad3,
    /// <kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control
    Numpad4,
    /// <kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control
    Numpad5,
    /// <kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control
    Numpad6,
    /// <kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
    /// or remote control
    Numpad7,
    /// <kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control
    Numpad8,
    /// <kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
    /// or remote control
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    NumpadBackspace,
    /// <kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
    /// <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
    /// numpad <kbd>Clear</kbd> key is encoded as [`NumLock`].
    ///
    /// [`NumLock`]: Self::NumLock
    NumpadClear,
    /// <kbd>C</kbd> (Clear Entry)
    NumpadClearEntry,
    /// <kbd>,</kbd> (thousands separator). For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
    NumpadComma,
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <kbd>,</kbd>.
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    NumpadEnter,
    /// <kbd>=</kbd>
    NumpadEqual,
    /// <kbd>#</kbd> on a phone or remote control device. This key is typically found
    /// below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
    NumpadHash,
    /// <kbd>M</kbd> Add current entry to the value stored in memory.
    NumpadMemoryAdd,
    /// <kbd>M</kbd> Clear the value stored in memory.
    NumpadMemoryClear,
    /// <kbd>M</kbd> Replace the current entry with the value stored in memory.
    NumpadMemoryRecall,
    /// <kbd>M</kbd> Replace the value stored in memory with the current entry.
    NumpadMemoryStore,
    /// <kbd>M</kbd> Subtract current entry from the value stored in memory.
    NumpadMemorySubtract,
    /// <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
    /// operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).
    ///
    /// Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls.
    NumpadMultiply,
    /// <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenLeft,
    /// <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenRight,
    /// <kbd>*</kbd> on a phone or remote control device.
    ///
    /// This key is typically found below the <kbd>7</kbd> key and to the left of
    /// the <kbd>0</kbd> key.
    ///
    /// Use <kbd>"NumpadMultiply"</kbd> for the <kbd>*</kbd> key on
    /// numeric keypads.
    NumpadStar,
    /// <kbd>-</kbd>
    NumpadSubtract,
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.
    Fn,
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// Some laptops place this key to the left of the <kbd>↑</kbd> key.
    ///
    /// This also the "back" button (triangle) on Android.
    BrowserBack,
    BrowserFavorites,
    /// Some laptops place this key to the right of the <kbd>↑</kbd> key.
    BrowserForward,
    /// The "home" button on Android.
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    /// <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
    /// keyboards.
    Eject,
    /// Sometimes labelled <kbd>My Computer</kbd> on the keyboard
    LaunchApp1,
    /// Sometimes labelled <kbd>Calculator</kbd> on the keyboard
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards, replacing the
    /// <kbd>Eject</kbd> key.
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    // Legacy modifier key. Also called "Super" in certain places.
    Meta,
    // Legacy modifier key.
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    /// Found on Sun’s USB keyboard.
    Again,
    /// Found on Sun’s USB keyboard.
    Copy,
    /// Found on Sun’s USB keyboard.
    Cut,
    /// Found on Sun’s USB keyboard.
    Find,
    /// Found on Sun’s USB keyboard.
    Open,
    /// Found on Sun’s USB keyboard.
    Paste,
    /// Found on Sun’s USB keyboard.
    Props,
    /// Found on Sun’s USB keyboard.
    Select,
    /// Found on Sun’s USB keyboard.
    Undo,
    /// Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
    Hiragana,
    /// Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
    Katakana,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F1,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F2,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F3,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F4,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F5,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F6,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F7,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F8,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F9,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F10,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F11,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F12,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F13,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F14,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F15,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F16,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F17,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F18,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F19,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F20,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F21,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F22,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F23,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F24,
    /// General-purpose function key.
    F25,
    /// General-purpose function key.
    F26,
    /// General-purpose function key.
    F27,
    /// General-purpose function key.
    F28,
    /// General-purpose function key.
    F29,
    /// General-purpose function key.
    F30,
    /// General-purpose function key.
    F31,
    /// General-purpose function key.
    F32,
    /// General-purpose function key.
    F33,
    /// General-purpose function key.
    F34,
    /// General-purpose function key.
    F35,
    /// Any other key not listed here
    Unidentified
}



#[cfg(feature = "b-winit")]
impl From<winit::keyboard::PhysicalKey> for PhysicalKey {
    fn from(value: winit::keyboard::PhysicalKey) -> Self {
        match value {
            winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                KeyCode::Backquote => Self::Backquote,
                KeyCode::Backslash => Self::Backslash,
                KeyCode::BracketLeft => Self::BracketLeft,
                KeyCode::BracketRight => Self::BracketRight,
                KeyCode::Comma => Self::Comma,
                KeyCode::Digit0 => Self::Digit0,
                KeyCode::Digit1 => Self::Digit1,
                KeyCode::Digit2 => Self::Digit2,
                KeyCode::Digit3 => Self::Digit3,
                KeyCode::Digit4 => Self::Digit4,
                KeyCode::Digit5 => Self::Digit5,
                KeyCode::Digit6 => Self::Digit6,
                KeyCode::Digit7 => Self::Digit7,
                KeyCode::Digit8 => Self::Digit8,
                KeyCode::Digit9 => Self::Digit9,
                KeyCode::Equal => Self::Equal,
                KeyCode::IntlBackslash => Self::IntlBackslash,
                KeyCode::IntlRo => Self::IntlRo,
                KeyCode::IntlYen => Self::IntlYen,
                KeyCode::KeyA => Self::KeyA,
                KeyCode::KeyB => Self::KeyB,
                KeyCode::KeyC => Self::KeyC,
                KeyCode::KeyD => Self::KeyD,
                KeyCode::KeyE => Self::KeyE,
                KeyCode::KeyF => Self::KeyF,
                KeyCode::KeyG => Self::KeyG,
                KeyCode::KeyH => Self::KeyH,
                KeyCode::KeyI => Self::KeyI,
                KeyCode::KeyJ => Self::KeyJ,
                KeyCode::KeyK => Self::KeyK,
                KeyCode::KeyL => Self::KeyL,
                KeyCode::KeyM => Self::KeyM,
                KeyCode::KeyN => Self::KeyN,
                KeyCode::KeyO => Self::KeyO,
                KeyCode::KeyP => Self::KeyP,
                KeyCode::KeyQ => Self::KeyQ,
                KeyCode::KeyR => Self::KeyR,
                KeyCode::KeyS => Self::KeyS,
                KeyCode::KeyT => Self::KeyT,
                KeyCode::KeyU => Self::KeyU,
                KeyCode::KeyV => Self::KeyV,
                KeyCode::KeyW => Self::KeyW,
                KeyCode::KeyX => Self::KeyX,
                KeyCode::KeyY => Self::KeyY,
                KeyCode::KeyZ => Self::KeyZ,
                KeyCode::Minus => Self::Minus,
                KeyCode::Period => Self::Period,
                KeyCode::Quote => Self::Quote,
                KeyCode::Semicolon => Self::Semicolon,
                KeyCode::Slash => Self::Slash,
                KeyCode::AltLeft => Self::AltLeft,
                KeyCode::AltRight => Self::AltRight,
                KeyCode::Backspace => Self::Backspace,
                KeyCode::CapsLock => Self::CapsLock,
                KeyCode::ContextMenu => Self::ContextMenu,
                KeyCode::ControlLeft => Self::ControlLeft,
                KeyCode::ControlRight => Self::ControlRight,
                KeyCode::Enter => Self::Enter,
                KeyCode::SuperLeft => Self::SuperLeft,
                KeyCode::SuperRight => Self::SuperRight,
                KeyCode::ShiftLeft => Self::ShiftLeft,
                KeyCode::ShiftRight => Self::ShiftRight,
                KeyCode::Space => Self::Space,
                KeyCode::Tab => Self::Tab,
                KeyCode::Convert => Self::Convert,
                KeyCode::KanaMode => Self::KanaMode,
                KeyCode::Lang1 => Self::Lang1,
                KeyCode::Lang2 => Self::Lang2,
                KeyCode::Lang3 => Self::Lang3,
                KeyCode::Lang4 => Self::Lang4,
                KeyCode::Lang5 => Self::Lang5,
                KeyCode::NonConvert => Self::NonConvert,
                KeyCode::Delete => Self::Delete,
                KeyCode::End => Self::End,
                KeyCode::Help => Self::Help,
                KeyCode::Home => Self::Home,
                KeyCode::Insert => Self::Insert,
                KeyCode::PageDown => Self::PageDown,
                KeyCode::PageUp => Self::PageUp,
                KeyCode::ArrowDown => Self::ArrowDown,
                KeyCode::ArrowLeft => Self::ArrowLeft,
                KeyCode::ArrowRight => Self::ArrowRight,
                KeyCode::ArrowUp => Self::ArrowUp,
                KeyCode::NumLock => Self::NumLock,
                KeyCode::Numpad0 => Self::Numpad0,
                KeyCode::Numpad1 => Self::Numpad1,
                KeyCode::Numpad2 => Self::Numpad2,
                KeyCode::Numpad3 => Self::Numpad3,
                KeyCode::Numpad4 => Self::Numpad4,
                KeyCode::Numpad5 => Self::Numpad5,
                KeyCode::Numpad6 => Self::Numpad6,
                KeyCode::Numpad7 => Self::Numpad7,
                KeyCode::Numpad8 => Self::Numpad8,
                KeyCode::Numpad9 => Self::Numpad9,
                KeyCode::NumpadAdd => Self::NumpadAdd,
                KeyCode::NumpadBackspace => Self::NumpadBackspace,
                KeyCode::NumpadClear => Self::NumpadClear,
                KeyCode::NumpadClearEntry => Self::NumpadClearEntry,
                KeyCode::NumpadComma => Self::NumpadComma,
                KeyCode::NumpadDecimal => Self::NumpadDecimal,
                KeyCode::NumpadDivide => Self::NumpadDivide,
                KeyCode::NumpadEnter => Self::NumpadEnter,
                KeyCode::NumpadEqual => Self::NumpadEqual,
                KeyCode::NumpadHash => Self::NumpadHash,
                KeyCode::NumpadMemoryAdd => Self::NumpadMemoryAdd,
                KeyCode::NumpadMemoryClear => Self::NumpadMemoryClear,
                KeyCode::NumpadMemoryRecall => Self::NumpadMemoryRecall,
                KeyCode::NumpadMemoryStore => Self::NumpadMemoryStore,
                KeyCode::NumpadMemorySubtract => Self::NumpadMemorySubtract,
                KeyCode::NumpadMultiply => Self::NumpadMultiply,
                KeyCode::NumpadParenLeft => Self::NumpadParenLeft,
                KeyCode::NumpadParenRight => Self::NumpadParenRight,
                KeyCode::NumpadStar => Self::NumpadStar,
                KeyCode::NumpadSubtract => Self::NumpadSubtract,
                KeyCode::Escape => Self::Escape,
                KeyCode::Fn => Self::Fn,
                KeyCode::FnLock => Self::FnLock,
                KeyCode::PrintScreen => Self::PrintScreen,
                KeyCode::ScrollLock => Self::ScrollLock,
                KeyCode::Pause => Self::Pause,
                KeyCode::BrowserBack => Self::BrowserBack,
                KeyCode::BrowserFavorites => Self::BrowserFavorites,
                KeyCode::BrowserForward => Self::BrowserForward,
                KeyCode::BrowserHome => Self::BrowserHome,
                KeyCode::BrowserRefresh => Self::BrowserRefresh,
                KeyCode::BrowserSearch => Self::BrowserSearch,
                KeyCode::BrowserStop => Self::BrowserStop,
                KeyCode::Eject => Self::Eject,
                KeyCode::LaunchApp1 => Self::LaunchApp1,
                KeyCode::LaunchApp2 => Self::LaunchApp2,
                KeyCode::LaunchMail => Self::LaunchMail,
                KeyCode::MediaPlayPause => Self::MediaPlayPause,
                KeyCode::MediaSelect => Self::MediaSelect,
                KeyCode::MediaStop => Self::MediaStop,
                KeyCode::MediaTrackNext => Self::MediaTrackNext,
                KeyCode::MediaTrackPrevious => Self::MediaTrackPrevious,
                KeyCode::Power => Self::Power,
                KeyCode::Sleep => Self::Sleep,
                KeyCode::AudioVolumeDown => Self::AudioVolumeDown,
                KeyCode::AudioVolumeMute => Self::AudioVolumeMute,
                KeyCode::AudioVolumeUp => Self::AudioVolumeUp,
                KeyCode::WakeUp => Self::WakeUp,
                KeyCode::Meta => Self::Meta,
                KeyCode::Hyper => Self::Hyper,
                KeyCode::Turbo => Self::Turbo,
                KeyCode::Abort => Self::Abort,
                KeyCode::Resume => Self::Resume,
                KeyCode::Suspend => Self::Suspend,
                KeyCode::Again => Self::Again,
                KeyCode::Copy => Self::Copy,
                KeyCode::Cut => Self::Cut,
                KeyCode::Find => Self::Find,
                KeyCode::Open => Self::Open,
                KeyCode::Paste => Self::Paste,
                KeyCode::Props => Self::Props,
                KeyCode::Select => Self::Select,
                KeyCode::Undo => Self::Undo,
                KeyCode::Hiragana => Self::Hiragana,
                KeyCode::Katakana => Self::Katakana,
                KeyCode::F1 => Self::F1,
                KeyCode::F2 => Self::F2,
                KeyCode::F3 => Self::F3,
                KeyCode::F4 => Self::F4,
                KeyCode::F5 => Self::F5,
                KeyCode::F6 => Self::F6,
                KeyCode::F7 => Self::F7,
                KeyCode::F8 => Self::F8,
                KeyCode::F9 => Self::F9,
                KeyCode::F10 => Self::F10,
                KeyCode::F11 => Self::F11,
                KeyCode::F12 => Self::F12,
                KeyCode::F13 => Self::F13,
                KeyCode::F14 => Self::F14,
                KeyCode::F15 => Self::F15,
                KeyCode::F16 => Self::F16,
                KeyCode::F17 => Self::F17,
                KeyCode::F18 => Self::F18,
                KeyCode::F19 => Self::F19,
                KeyCode::F20 => Self::F20,
                KeyCode::F21 => Self::F21,
                KeyCode::F22 => Self::F22,
                KeyCode::F23 => Self::F23,
                KeyCode::F24 => Self::F24,
                KeyCode::F25 => Self::F25,
                KeyCode::F26 => Self::F26,
                KeyCode::F27 => Self::F27,
                KeyCode::F28 => Self::F28,
                KeyCode::F29 => Self::F29,
                KeyCode::F30 => Self::F30,
                KeyCode::F31 => Self::F31,
                KeyCode::F32 => Self::F32,
                KeyCode::F33 => Self::F33,
                KeyCode::F34 => Self::F34,
                KeyCode::F35 => Self::F35,
                _ => {
                    warn!("Lumi2D detected an unidentified KeyCode! Please report this!");
                    Self::Unidentified
                }
            },
            winit::keyboard::PhysicalKey::Unidentified(scan) => {
                warn!("Lumi2D detected an unidentified KeyCode: {scan:?}\nPlease report this!");
                Self::Unidentified
            },
        }
    }
}
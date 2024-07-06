pub enum Objects {
    Rectangle { rounding: Option<Rounding>, color: u32, rect: Rect },
    Text { text: String, font: Option<String>, size: u32, color: u32, rect: Rect }
}

pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32
}

pub struct Rounding {
    pub top_l: u16,
    pub top_r: u16,
    pub bottom_l: u16,
    pub bottom_r: u16,
}

impl Rounding {
    pub fn new(top_l: u16, top_r: u16, bottom_l: u16, bottom_r: u16) -> Self {
        Self { top_l, top_r, bottom_l, bottom_r }
    }

    pub fn new_uniform(radius: u16) -> Self {
        Self::new(radius, radius, radius, radius)
    }

    pub fn new_horizontal(top: u16, bottom: u16) -> Self {
        Self::new(top, top, bottom, bottom)
    }

    pub fn new_vertical(left: u16, right: u16) -> Self {
        Self::new(left, right, left, right)
    }
}

#[allow(clippy::too_many_arguments)]
impl Objects {
    /// Shorthand function for creating an `Objects::Rectangle` with the specified properties
    pub fn rectangle(x: u32, y: u32, width: u32, height: u32, color: u32, rounding: Option<Rounding>) -> Objects {
        Objects::Rectangle { color, rounding, rect: Rect { x, y, width, height } }
    }

    /// Shorthand function for creating an `Objects::Text` with the specified properties
    pub fn text(x: u32, y: u32, width: u32, height: u32, text: String, font: Option<String>, size: u32, color: u32) -> Objects {
        Objects::Text { text, font, color, size, rect: Rect { x, y, width, height } }
    }
}
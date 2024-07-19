use super::{images::CacheableImage, svgs::CacheableSvg};

pub enum Objects {
    Rectangle { rounding: Option<Rounding>, color: u32, rect: Rect },
    Text { text: String, font: Option<String>, size: u32, color: u32, rect: Rect },
    Image { rect: Rect, image: CacheableImage },
    Svg { rect: Rect, svg: CacheableSvg, color: u32, scale: (f32, f32) }
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
    pub fn rect(x: u32, y: u32, width: u32, height: u32) -> Rect {
        Rect { x, y, width, height }
    }

    /// Shorthand function for creating an `Objects::Rectangle` with the given properties
    pub fn rectangle(x: u32, y: u32, width: u32, height: u32, color: u32, rounding: Option<Rounding>) -> Objects {
        Objects::Rectangle { color, rounding, rect: Self::rect(x, y, width, height) }
    }

    /// Shorthand function for creating an `Objects::Text` with the given properties
    pub fn text(x: u32, y: u32, width: u32, height: u32, text: String, font: Option<String>, size: u32, color: u32) -> Objects {
        Objects::Text { text, font, color, size, rect: Self::rect(x, y, width, height) }
    }

    /// Shorthand function for creating an `Objects::Image` with the given properties
    pub fn image(x: u32, y: u32, width: u32, height: u32, image: CacheableImage) -> Objects {
        Objects::Image { rect: Self::rect(x, y, width, height), image }
    }
    
    /// Shorthand function for creating an `Objects::Image` with the given properties.  
    /// Currently uses relative scaling of the SVG as skia-bindings does not support getting the size from the SVG yet :(  
    /// This means that the final size will be the base svg size multiplied by the scale
    pub fn svg_scaled(x: u32, y: u32, width: u32, height: u32, svg: CacheableSvg, color: u32, scale: (f32, f32)) -> Objects {
        Objects::Svg { rect: Self::rect(x, y, width, height), svg, color, scale }
    }
}
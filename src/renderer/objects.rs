use std::ops::Mul;

use crate::{backend::windows::{WindowTrait, Window}, structs::Position};

use super::{images::CacheableImage, svgs::CacheableSvg, text::Paragraph};

#[derive(Debug)]
pub enum Object {
    Rectangle { rounding: Option<Rounding>, color: u32, rect: Rect },
    Text { text: String, font: Option<String>, size: f32, color: u32, position: Position<i32>},
    Paragraph { paragraph: Paragraph, position: Position<u32> },
    Image { image: CacheableImage, rect: Rect },
    Svg { svg: CacheableSvg, color: u32, rect: Rect }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32
}

#[derive(Debug, Clone)]
pub struct Rounding {
    pub top_l: u16,
    pub top_r: u16,
    pub bottom_l: u16,
    pub bottom_r: u16,
}

impl Rounding {
    #[inline]
    pub fn new(top_l: u16, top_r: u16, bottom_l: u16, bottom_r: u16) -> Self {
        Self { top_l, top_r, bottom_l, bottom_r }
    }

    #[inline]
    pub fn new_uniform(radius: u16) -> Self {
        Self::new(radius, radius, radius, radius)
    }

    #[inline]
    pub fn new_horizontal(top: u16, bottom: u16) -> Self {
        Self::new(top, top, bottom, bottom)
    }

    #[inline]
    pub fn new_vertical(left: u16, right: u16) -> Self {
        Self::new(left, right, left, right)
    }
}

#[allow(clippy::too_many_arguments)]
impl Object {
    #[inline]
    pub fn rect(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect { x, y, width, height }
    }

    /// Shorthand function for creating an `Objects::Rectangle` with the given properties.
    #[inline]
    pub fn rectangle(x: i32, y: i32, width: u32, height: u32, color: u32, rounding: Option<Rounding>) -> Object {
        Object::Rectangle { color, rounding, rect: Self::rect(x, y, width, height) }
    }

    /// Shorthand function for creating an `Objects::Text` with the given properties.
    #[inline]
    pub fn text(x: i32, y: i32, text: String, font: Option<String>, size: f32, color: u32) -> Object {
        Object::Text { text, font, color, size, position: Position::new(x, y) }
    }

    /// Shorthand function for creating an `Objects::Paragraph` with the given properties.
    #[inline]
    pub fn paragraph(x: u32, y: u32, paragraph: Paragraph) -> Object {
        Object::Paragraph { position: Position::new(x, y), paragraph }
    }

    /// Shorthand function for creating an `Objects::Image` with the given properties.
    #[inline]
    pub fn image(x: i32, y: i32, width: u32, height: u32, image: CacheableImage) -> Object {
        Object::Image { image, rect: Self::rect(x, y, width, height) }
    }
    
    /// Shorthand function for creating an `Objects::Svg` with the given properties.  
    #[inline]
    pub fn svg(x: i32, y: i32, width: u32, height: u32, svg: CacheableSvg, color: u32) -> Object {
        Object::Svg { svg, color, rect: Self::rect(x, y, width, height) }
    }

    #[inline]
    pub fn scale_with(self, window: &Window) -> Self {
        self * window.current_scale()
    }
}

impl Mul<f32> for Object {
    type Output = Self;

    fn mul(self, with: f32) -> Self::Output {
        if with == 1.0 { return self; }

        match self {
            Object::Rectangle { rounding, color, rect } => Object::Rectangle {
                rounding, color, rect: rect * with
            },
            Object::Text { text, font, size, color, position } => Object::Text {
                text, font, size: size * with, color, position: position * with
            },
            Object::Paragraph { position, paragraph } => Object::Paragraph {
                position: position * with, paragraph
            },
            Object::Image { rect, image } => Object::Image {
                rect: rect * with, image
            },
            Object::Svg { rect, svg, color } => Object::Svg {
                rect: rect * with, svg, color
            }
        }
    }
}

impl Mul<f32> for Rect {
    type Output = Self;

    #[inline]
    fn mul(self, with: f32) -> Self::Output {
        let Self { x, y, width, height } = self;

        let x = (x as f32 * with).round() as i32;
        let y = (y as f32 * with).round() as i32;
        let width = (width as f32 * with).round() as u32;
        let height = (height as f32 * with).round() as u32;

        Self { x, y, width, height }
    }
}
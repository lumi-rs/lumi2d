use std::sync::Arc;

use enum_dispatch::enum_dispatch;

use super::Renderer;


#[derive(Debug, Clone)]
#[enum_dispatch(ParagraphTrait)]
pub enum Paragraph {
    #[cfg(feature = "r-skia")]
    Skia(Arc<super::skia::text::SkiaParapgraph>),
    #[cfg(feature = "r-wgpu")]
    Wgpu
}

impl Paragraph {
    pub fn new(renderer: &Renderer, text: String, width: u32, max_height: Option<u32>, options: TextOptions) -> Self {
        match renderer {
            #[cfg(feature = "r-skia")]
            Renderer::Skia(r) => Self::Skia(Arc::new(
                super::skia::text::SkiaParapgraph::new(r, text, width, max_height, options)
            ))
        }
    }
}

#[enum_dispatch]
pub trait ParagraphTrait {
    fn options(&self) -> &TextOptions;
    fn height(&self) -> u32;
}


#[derive(Debug, Clone)]
pub struct TextOptions {
    pub weight: u32,
    pub size: f32,
    pub font: Option<String>,
    pub color: u32,
    pub italic: bool,
    pub underline: bool,
    pub wrap: TextWrap,
    pub overflow: TextOverflow
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum TextWrap {
    NoWrap,
    #[default]
    WordWrap
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum TextOverflow {
    Clip,
    #[default]
    Elide
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            weight: 500,
            size: 16.0,
            font: None,
            color: 0xFFFFFFFF,
            italic: false,
            underline: false,
            wrap: TextWrap::default(),
            overflow: TextOverflow::default()
        }
    }
}
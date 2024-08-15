use std::sync::Arc;

use enum_dispatch::enum_dispatch;

use super::Renderers;


#[derive(Debug, Clone)]
#[enum_dispatch(Paragraph)]
pub enum Paragraphs {
    #[cfg(feature = "r-skia")]
    Skia(Arc<super::skia::text::SkiaParapgraph>),
    #[cfg(feature = "r-wgpu")]
    Wgpu
}

impl Paragraphs {
    pub fn new(renderer: &Renderers, text: String, width: u32, options: TextOptions) -> Self {
        match renderer {
            #[cfg(feature = "r-skia")]
            Renderers::Skia(r) => Self::Skia(Arc::new(
                super::skia::text::SkiaParapgraph::new(r, text, width, options)
            ))
        }
    }
}

#[enum_dispatch]
pub trait Paragraph {
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
    pub underline: bool
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
        }
    }
}
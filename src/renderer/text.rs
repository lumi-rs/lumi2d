use std::{num::NonZeroU32, sync::Arc};

use enum_dispatch::enum_dispatch;

use crate::backend::renderer_data::RendererData;


#[derive(Debug, Clone)]
#[enum_dispatch(ParagraphTrait)]
pub enum Paragraph {
    #[cfg(feature = "r-skia")]
    Skia(Arc<super::skia::text::SkiaParapgraph>),
    #[cfg(feature = "r-wgpu")]
    Wgpu
}

impl Paragraph {
    pub fn new(renderer_data: &RendererData, text: String, width: u32, max_height: Option<NonZeroU32>, options: TextOptions) -> Self {
        match renderer_data {
            #[cfg(feature = "r-skia")]
            RendererData::Skia(data) => Self::Skia(Arc::new(
                super::skia::text::SkiaParapgraph::new(data, text, width, max_height, options)
            )),
            _ => panic!()
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
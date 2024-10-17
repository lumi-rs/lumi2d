use std::{num::NonZeroU32, sync::Arc};

use skia_safe::{font_style::{Slant, Weight, Width}, textlayout::{self, ParagraphBuilder, ParagraphStyle, TextStyle}, Font, FontStyle};

use crate::renderer::text::{ParagraphTrait, TextOptions, TextOverflow, TextWrap};

use super::{adapter::paint, SkiaRenderer};


#[derive(Debug)]
pub struct SkiaParapgraph {
    pub options: TextOptions,
    pub(crate) paragraph: textlayout::Paragraph
}

impl SkiaParapgraph {
    pub fn new(renderer: &SkiaRenderer, text: String, width: u32, max_height: Option<NonZeroU32>, options: TextOptions) -> Self {
        let mut paragraph_style = ParagraphStyle::new();
        let mut text_style = TextStyle::new();
        let paint = paint(options.color, 0.0);
        let typeface = renderer.get_font(&options.font);

        text_style
        .set_foreground_paint(&paint)
        .set_font_size(options.size)
        .set_font_style(FontStyle::new(
            Weight::from(options.weight as i32),
            Width::NORMAL,
            if options.italic {
                Slant::Italic
            } else {
                Slant::Upright
            }
        ));

        if let Some(font) = &typeface {
            text_style.set_font_families(&[font.family_name()]);
        }
        if options.overflow == TextOverflow::Elide {
            paragraph_style.set_ellipsis("…");
        }
        
        if let Some(max_h) = max_height {
            if options.wrap == TextWrap::NoWrap {
                paragraph_style.set_max_lines(1);
            } else {
                let font = Font::from_typeface(typeface.expect("No font found"), options.size);
                let (line_height, _) = font.metrics();
                paragraph_style.set_max_lines((max_h.get() as f32 / line_height).floor() as usize);
            }
        }
        
        paragraph_style.set_text_style(&text_style);

        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, renderer.font_collection.clone());
        paragraph_builder.push_style(&text_style);
        paragraph_builder.add_text(text);

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(width as _);

        Self {
            options,
            paragraph
        }
    }
}

impl ParagraphTrait for Arc<SkiaParapgraph> {
    fn options(&self) -> &TextOptions {
        &self.options
    }
    
    fn height(&self) -> u32 {
        self.paragraph.height() as _
    }
}
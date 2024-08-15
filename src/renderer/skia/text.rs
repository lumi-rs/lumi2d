use std::sync::Arc;

use skia_safe::textlayout::{self, ParagraphBuilder, ParagraphStyle, TextStyle};

use crate::renderer::text::{Paragraph, TextOptions};

use super::{adapter::paint, SkiaRenderer};


#[derive(Debug)]
pub struct SkiaParapgraph {
    pub options: TextOptions,
    pub(crate) paragraph: textlayout::Paragraph
}

impl SkiaParapgraph {
    pub fn new(renderer: &SkiaRenderer, text: String, width: u32, options: TextOptions) -> Self {
        let paragraph_style = ParagraphStyle::new();
        let mut text_style = TextStyle::new();
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, renderer.font_collection.clone());
        let paint = paint(options.color, 0.0);

        text_style
        .set_foreground_paint(&paint)
        .set_font_size(options.size);

        if let Some(font) = renderer.get_font(options.font.clone()) {
            text_style.set_font_families(&[font.family_name()]);
        }

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

impl Paragraph for Arc<SkiaParapgraph> {
    fn options(&self) -> &TextOptions {
        &self.options
    }
    
    fn height(&self) -> u32 {
        self.paragraph.height() as _
    }
}
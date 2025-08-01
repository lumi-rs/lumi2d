use std::{num::NonZeroU32, rc::Rc};

use skia_safe::{font_arguments::{variation_position::Coordinate, VariationPosition}, font_style::{Slant, Weight, Width}, textlayout::{self, ParagraphBuilder, ParagraphStyle, TextStyle}, Font, FontArguments, FontStyle, FourByteTag};

use crate::{backend::renderer_data::skia::SkiaRendererData, renderer::text::{ParagraphTrait, TextOptions, TextOverflow, TextWrap}};

use super::adapter::paint;


#[derive(Debug)]
pub struct SkiaParapgraph {
    pub options: TextOptions,
    pub(crate) paragraph: textlayout::Paragraph
}

impl SkiaParapgraph {
    pub fn new(data: &SkiaRendererData, text: String, width: u32, max_height: Option<NonZeroU32>, options: TextOptions) -> Self {
        let mut paragraph_style = ParagraphStyle::new();
        let mut text_style = TextStyle::new();
        let paint = paint(options.color, 0.0);
        let typeface = data.get_font(&options.font);

        let var_coords = VariationPosition {
            coordinates: &make_var_coords(&[("wght", options.weight as f32)])
        };
        let arguments = FontArguments::new().set_variation_design_position(var_coords);

        if let Some(typeface) = &typeface {
            text_style.set_typeface(typeface.clone_with_arguments(&arguments));
            text_style.set_font_families(&[typeface.family_name()]);
        }

        

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
        ))
        .set_font_arguments(&arguments);


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

        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, data.font_collection.clone());
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

impl ParagraphTrait for Rc<SkiaParapgraph> {
    fn options(&self) -> &TextOptions {
        &self.options
    }
    
    fn height(&self) -> f32 {
        self.paragraph.height()
    }
}

fn make_var_coords(from: &[(&str, f32)]) -> Vec<Coordinate> {
    let coordinates: Vec<Coordinate> = from.iter().map(|(axis, val)| {
        let b = axis.as_bytes();
        let converted: u32 = *bytemuck::from_bytes(&[b[3], b[2], b[1], b[0]]);

        // dbg!(FourByteTag::new(converted), FourByteTag::from_chars('w', 'g', 'h', 't'));

        Coordinate {
            axis: FourByteTag::new(converted),
            value: *val
        }
    }).collect();

    coordinates
}
use std::{fmt::Debug, num::NonZeroU32, rc::Rc};

use skrifa::MetadataProvider;
use text_layout::{Item, ParagraphLayout};
use vello::{kurbo::Affine, peniko::Fill, Glyph, Scene};

use crate::{backend::renderer_data::vello::VelloRendererData, types::{ParagraphTrait, TextOptions, TextWrap}};

#[derive(Clone)]
pub struct VelloParagraph {
    options: TextOptions,
    scene: Scene
}

impl ParagraphTrait for Rc<VelloParagraph> {
    fn options(&self) -> &TextOptions {
        &self.options
    }

    fn height(&self) -> u32 {
        100
    }
}

impl VelloParagraph {
    pub fn new(data: &VelloRendererData, text: String, width: u32, max_height: Option<NonZeroU32>, options: TextOptions) -> Self {
        let font = data.get_font(&options.font).expect("No Font available!");

        let size = options.size;
        let should_layout = options.wrap == TextWrap::WordWrap;
        
        let layout: text_layout::KnuthPlass<f32> = text_layout::KnuthPlass::new().with_threshold(f32::INFINITY);
        let mut layout_items: Option<Vec<Item<()>>> = should_layout.then(|| Vec::with_capacity(text.len()));

        let axes = font.font_ref.axes();
        let var_loc = axes.location::<&[(&str, f32)]>(&[
            ("wght", options.weight as f32),
            ("ital", if options.italic { 1.0 } else { 0.0 }),
            ("slnt", if options.italic { -45.0 } else { 0.0 }) // TODO: Test this
        ]);

        let font_size = skrifa::instance::Size::new(size);
        let charmap = font.font_ref.charmap();
        let metrics = font.font_ref.metrics(font_size, &var_loc);
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = font.font_ref.glyph_metrics(font_size, &var_loc);
        
        let mut scene = vello::Scene::new();

        let glyphs: Vec<_> = text.chars().map(|ch| {
            let gid = charmap.map(ch).unwrap_or_default();
            let advance = glyph_metrics.advance_width(gid).unwrap_or_default();

            if let Some(items) = &mut layout_items {
                items.push(if ch.is_whitespace() && items.len() != 0 {
                    Item::Glue {
                        width: advance,
                        stretch: 1.0,
                        shrink: 0.0,
                        data: (),
                    }
                } else {
                    Item::Box {
                        width: advance,
                        data: (),
                    }
                });
            }

            (gid, advance)
        }).collect();


        let breaks = if let Some(items) = layout_items {
            layout.layout_paragraph(items.as_slice(),  width as f32)
        } else {
            Vec::new()
        };

        let start_x = 0.0;
        let mut pen_x = start_x;
        let mut pen_y = 0.0 + line_height;
        let mut break_i = 0;

        scene.draw_glyphs(&font.font)
        .hint(true)
        .font_size(size)
        .brush(super::adapter::color_from_rgba(options.color))
        .normalized_coords(bytemuck::cast_slice(var_loc.coords()))
        .draw(Fill::NonZero, glyphs.iter().enumerate().map(|(index, (id, mut advance))| {
            if breaks.len() > break_i && index == breaks[break_i].break_at {
                break_i += 1;

                pen_y += line_height;
                pen_x = start_x;
                advance = 0.0;
            }

            let x = pen_x;
            pen_x += advance;
             
            Glyph {
                id: id.to_u32(),
                x,
                y: pen_y
            }
        }));


        Self {
            options,
            scene
        }
    }

    pub fn append_to(&self, scene: &mut Scene, transform: Option<Affine>) {
        scene.append(&self.scene, transform);
    }
}


impl Debug for VelloParagraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VelloParagraph")
        .field("options", &self.options)
        .finish_non_exhaustive()
    }
}
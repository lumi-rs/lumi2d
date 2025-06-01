use skrifa::MetadataProvider;
use vello::{kurbo::{Affine, RoundedRect, RoundedRectRadii}, peniko::{color::Rgba8, BlendMode, Brush, BrushRef, Color, Fill, Font}, Glyph, Scene};
use vello_svg::usvg::Size;

use crate::{backend::renderer_data::vello::VelloRendererData, types::{Object, Rect, WindowId}};

use super::VelloRenderer;


pub(crate) fn draw_object(_renderer: &VelloRenderer, data: &VelloRendererData, scene: &mut Scene, object: &Object, scale: f32, window_id: &WindowId) {
    match object {
        Object::Rectangle { rounding, color, rect } => {
            if let Some(r) = rounding {
                let rect = RoundedRect::from_rect(
                    vello_rect(rect),
                    RoundedRectRadii::new(r.top_l.into(), r.top_r.into(), r.bottom_l.into(), r.bottom_r.into())
                );
                let color = color_from_rgba(*color);

                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    color,
                    None,
                    &rect
                );
            }
        },
        Object::Text { text, font, size, color, position } => {
            let font = data.get_font(font).expect("No Font available!");

            let size = size * scale;
            
            let axes = font.font_ref.axes();
            let var_loc = axes.location::<&[(&str, f32)]>(&[]);
            let font_size = skrifa::instance::Size::new(size);
            let charmap = font.font_ref.charmap();
            let metrics = font.font_ref.metrics(font_size, &var_loc);
            let line_height = metrics.ascent - metrics.descent + metrics.leading;
            let glyph_metrics = font.font_ref.glyph_metrics(font_size, &var_loc);
            let start_x = position.x as f32 * scale;
            let mut pen_x = start_x;
            let mut pen_y = position.y as f32 * scale + line_height;
            
            scene.draw_glyphs(&font.font)
            .hint(true)
            .font_size(size)
            .brush(color_from_rgba(*color))
            .normalized_coords(bytemuck::cast_slice(var_loc.coords()))
            .draw(Fill::NonZero, text.chars().filter_map(|ch| {
                    if ch == '\n' {
                        pen_y += line_height;
                        pen_x = start_x;
                        return None;
                    }
                    let gid = charmap.map(ch).unwrap_or_default();
                    let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                    let x = pen_x;
                    let y = pen_y;
                    pen_x += advance;

                    Some(Glyph {
                        id: gid.to_u32(),
                        x,
                        y,
                    })
            }));
        },
        Object::Paragraph { paragraph, position } => {

        },
        Object::Image { image, rect } => {

        },
        Object::Svg { svg, color, rect } => {
            let svg =  vello_svg::usvg::Tree::from_data(
                &svg.bytes(),
                &vello_svg::usvg::Options {
                    ..Default::default()
                }
            ).unwrap();

            let mut svg_scene = Scene::new();

            let (svg_width, svg_height) = (svg.size().width(), svg.size().height());
            let v_rect = vello::kurbo::Rect::from_origin_size((0.0, 0.0), (svg_width as _, svg_height as _));


            svg_scene.push_layer(
                vello::peniko::BlendMode::new(vello::peniko::Mix::Normal, vello::peniko::Compose::SrcOver),
                1.0,
                Affine::IDENTITY,
                &v_rect
            );
        
            vello_svg::append_tree(&mut svg_scene, &svg);
        
                svg_scene.push_layer(
                    vello::peniko::BlendMode::new(vello::peniko::Mix::Clip, vello::peniko::Compose::SrcIn),
                    1.0,
                    Affine::IDENTITY,
                    &v_rect
                );
            
                svg_scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    &color_from_rgba(*color),
                    None, &v_rect
                );
                svg_scene.pop_layer();
        
            svg_scene.pop_layer();
        
            let (scale_x, scale_y) = (rect.width as f32 / svg_width, rect.height as f32 / svg_height);
            scene.append(&svg_scene, Some(
                Affine::scale(scale as _)
                * Affine::translate((rect.x as f64, rect.y as f64))
                * Affine::scale_non_uniform(scale_x as _, scale_y as _)
            ));
        },
    }


}

pub fn color_from_rgba(rgba: u32) -> Color {
    let (r, g, b, a) = (rgba >> 24, rgba >> 16, rgba >> 8, rgba);
    Color::from_rgba8(r as u8, g as u8, b as u8, a as u8)
}

fn vello_rect(rect: &Rect) -> vello::kurbo::Rect {
    vello::kurbo::Rect::new(
        rect.x as f64,
        rect.y as f64,
        rect.x as f64 + rect.width as f64,
        rect.y as f64 + rect.height as f64
    )
}
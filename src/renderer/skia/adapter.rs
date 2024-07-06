use skia_safe::{Canvas, Color4f, Font, Paint, Point, RRect, Rect, TextBlob};

use crate::Objects;

use super::SkiaRenderer;

pub fn draw_object(renderer: &SkiaRenderer, canvas: &Canvas, object: Objects) {
    match object {
        Objects::Rectangle { rounding, color, rect } => {
            let paint = paint(color, 1.0);
            
            let mut rect = RRect::new_rect(
                Rect::from_point_and_size(
                    (rect.x as i32, rect.y as i32), 
                    (rect.width as i32, rect.height as i32)
                )
            );
            
            if let Some(r) = rounding {
                let radii: [Point; 4] = [
                    (r.top_l, r.top_l),
                    (r.top_r, r.top_r),
                    (r.bottom_r, r.bottom_r),
                    (r.bottom_l, r.bottom_l),
                ].map(
                    |(x, y)| (x as i32, y as i32).into()
                );

                rect.set_rect_radii(rect.rect().clone(), &radii);
            }

            canvas.draw_rrect(rect, &paint);
        },
        Objects::Text { text, font, size, color, rect } => {
            let typeface = renderer.get_font(font).unwrap();
            let paint = paint(color, 0.0);

            let mut skia_font = Font::from_typeface(typeface, size as f32);
            skia_font.set_edging(skia_safe::font::Edging::SubpixelAntiAlias);
            skia_font.set_hinting(skia_safe::FontHinting::Slight);
            skia_font.set_baseline_snap(true);
            skia_font.set_subpixel(true);

            let text_blob = TextBlob::from_str(text, &skia_font).unwrap();

            canvas.draw_text_blob(
                text_blob, 
                (rect.x as f32, (rect.y + size) as f32),
                &paint
            );
        },
    }
}

fn paint(color: u32, width: f32) -> Paint {
    let mut paint = Paint::new(rgba_to_color4f(color), None);
    paint.set_anti_alias(true);
    paint.set_style(skia_safe::PaintStyle::StrokeAndFill);
    paint.set_stroke_width(width);

    paint
}

fn rgba_to_color4f(color: u32) -> Color4f {
    let (r, g, b, a) = (color >> 24, color >> 16 & 0xFF, color >> 8 & 0xFF, color & 0xFF);
    Color4f::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
}

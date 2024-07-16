use skia_safe::{Canvas, Color4f, Font, Paint, Point, RRect, Rect, TextBlob};

use crate::{renderer::{images::CacheableImage, objects}, Objects};

use super::SkiaRenderer;

pub fn draw_object(renderer: &SkiaRenderer, canvas: &Canvas, object: Objects) {
    match object {
        Objects::Rectangle { rounding, color, rect } => {
            let paint = paint(color, 1.0);
            
            let mut rrect = RRect::new_rect(skia_rect(rect));
            
            if let Some(r) = rounding {
                let radii: [Point; 4] = [
                    (r.top_l, r.top_l),
                    (r.top_r, r.top_r),
                    (r.bottom_r, r.bottom_r),
                    (r.bottom_l, r.bottom_l),
                ].map(
                    |(x, y)| (x as i32, y as i32).into()
                );

                rrect.set_rect_radii(rrect.rect().clone(), &radii);
            }

            canvas.draw_rrect(
                rrect,
                &paint
            );
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
        Objects::Image { rect, image } => {
            let skia_image = renderer.get_or_load_image(&image);

            let lattice = skia_safe::canvas::lattice::Lattice {
                x_divs: &[],
                y_divs: &[],
                rect_types: None,
                bounds: None,
                colors: None,
            };

            canvas.draw_image_lattice(
                skia_image,
                &lattice, 
                skia_rect(rect),
                skia_safe::FilterMode::Nearest,
                None
            );
        }
    }
}

pub(crate) fn paint(color: u32, width: f32) -> Paint {
    let mut paint = Paint::new(rgba_to_color4f(color), None);
    paint.set_anti_alias(true);
    paint.set_style(skia_safe::PaintStyle::StrokeAndFill);
    paint.set_stroke_width(width);

    paint
}

pub(crate) fn skia_rect(rect: objects::Rect) -> Rect {
    Rect::from_point_and_size(
        (rect.x as i32, rect.y as i32), 
        (rect.width as i32, rect.height as i32)
    )
}

pub(crate) fn rgba_to_color4f(color: u32) -> Color4f {
    let (r, g, b, a) = (color >> 24, color >> 16 & 0xFF, color >> 8 & 0xFF, color & 0xFF);
    Color4f::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
}

pub(crate) fn image_to_skia(image: &CacheableImage) -> skia_safe::Image {
    let dimensions = image.dimensions();

    let data = unsafe { skia_safe::Data::new_bytes(&image.pixels()) };
    let image_info = skia_safe::ImageInfo::new(
        (dimensions.width as _, dimensions.height as _),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Unpremul,
        None
    );

    let skia_image = skia_safe::images::raster_from_data(
        &image_info, 
        data, 
        dimensions.width as usize * 4
    ).unwrap();

    skia_image
}
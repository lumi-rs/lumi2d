use skia_safe::{canvas::Lattice, color_filters, /*svg::Dom,*/ AlphaType, BlendMode, Canvas, Color4f, ColorType, Data, FilterMode, Font, FontMgr, ImageInfo, Paint, PaintStyle, Point, RRect, Rect, SamplingOptions, TextBlob};

use crate::{renderer::{images::{CacheableImage, PixelFormat}, objects, svgs::CacheableSvg}, Objects};

use super::SkiaRenderer;

pub(crate) fn draw_object(renderer: &SkiaRenderer, canvas: &Canvas, object: Objects) {
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

                rrect.set_rect_radii(*rrect.rect(), &radii);
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

            let lattice = Lattice {
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
                FilterMode::Nearest,
                None
            );
        },
        Objects::Svg { rect, svg, color, scale } => {
            //let dom = renderer.get_or_load_svg(&svg);
            let mut paint = paint(color, 1.0);

            let mut surface = canvas.new_surface(&canvas.image_info(), None).unwrap();
            let svg_canvas = surface.canvas();

            paint.set_color_filter(color_filters::blend(rgba_to_color4f(color).to_color(), BlendMode::SrcIn));

            svg_canvas.scale(scale);
            //dom.render(svg_canvas);
            
            surface.draw(
                canvas,
                (rect.x as f32, rect.y as f32),
                SamplingOptions::default(),
                Some(&paint)
            );
        }
    }
}

pub(crate) fn paint(color: u32, width: f32) -> Paint {
    let mut paint = Paint::new(rgba_to_color4f(color), None);
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::StrokeAndFill);
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
    let (pixels, color_type, alpha_type) = match image.format() {
        PixelFormat::RGB8 => (
            // Convert to RGBA8, as Skia does not support just RGB8
            image.pixels()
            .chunks(3)
            .flat_map(|rgb| [rgb[0], rgb[1], rgb[2], 255])
            .collect(),
            ColorType::RGBA8888,
            AlphaType::Unpremul
        ),
        PixelFormat::RGBA8 => (image.pixels(), ColorType::RGBA8888, AlphaType::Unpremul),
        PixelFormat::RGBA8Premul => (image.pixels(), ColorType::RGBA8888, AlphaType::Premul)
    };

    let data = unsafe { Data::new_bytes(&pixels) };
    let image_info = ImageInfo::new(
        (dimensions.width as _, dimensions.height as _),
        color_type,
        alpha_type,
        None
    );

    skia_safe::images::raster_from_data(
        &image_info, 
        data, 
        dimensions.width as usize * 4
    ).unwrap()
}

pub(crate) fn svg_to_skia(svg: &CacheableSvg, font_mgr: FontMgr) /*-> Dom*/ {
    //Dom::from_bytes(&svg.bytes(), font_mgr).unwrap()
}
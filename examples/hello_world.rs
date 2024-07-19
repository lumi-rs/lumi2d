use std::time::Instant;

use log::*;
use lumi2d::{backend::{windows::WindowDetails, Backend, Backends}, renderer::{images::CacheableImage, objects::Rounding, svgs::CacheableSvg, Renderer}, Objects};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_level(
        LevelFilter::Debug
    ).env().init().expect("Failed to initialize logger");

    Backends::create(|backend| {
        // TODO: Don't depend on local paths...
        let window = backend.create_window(WindowDetails {
            width: 800,
            height: 200,
            title: "Amongus".to_string(),
            ..Default::default()
        });

        let renderer = window.create_renderer().unwrap();
        let font_bytes = include_bytes!("/home/der/Programering2/yetalauncher/resources/fonts/Nunito-Medium.ttf");
        renderer.register_font(font_bytes, "Nunito");
        let mut last_frame = Instant::now();

        let image_bytes = include_bytes!("/home/der/Downloads/cat/album_2024-05-08_21-21-49.gif");
        let image = CacheableImage::from_encoded(image_bytes);

        let svg_bytes = include_bytes!("home.svg");
        let svg = CacheableSvg::new_cloned(svg_bytes);


        window.run(renderer, |_events| {
            //debug!("{:?}", Instant::now() - last_frame);
            last_frame = Instant::now();

            Vec::from([
                Objects::text(90, 100, 10, 10, "t  r  a  n  s  p  a  r  e  n  c  y".to_string(), Some("Nunito".to_string()), 22, 0x88AAFFFF),
                Objects::rectangle(100, 100, 200, 300, 0xFF9999DD, Some(Rounding::new_uniform(16))),
                Objects::svg_scaled(20, 20, 0, 0, svg.clone(), 0xFFFFFFFF, (2.0, 2.0)),
                Objects::text(20, 20, 10, 10, "Hello, world!".to_string(), None, 30, 0xFFFFFFFF),
                Objects::text(100, 400, 10, 10, "TeXt!!1".to_string(), None, 100, 0xFFFFFFFF),
                Objects::image(400, 10, image.dimensions().width / 4, image.dimensions().height / 4, image.clone()),
            ])
        });
    }).unwrap();
}
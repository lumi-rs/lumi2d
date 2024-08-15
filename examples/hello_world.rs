use std::time::Instant;

use log::*;
use lumi2d::{backend::{events::WindowEvents, windows::{BackendWindow, WindowDetails}, Backend, Backends}, renderer::{images::CacheableImage, objects::Rounding, svgs::CacheableSvg, text::Paragraph, Renderer}, Objects};
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
        renderer.register_font(font_bytes, "Nunit");
        let mut last_frame = Instant::now();

        let image_bytes = include_bytes!("/home/der/Downloads/cat/album_2024-05-08_21-21-49.gif");
        let image = CacheableImage::from_encoded(image_bytes);

        let svg_bytes = include_bytes!("home.svg");
        let svg = CacheableSvg::new_cloned(svg_bytes);

        let lorem_ipsum = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
        let paragraph = renderer.create_paragraph(lorem_ipsum.to_string(), 400, Default::default());

        backend.subscribe_events(|events| {
            let frame_time = format!("{:?}", Instant::now() - last_frame);
            last_frame = Instant::now();

            for event in events {
                match event.event {
                    WindowEvents::CloseRequested => {
                        backend.exit();
                        break;
                    },
                    WindowEvents::MouseScroll(_, y) => {
                        window.set_scale(window.current_scale() * if y > 0 { 1.05 } else { 1.0/1.05 });
                    },
                    WindowEvents::WindowSize(_) => {
                        renderer.recreate(&window)
                    },
                    _ => {}
                }
            }

            renderer.render(
                &window,
                Vec::from([
                    Objects::text(90, 100, "t  r  a  n  s  p  a  r  e  n  c  y".to_string(), Some("Nunito".to_string()), 22.0, 0x88AAFFFF),
                    Objects::rectangle(100, 100, 200, 300,  0xFF9999DD, Some(Rounding::new_uniform(16))),
                    Objects::svg_scaled(20, 200, 0, 0, svg.clone(), 0xFFFFFFFF, (2.0, 2.0)),
                    Objects::text(20, 20,  "Hello, world!".to_string(), None, 30.0, 0xFFFFFFFF),
                    Objects::text(100, 400,  "TeXt!!1".to_string(), None, 100.0, 0xFFFFFFFF),
                    Objects::image(400, 10, image.dimensions().width / 4, image.dimensions().height / 4, image.clone()),
                    Objects::text(20, 55, frame_time, None, 16.0, 0xFFFFFFFF),
                    Objects::paragraph(30, 500, paragraph.clone()),
                    Objects::text(30, 500 + paragraph.height(), paragraph.height().to_string(), None, 20.0, 0xFFFFFFFF)
                ])
            ).unwrap();
        });
    }).unwrap();
}
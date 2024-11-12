use std::time::Instant;

use log::*;
use lumi2d::prelude::*;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_level(
        LevelFilter::Debug
    ).env().init().expect("Failed to initialize logger");

    Backend::create(|backend| {
        let window = backend.create_window(WindowDetails {
            width: 800,
            height: 200,
            title: "Amogus".to_string(),
            ..Default::default()
        });

        let renderer = window.create_renderer(&backend).unwrap();
        renderer.render(&window, &backend.data(), Vec::new()).unwrap();

        let inter_font = include_bytes!("Inter-Tight.ttf");
        // Since this is the first registered font, it will be set as the default/fallback font.
        // If you want to register another font as the default, call renderer.register_default_font instead.
        backend.data().register_font(inter_font, "Inter");

        let jetbrains_font = include_bytes!("JetBrains_Mono.ttf");
        backend.data().register_font(jetbrains_font, "JetBrains Mono");

        let mut last_frame = Instant::now();

        let image_bytes = include_bytes!("nori.gif");
        let image = CacheableImage::from_encoded(image_bytes);

        let svg_bytes = include_bytes!("home.svg");
        let svg = CacheableSvg::new_cloned(svg_bytes);

        let lorem_ipsum = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
        let paragraph = backend.data().create_paragraph(lorem_ipsum.to_string(), 400, Some(120), Default::default());

        let const_objects = Vec::from([
            // Using a specific font
            Object::text(70, 100, "t r a n s p a r e n c y".to_string(), Some("JetBrains Mono".to_string()), 22.0, 0x88AAFFFF),
            Object::rectangle(100, 100, 200, 300,  0xFF9999DD, Some(Rounding::new_uniform(16))),
            Object::svg(20, 200, 80, 40, svg.clone(), 0xFFAAFFFF),
            // Using the default/fallback font
            Object::text(20, 20,  "Hello, world!".to_string(), None, 30.0, 0xFFFFFFFF),
            Object::text(100, 400,  "TeXt!!1".to_string(), None, 100.0, 0xFFFFFFFF),
            Object::image(400, 10, image.dimensions().width / 4, image.dimensions().height / 4, image.clone()),
            Object::text(30, 500 + paragraph.height() as i32, paragraph.height().to_string(), None, 20.0, 0xFFFFFFFF),
            // For multiline text
            Object::paragraph(30, 500, paragraph.clone())
        ]);

        backend.subscribe_events(|events| {
            let frame_time = format!("{:?}", Instant::now() - last_frame);
            last_frame = Instant::now();

            for event in events {
                match event {
                    Event::Backend(backend_event) => match backend_event.event.scale_with(window.current_scale()) {
                        WindowEvent::CloseRequested => {
                            backend.exit();
                            break;
                        },
                        WindowEvent::MouseScroll(_, y) => {
                            window.set_scale(window.current_scale() * if y > 0 { 1.05 } else { 1.0/1.05 });
                        },
                        WindowEvent::WindowSize(_) => {
                            renderer.recreate(&window)
                        },
                        _ => {}
                    }
                    _ => {}
                }
            }

            renderer.render(
                &window,
                &backend.data(),
                const_objects.iter()
                .chain([
                    &Object::text(20, 55, frame_time, Some("JetBrains Mono".to_string()), 16.0, 0xFFFFFFFF)
                ])
                .collect()
            ).unwrap();
        });
    }).unwrap();
}
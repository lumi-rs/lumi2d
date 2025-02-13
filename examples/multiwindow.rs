use lumi2d::prelude::*;

fn main() {
    Backend::create(|backend| {
        let mut windows: Vec<(Window, Renderer)> = Vec::new();
        let main_window = backend.create_window(WindowDetails {
            title: "Main".to_string(),
            ..Default::default()
        });
        let renderer = main_window.create_renderer(&backend).unwrap();
        
        let font_bytes = include_bytes!("/home/der/Programering2/yetalauncher/resources/fonts/Nunito-Medium.ttf");
        backend.renderer_data().register_font(font_bytes, "Nunito");

        windows.push((main_window, renderer));

        backend.subscribe_events(|events| {
            for event in &events {
                match event {
                    Event::Backend(event) => match event.event {
                        WindowEvent::MouseButton(1, KeyAction::Release) => {
                            println!("Opening window!");
                            let window = backend.create_window(WindowDetails {
                                title: (windows.len()).to_string(),
                                ..Default::default()
                            });
                            let renderer = window.create_renderer(&backend).unwrap();
                            backend.data().register_font(font_bytes, "");
            
                            windows.push((window, renderer));
                        },
                        WindowEvent::CloseRequested => {
                            let index = windows.iter().position(|(win, _)| win.id() == event.window_id).unwrap();
                            windows.remove(index).0.close(&backend.data());
                            if windows.is_empty() {
                                backend.exit();
                            }
                        },
                        WindowEvent::WindowSize(_) => {
                            windows.iter()
                            .find(|(win, _)| win.id() == event.window_id)
                            .map(|(win, renderer)| renderer.recreate(win));
                        },
                        _ => {}
                    }
                    _ => {}
                }
            }

            for (index, (window, renderer)) in windows.iter().enumerate() {
                renderer.render(
                    window,
                    &backend.data(),
                    vec![&Object::text(10, 10, index.to_string(), None, 50.0, 0xFFFFFFFF)]
                ).unwrap();
            }
        });
    }).unwrap();
}
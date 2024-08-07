use lumi2d::{backend::{events::WindowEvents, keys::KeyAction, windows::{BackendWindow, BackendWindows, WindowDetails}, Backend, Backends}, renderer::{Renderer, Renderers}, Objects};

fn main() {
    Backends::create(|backend| {
        let mut windows: Vec<(BackendWindows, Renderers)> = Vec::new();
        let main_window = backend.create_window(WindowDetails {
            title: "Main".to_string(),
            ..Default::default()
        });
        let renderer = main_window.create_renderer().unwrap();
        
        let font_bytes = include_bytes!("/home/der/Programering2/yetalauncher/resources/fonts/Nunito-Medium.ttf");
        renderer.register_font(font_bytes, "Nunito");

        windows.push((main_window, renderer));

        backend.subscribe_events(|events| {
            for event in &events {
                match event.event {
                    WindowEvents::MouseButton(1, KeyAction::Release) => {
                        println!("Opening window!");
                        let window = backend.create_window(WindowDetails {
                            title: (windows.len()).to_string(),
                            ..Default::default()
                        });
                        let renderer = window.create_renderer().unwrap();
                        renderer.register_font(font_bytes, "");
        
                        windows.push((window, renderer));
                    },
                    WindowEvents::CloseRequested => {
                        let index = windows.iter().position(|(win, _)| win.id() == event.window_id).unwrap();
                        windows.remove(index).0.close();
                        if windows.is_empty() {
                            backend.exit();
                        }
                    },
                    WindowEvents::WindowSize(_) => {
                        windows.iter()
                        .find(|(win, _)| win.id() == event.window_id)
                        .map(|(win, renderer)| renderer.recreate(win));
                    },
                    _ => {}
                }
            }

            for (index, (window, renderer)) in windows.iter().enumerate() {
                renderer.render(
                    window, 
                    vec![Objects::text(10, 10, 0, 0, index.to_string(), None, 50.0, 0xFFFFFFFF)]
                ).unwrap();
            }
        });
    }).unwrap();
}
use lumi2d::prelude::*;

fn main() {
    Backend::create(|backend| {
        let window = backend.create_window(Default::default());
        let renderer = window.create_renderer(&backend).unwrap();
        
        loop {
            for event in backend.flush_events() {
                match event {
                    Event::Backend(backend_event) => match backend_event.event {
                        WindowEvent::CloseRequested => {
                            backend.unsubscribe();
                            return;
                        },
                        WindowEvent::Redraw => {
                            renderer.recreate(&window, &backend.renderer_data());
                        },
                        _ => {}
                    },
                    _ => {}
                }

            }

            renderer.render(&window, &backend.data(), Vec::new()).unwrap();
        }
    }).unwrap();
}
use lumi2d::{backend::{events::WindowEvents, Backend, Backends}, renderer::Renderer};

fn main() {
    Backends::create(|backend| {
        let window = backend.create_window(Default::default());
        let renderer = window.create_renderer().unwrap();
        
        loop {
            for event in backend.flush_events() {
                match event.event {
                    WindowEvents::CloseRequested => {
                        backend.exit();
                        break;
                    },
                    WindowEvents::Redraw => {
                        renderer.recreate(&window);
                    },
                    _ => {}
                }
            }

            renderer.render(&window, Vec::new()).unwrap();
        }
    }).unwrap();
}
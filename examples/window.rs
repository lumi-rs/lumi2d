use lumi2d::{backend::{events::WindowEvent, BackendTrait, Backend}, renderer::RendererTrait};

fn main() {
    Backend::create(|backend| {
        let window = backend.create_window(Default::default());
        let renderer = window.create_renderer().unwrap();
        
        loop {
            for event in backend.flush_events() {
                match event.event {
                    WindowEvent::CloseRequested => {
                        backend.exit();
                        break;
                    },
                    WindowEvent::Redraw => {
                        renderer.recreate(&window);
                    },
                    _ => {}
                }
            }

            renderer.render(&window, Vec::new()).unwrap();
        }
    }).unwrap();
}
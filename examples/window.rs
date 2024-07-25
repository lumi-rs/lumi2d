use lumi2d::backend::{events::WindowEvents, Backend, Backends};

fn main() {
    Backends::create(|backend| {
        let window = backend.create_window(Default::default());
        let renderer = window.create_renderer().unwrap();

        window.run(&renderer, |events| {
            if events.contains(&WindowEvents::CloseRequested) {
                backend.exit();
            }
            Vec::new()
        });
    }).unwrap();
}
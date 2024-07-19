use lumi2d::{backend::{Backend, Backends}, renderer::Renderers};

fn main() {
    Backends::create(|backend| {
        let window = backend.create_window(Default::default());
        let renderer = Renderers::create(&window).unwrap();

        window.run(renderer, |_events| Vec::new());
    }).unwrap();
}
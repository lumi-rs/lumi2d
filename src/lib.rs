pub mod backend;
pub mod renderer;

use std::sync::OnceLock;

pub use renderer::objects::Objects;


const VSYNC: OnceLock<bool> = OnceLock::new();

pub fn vsync() -> bool {
    *VSYNC.get_or_init(|| {
        let env = std::env::var("LUMI_VSYNC").or_else(|_| std::env::var("LUMI2D_VSYNC"));

        env.is_err() || env.is_ok_and(|val| match val.as_str() {
            "false" | "no" | "off" | "0" => false,
            _ => true
        })
    })
}
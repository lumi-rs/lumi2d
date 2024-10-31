pub mod backend;
pub mod renderer;
pub mod structs;

use std::sync::OnceLock;

pub use renderer::objects::Object;
pub use backend::{Backend, BackendTrait, windowing::window::{Window, WindowTrait, WindowDetails}};


static VSYNC: OnceLock<bool> = OnceLock::new();
static POLLING: OnceLock<bool> = OnceLock::new();

pub fn vsync() -> bool {
    *VSYNC.get_or_init(|| {
        let env = std::env::var("LUMI_VSYNC").or_else(|_| std::env::var("LUMI2D_VSYNC"));

        !env.is_ok_and(|val| matches!(val.as_str(), "false" | "no" | "off" | "0"))
    })
}

pub fn polling() -> bool {
    *POLLING.get_or_init(|| {
        let env = std::env::var("LUMI_POLLING").or_else(|_| std::env::var("LUMI2D_POLLING"));

        env.is_ok_and(|val| matches!(val.as_str(), "true" | "yes" | "on" | "1"))
    })
}
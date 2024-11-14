pub mod backend;
pub mod renderer;
pub mod structs;

use std::sync::OnceLock;

pub mod types {
    pub use crate::renderer::{Renderer, RendererType, RResult, objects::*, images::*, svgs::*, text::*, errors::*};
    pub use crate::backend::{Backend, BackendType, BResult, events::*, keys::*, windowing::{*, window::*}, renderer_data::RendererData};
    pub use crate::structs::*;
}

pub mod traits {
    pub use crate::{
        backend::{BackendTrait, windowing::window::WindowTrait, renderer_data::RendererDataTrait},
        renderer::{RendererTrait, text::ParagraphTrait}
    };
}
pub mod prelude {
    pub use crate::{types::*, traits::*};
}

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
use std::path::Path;

use enum_dispatch::enum_dispatch;
use log::warn;
use strum::{EnumIter, IntoEnumIterator};


pub mod errors;
pub mod objects;

#[cfg(feature = "r-wgpu")]
pub mod wgpu;
#[cfg(feature = "r-skia")]
pub mod skia;


use crate::{backend::windows::BackendWindows, Objects};

use self::errors::RendererError;
#[cfg(feature = "r-wgpu")]
use self::wgpu::WgpuRenderer;
#[cfg(feature = "r-skia")]
use self::skia::SkiaRenderer;

pub type RResult<T> = core::result::Result<T, RendererError>;


#[derive(Debug, EnumIter)]
pub enum RendererTypes {
    #[cfg(feature = "r-wgpu")]
    Wgpu,
    #[cfg(feature = "r-skia")]
    Skia,
}

impl Default for RendererTypes {
    fn default() -> Self {
        RendererTypes::iter().next().expect("Lumi2D was compiled without any enabled renderers!")
    }
}

#[enum_dispatch(Renderer)]
pub enum Renderers {
    #[cfg(feature = "r-wgpu")]
    Wgpu(WgpuRenderer),
    #[cfg(feature = "r-skia")]
    Skia(SkiaRenderer),
}

impl Renderers {
    pub fn create(window: &BackendWindows) -> RResult<Renderers> {
        let backends = RendererTypes::iter();
        for typ in backends {
            match Self::create_type(&typ, window) {
                Ok(backend) => return Ok(backend),
                Err(err) => warn!("Error initalizing Skia {typ:?} backend: {err}; attempting next backend..."),
            }
        }
        Err(RendererError::NoRenderer)
    }

    pub fn create_type(typ: &RendererTypes, window: &BackendWindows) -> RResult<Renderers> {
        Ok(match typ {
            #[cfg(feature = "r-skia")]
            RendererTypes::Skia => {
                Renderers::Skia(SkiaRenderer::new(window)?)
            },
        })
    }
}

#[enum_dispatch]
pub trait Renderer {
    fn render(&self, window: &BackendWindows, objects: Vec<Objects>) -> RResult<()>;
    fn recreate(&self, window: &BackendWindows);
    /// Register a font to be used with the given alias
    fn register_font(&self, font_file: impl AsRef<Path>, alias: &str);
    fn register_default_font(&self, font_file: impl AsRef<Path>, alias: &str);
}


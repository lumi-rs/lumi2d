use enum_dispatch::enum_dispatch;
use log::warn;
use strum::{EnumIter, IntoEnumIterator};


pub mod errors;
pub mod objects;
pub mod images;
pub mod svgs;
pub mod text;

#[cfg(feature = "r-wgpu")]
pub mod wgpu;
#[cfg(feature = "r-vello")]
pub mod vello;
#[cfg(feature = "r-skia")]
pub mod skia;


use crate::{backend::{renderer_data::RendererData, windowing::window::Window}, types::{Backend, Object}};

use self::errors::RendererError;

pub type RResult<T> = core::result::Result<T, RendererError>;


#[derive(Debug, EnumIter, Clone, Copy)]
pub enum RendererType {
    #[cfg(feature = "r-wgpu")]
    Wgpu,
    #[cfg(feature = "r-vello")]
    Vello,
    #[cfg(feature = "r-skia")]
    Skia,
}

impl Default for RendererType {
    fn default() -> Self {
        RendererType::iter().next().expect("Lumi2D was compiled without any enabled renderers!")
    }
}

#[derive(Debug)]
#[enum_dispatch(RendererTrait)]
pub enum Renderer {
    #[cfg(feature = "r-wgpu")]
    Wgpu(self::wgpu::WgpuRenderer),
    #[cfg(feature = "r-vello")]
    Vello(self::vello::VelloRenderer),
    #[cfg(feature = "r-skia")]
    Skia(self::skia::SkiaRenderer),
}

impl Renderer {
    pub fn create<T>(backend: &Backend<T>, window: &Window) -> RResult<Renderer> {
        let renderers = RendererType::iter();
        for typ in renderers {
            let created = Self::create_type(&typ, window, &backend.renderer_data());
            match created {
                Ok(renderer) => {
                    backend.transform_renderer_data(&renderer);
                    return Ok(renderer)
                },
                Err(err) => warn!("Error initalizing Skia {typ:?} backend: {err}; attempting next backend..."),
            }
        }
        Err(RendererError::NoRenderer)
    }

    pub fn create_type(typ: &RendererType, window: &Window, renderer_data: &RendererData) -> RResult<Renderer> {
        Ok(match typ {
            #[cfg(feature = "r-wgpu")]
            RendererType::Wgpu => {
                unimplemented!()
            },
            #[cfg(feature = "r-vello")]
            RendererType::Vello => {
                self::vello::VelloRenderer::new(window, renderer_data)?.into()
            },
            #[cfg(feature = "r-skia")]
            RendererType::Skia => {
                self::skia::SkiaRenderer::new(window)?.into()
            }
        })
    }
}

#[enum_dispatch]
pub trait RendererTrait {
    fn render(&self, window: &Window, data: &RendererData, objects: Vec<&Object>) -> RResult<()>;
    fn recreate(&self, window: &Window, data: &RendererData);
    fn transform_data(&self, data: &RendererData) -> Option<RendererData>;
}

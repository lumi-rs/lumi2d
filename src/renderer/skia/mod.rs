use crate::{backend::{renderer_data::RendererData, windowing::window::{Window, WindowTrait}}, types::Object};

use super::{errors::RendererError, RResult, RendererTrait};

pub mod errors;
pub mod adapter;
pub mod text;

#[cfg(feature = "skia-opengl")]
pub mod opengl;
#[cfg(feature = "skia-vulkan")]
pub mod vulkan;


use enum_dispatch::enum_dispatch;
use errors::SkiaRendererError;
use log::warn;
use skia_safe::{Canvas, Color4f};
use strum::{EnumIter, IntoEnumIterator};

#[cfg(feature = "skia-opengl")]
use opengl::SkiaOpenGLSurface;
#[cfg(feature = "skia-vulkan")]
use vulkan::SkiaVulkanBackend;


#[derive(Debug)]
pub struct SkiaRenderer {
    skia_backend: SkiaRenderingBackends,
}

impl SkiaRenderer {
    pub fn new(window: &Window) -> RResult<Self> {
        Ok(SkiaRenderer {
            skia_backend: SkiaRenderingBackends::create(window)?
        })
    }

}

impl RendererTrait for SkiaRenderer {
    fn render(&self, window: &Window, data: &RendererData, objects: Vec<&Object>) -> RResult<()> {
        self.skia_backend.render(window, |canvas: &Canvas| {
            canvas.draw_color(Color4f::new(0.1, 0.1, 0.1, 1.0), None);

            let scale = window.current_scale();
            canvas.scale((scale, scale));

            let skia_data = data.try_as_skia_ref().unwrap();

            for object in objects {
                adapter::draw_object(self, skia_data, canvas, object, scale, window.id());
            }
        })
    }

    fn recreate(&self, window: &Window) {
        self.skia_backend.recreate(window)
    }
}

#[derive(Debug, EnumIter)]
pub enum SkiaRenderingBackendTypes {
    #[cfg(feature = "skia-vulkan")]
    Vulkan,
    #[cfg(feature = "skia-d3d")]
    D3D,
    #[cfg(feature = "skia-metal")]
    Metal,
    #[cfg(feature = "skia-opengl")]
    OpenGL,
}


#[derive(Debug)]
#[enum_dispatch(SkiaRenderingBackend)]
pub enum SkiaRenderingBackends {
    #[cfg(feature = "skia-vulkan")]
    Vulkan(SkiaVulkanBackend),
    #[cfg(feature = "skia-d3d")]
    D3D,
    #[cfg(feature = "skia-metal")]
    Metal,
    #[cfg(feature = "skia-opengl")]
    OpenGL,
}

impl SkiaRenderingBackends {
    pub fn create(window: &Window) -> RResult<SkiaRenderingBackends> {
        let backends = SkiaRenderingBackendTypes::iter();
        for typ in backends {
            match Self::create_type(&typ, window) {
                Ok(backend) => return Ok(backend),
                Err(err) => warn!("Error initalizing Skia {typ:?} backend: {err}; attempting next backend..."),
            }
        }
        Err(RendererError::Skia(SkiaRendererError::NoBackend))
    }

    pub fn create_type(typ: &SkiaRenderingBackendTypes, window: &Window) -> RResult<SkiaRenderingBackends> {
        Ok(match typ {
            #[cfg(feature = "skia-vulkan")]
            SkiaRenderingBackendTypes::Vulkan => {
                SkiaRenderingBackends::Vulkan(SkiaVulkanBackend::new(window)?)
            },
            #[cfg(feature = "skia-d3d")]
            SkiaRenderingBackendTypes::D3D => todo!(),
            #[cfg(feature = "skia-metal")]
            SkiaRenderingBackendTypes::Metal => todo!(),
            #[cfg(feature = "skia-opengl")]
            SkiaRenderingBackendTypes::OpenGL => todo!(),
        })
    }
}


#[enum_dispatch]
pub trait SkiaRenderingBackend {
    fn render(&self, window: &Window, canvas: impl FnOnce(&Canvas)) -> RResult<()>;
    fn recreate(&self, window: &Window);
}


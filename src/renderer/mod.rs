use std::num::NonZeroU32;

use enum_dispatch::enum_dispatch;
use images::CacheableImage;
use log::warn;
use strum::{EnumIter, IntoEnumIterator};
use svgs::CacheableSvg;
use text::{Paragraph, TextOptions};


pub mod errors;
pub mod objects;
pub mod images;
pub mod svgs;
pub mod text;

#[cfg(feature = "r-wgpu")]
pub mod wgpu;
#[cfg(feature = "r-skia")]
pub mod skia;


use crate::{backend::windows::Window, Object};

use self::errors::RendererError;

pub type RResult<T> = core::result::Result<T, RendererError>;


#[derive(Debug, EnumIter)]
pub enum RendererType {
    #[cfg(feature = "r-wgpu")]
    Wgpu,
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
    #[cfg(feature = "r-skia")]
    Skia(self::skia::SkiaRenderer),
}

impl Renderer {
    pub fn create(window: &Window) -> RResult<Renderer> {
        let backends = RendererType::iter();
        for typ in backends {
            match Self::create_type(&typ, window) {
                Ok(backend) => return Ok(backend),
                Err(err) => warn!("Error initalizing Skia {typ:?} backend: {err}; attempting next backend..."),
            }
        }
        Err(RendererError::NoRenderer)
    }

    pub fn create_type(typ: &RendererType, window: &Window) -> RResult<Renderer> {
        Ok(match typ {
            #[cfg(feature = "r-skia")]
            RendererType::Skia => {
                Renderer::Skia(self::skia::SkiaRenderer::new(window)?)
            },
        })
    }

    pub fn create_paragraph(&self, text: String, width: u32, max_height: Option<u32>, options: TextOptions) -> Paragraph {
        let max_h = max_height.and_then(NonZeroU32::new);
        
        Paragraph::new(self, text, width, max_h, options)
    }
}

#[enum_dispatch]
pub trait RendererTrait {
    fn render(&self, window: &Window, objects: Vec<&Object>) -> RResult<()>;
    fn recreate(&self, window: &Window);
    /// Register a font to be used with the given alias
    fn register_font(&self, bytes: &[u8], alias: &str);
    /// Register a font to be used with the given alias, and set it as the deafult font.  
    /// If this is not called, the default font will be the first one registered.
    fn register_default_font(&self, bytes: &[u8], alias: &str);
    /// Preload an image into the Renderer's image cache. Not required to be called manually.
    fn load_image(&self, image: &CacheableImage);
    /// Remove an image from the Renderer's cache. Needs to be called manually if the image should not be loaded permanently (for now).
    fn unload_image(&self, image: &CacheableImage);
    /// Preload an SVG into the Renderer's SVG cache. Not required to be called manually.
    fn load_svg(&self, svg: &CacheableSvg);
    /// Remove an SVG from the Renderer's cache. Needs to be called manually if the SVG should not be loaded permanently (for now).
    fn unload_svg(&self, svg: &CacheableSvg);
}

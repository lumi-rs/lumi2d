use std::num::NonZeroU32;

use enum_dispatch::enum_dispatch;
use placeholder::PlaceholderRendererData;

use crate::renderer::{images::CacheableImage, svgs::CacheableSvg, text::{Paragraph, TextOptions}, Renderer};


pub mod placeholder;
#[cfg(feature = "r-skia")]
pub mod skia;


#[derive(Debug, strum::EnumTryAs)]
#[enum_dispatch(RendererDataTrait)]
pub enum RendererData {
    Placeholder(placeholder::PlaceholderRendererData),
    #[cfg(feature = "r-skia")]
    Skia(skia::SkiaRendererData),
    #[cfg(feature = "r-wgpu")]
    Wgpu
}

impl RendererData {
    pub fn placeholder() -> Self {
        RendererData::Placeholder(PlaceholderRendererData::new())
    }

    pub fn create_paragraph(&self, text: String, width: u32, max_height: Option<u32>, options: TextOptions) -> Paragraph {
        let max_h = max_height.and_then(NonZeroU32::new);
        
        Paragraph::new(self, text, width, max_h, options)
    }
}


#[enum_dispatch]
pub trait RendererDataTrait {
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
    /// Internal function
    fn transform_with(&self, renderer: &Renderer) -> Option<RendererData>;
}
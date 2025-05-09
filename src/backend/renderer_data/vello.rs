use crate::types::{CacheableImage, CacheableSvg, Renderer, WindowId};

use super::{RendererData, RendererDataTrait};

#[derive(Debug)]
pub struct VelloRendererData {}

impl RendererDataTrait for VelloRendererData {
    fn register_font(&self, bytes: &[u8], alias: &str) {
        todo!()
    }

    fn register_default_font(&self, bytes: &[u8], alias: &str) {
        todo!()
    }

    fn load_image(&self, image: &CacheableImage) {
        todo!()
    }

    fn unload_image(&self, image: &CacheableImage) {
        todo!()
    }

    fn load_svg(&self, svg: &CacheableSvg) {
        todo!()
    }

    fn unload_svg(&self, svg: &CacheableSvg) {
        todo!()
    }

    fn transform_with(&self, renderer: &Renderer) -> Option<RendererData> {
        todo!()
    }

    fn remove_window_data(&self, window_id: &WindowId) {
        todo!()
    }
}

use std::{cell::{RefCell}, collections::HashMap, fmt::Debug, mem::ManuallyDrop};

use log::warn;
use skrifa::{raw::FileRef, FontRef};
use vello::{peniko::{Blob, Font}, util::RenderContext};

use crate::types::{CacheableImage, CacheableSvg, WindowId};

use super::{RendererDataTrait};

pub struct VelloRendererData {
    pub context: RefCell<ManuallyDrop<RenderContext>>,
    pub fonts: RefCell<HashMap<String, VelloFont>>,
    pub default_font_alias: RefCell<String>
}

#[derive(Clone)]
pub struct VelloFont {
    pub font: Font,
    pub font_ref: FontRef<'static>
}

impl VelloRendererData {
    pub fn new(context: RenderContext) -> Self {
        let context = RefCell::new(ManuallyDrop::new(context));

        Self {
            context,
            fonts: RefCell::new(HashMap::new()),
            default_font_alias: RefCell::new(String::new())
        }
    }

    pub fn get_font(&self, alias: &Option<String>) -> Option<VelloFont> {
        let fonts = self.fonts.borrow();

        alias
        .as_ref()
        .map(|al| fonts.get(al).cloned().or_else(|| {
            warn!("Unregistered font: {al}! Please register it first with RendererData::register_font");
            None
        }))
        .flatten()
        .or_else(|| fonts.get(self.default_font_alias.borrow().as_str()).cloned())
    }
}

impl RendererDataTrait for VelloRendererData {
    fn register_font(&self, bytes: &[u8], alias: &str) {
        let font = Font::new(Blob::from(Vec::from_iter(bytes.iter().copied())), 0);

        let file_ref = FileRef::new(font.data.data()).unwrap();
        let font_ref = match file_ref {
            FileRef::Font(font_ref) => font_ref,
            FileRef::Collection(collection_ref) => collection_ref.get(0).unwrap(),
        };

        let vello_font = VelloFont {
            font: font.clone(),
            font_ref: unsafe { std::mem::transmute(font_ref) }
        };

        let mut default_alias = self.default_font_alias.borrow_mut();
        if *default_alias == "" {
            *default_alias = alias.to_string();
        }

        self.fonts.borrow_mut().insert(
            alias.to_string(),
            vello_font
        );
    }

    fn register_default_font(&self, bytes: &[u8], alias: &str) {
        *self.default_font_alias.borrow_mut() = alias.to_string();
        self.register_font(bytes, alias);
    }

    fn load_image(&self, _image: &CacheableImage) {
        todo!()
    }

    fn unload_image(&self, _image: &CacheableImage) {
        todo!()
    }

    fn load_svg(&self, _svg: &CacheableSvg) {
        todo!()
    }

    fn unload_svg(&self, _svg: &CacheableSvg) {
        todo!()
    }

    fn remove_window_data(&self, _window_id: &WindowId) {
        // Do nothing
    }
}


impl Debug for VelloRendererData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VelloRendererData")
        .finish_non_exhaustive()
    }
}
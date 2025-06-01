use std::{cell::{Cell, RefCell}, collections::HashMap, fmt::Debug, mem::ManuallyDrop, sync::Arc};

use log::warn;
use skrifa::{raw::{FileRef, TableProvider}, FontRef};
use vello::{peniko::{Blob, Font}, util::RenderContext};

use crate::types::{CacheableImage, CacheableSvg, Renderer, WindowId};

use super::{RendererData, RendererDataTrait};

pub struct VelloRendererData {
    pub context: RefCell<ManuallyDrop<RenderContext>>,
    pub fonts: RefCell<HashMap<String, VelloFont>>,
    pub default_font_index: Cell<isize>
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
            default_font_index: Cell::new(-1)
        }
    }

    pub fn get_font(&self, alias: &Option<String>) -> Option<VelloFont> {
        let fonts = self.fonts.borrow();

        if let Some(alias) = alias {
            fonts.get(alias).or_else(|| {
                warn!("Unregistered font: {alias}! Please register it first with RendererData::register_font");
                None
            }).cloned()
        } else {
            fonts.values().next().cloned()
        }
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

        self.fonts.borrow_mut().insert(
            alias.to_string(),
            vello_font
        );
    }

    fn register_default_font(&self, bytes: &[u8], alias: &str) {
        self.default_font_index.set(self.fonts.borrow().len() as isize);
        self.register_font(bytes, alias);
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

    fn remove_window_data(&self, window_id: &WindowId) {
        // Do nothing
    }
}


impl Debug for VelloRendererData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VelloRendererData")
        .finish_non_exhaustive()
    }
}
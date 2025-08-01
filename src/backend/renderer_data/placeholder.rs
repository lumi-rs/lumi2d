use std::cell::{Cell, RefCell};


use crate::{renderer::{images::CacheableImage, svgs::CacheableSvg}, types::WindowId};

use super::RendererDataTrait;


#[derive(Debug)]
pub struct PlaceholderRendererData {
    pub fonts: RefCell<Vec<(String, Vec<u8>)>>,
    pub default_index: Cell<isize>,
    // These will be dynamically cached when needed, so no need to transfer them from here
    // images: RefCell<HashMap<Uuid, CacheableImage>>,
    // svgs: RefCell<HashMap<Uuid, CacheableSvg>>
}

impl PlaceholderRendererData {
    pub fn new() -> Self {
        Self {
            fonts: RefCell::new(Vec::new()),
            default_index: Cell::new(-1),
            // images: RefCell::new(HashMap::new()),
            // svgs: RefCell::new(HashMap::new())
        }
    }
}

impl RendererDataTrait for PlaceholderRendererData {
    fn register_font(&self, bytes: &[u8], alias: &str) {
        self.fonts.borrow_mut().push((alias.to_string(), bytes.to_vec()));
        if self.default_index.get() == -1 {
            self.default_index.set(self.fonts.borrow().len() as isize - 1);
        }
    }

    fn register_default_font(&self, bytes: &[u8], alias: &str) {
        self.fonts.borrow_mut().push((alias.to_string(), bytes.to_vec()));
        self.default_index.set(self.fonts.borrow().len() as isize - 1);
    }

    fn load_image(&self, _image: &CacheableImage) {
        // self.images.borrow_mut().insert(*image.uuid(), image.clone());
    }

    fn unload_image(&self, _image: &CacheableImage) {
        // self.images.borrow_mut().remove(image.uuid());
    }

    fn load_svg(&self, _svg: &CacheableSvg) {
        // self.svgs.borrow_mut().insert(*svg.uuid(), svg.clone());
    }

    fn unload_svg(&self, _svg: &CacheableSvg) {
        // self.svgs.borrow_mut().remove(svg.uuid());
    }

    /*
    fn transform_with(&self, renderer: &Renderer) -> Option<RendererData> {
        match renderer {
            #[cfg(feature = "r-wgpu")]
            Renderer::Wgpu(_) => {
                unimplemented!()
            },
            #[cfg(feature = "r-vello")]
            Renderer::Vello(_) => {
                unimplemented!()
            },
            #[cfg(feature = "r-skia")]
            Renderer::Skia(_skia_renderer) => {
                use skia_safe::{textlayout::{FontCollection, TypefaceFontProvider}, FontMgr};
                use super::skia::SkiaRendererData;

                let mut font_collection = FontCollection::new();
                let font_mgr = FontMgr::new();
                let font_provider = TypefaceFontProvider::new();
                font_collection.set_default_font_manager(Some(font_provider.clone().into()), None);

                let new = SkiaRendererData {
                    font_map: RefCell::new(HashMap::new()),
                    font_mgr,
                    font_collection,
                    font_provider,
                    default_font: RefCell::new(None),
                    image_cache: RefCell::new(HashMap::new()),
                    svg_cache: RefCell::new(HashMap::new())
                };

                let default_index = self.default_index.get() as usize;
                for (index, (alias, bytes)) in self.fonts.borrow_mut().drain(..).enumerate() {
                    if index == default_index {
                        new.register_default_font(&bytes, &alias);
                    } else {
                        new.register_font(&bytes, &alias);
                    }
                }

                Some(RendererData::Skia(new))
            },
        }
    }
    */

    fn remove_window_data(&self, _window_id: &WindowId) {
        // Nothing needed here
    }
}
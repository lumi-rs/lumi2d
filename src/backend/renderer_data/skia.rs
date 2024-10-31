use std::{cell::RefCell, collections::HashMap};

use skia_safe::{svg::Dom, textlayout::{FontCollection, TypefaceFontProvider}, wrapper::PointerWrapper, Canvas, FontMgr, Typeface};
use uuid::Uuid;

use crate::renderer::{images::CacheableImage, skia::adapter, svgs::CacheableSvg};

use super::RendererDataTrait;


#[derive(Debug)]
pub struct SkiaRendererData {
    pub font_map: RefCell<HashMap<String, Typeface>>,
    pub font_mgr: FontMgr,
    pub font_collection: FontCollection,
    pub font_provider: TypefaceFontProvider,
    pub default_font: RefCell<Option<Typeface>>,
    pub image_cache: RefCell<HashMap<Uuid, skia_safe::Image>>,
    pub(crate) svg_cache: RefCell<HashMap<Uuid, SkiaCachedSvg>>
}

impl SkiaRendererData {
    pub fn new() -> Self {
        let mut font_collection = FontCollection::new();
        let font_mgr = FontMgr::new();
        let font_provider = TypefaceFontProvider::new();
        font_collection.set_default_font_manager(Some(font_provider.clone().into()), None);

        Self {
            font_map: RefCell::new(HashMap::new()),
            font_mgr,
            font_collection,
            font_provider,
            default_font: RefCell::new(None),
            image_cache: RefCell::new(HashMap::new()),
            svg_cache: RefCell::new(HashMap::new())
        }
    }

    pub fn get_font(&self, alias: &Option<String>) -> Option<Typeface> {
        if let Some(alias) = alias {
            if let Some(font) = self.font_map.borrow().get(alias) {
                Some(font.clone())
            } else {
                self.default_font.borrow().clone()
            }  
        } else {
            self.default_font.borrow().clone()
        }
    }

    pub fn get_or_load_image(&self, image: &CacheableImage) -> skia_safe::Image {
        let mut cache = self.image_cache.borrow_mut();

        if let Some(i) = cache.get(image.uuid()) {
            i.clone()
        } else {
            let skia_image = adapter::image_to_skia(image);
    
            cache.insert(*image.uuid(), skia_image.clone());
            skia_image
        }
    }

    pub(crate) fn get_or_load_svg(&self, svg: &CacheableSvg, canvas: &Canvas, width: u32, height: u32) -> SvgWithSurface {
        let mut cache = self.svg_cache.borrow_mut();

        if let Some(cached_svg) = cache.get_mut(svg.uuid()) {
            match cached_svg {
                SkiaCachedSvg::Surface(svgws) => {
                    if (svgws.width, svgws.height) != (width, height) {
                        *svgws = svg_dom_to_with_surface(svgws.dom.clone(), canvas, width, height)
                    }

                    svgws.clone()
                },
                SkiaCachedSvg::Dom(dom) => {
                    let svg_with_surface = svg_dom_to_with_surface(dom.clone(), canvas, width, height);
                    *cached_svg = SkiaCachedSvg::Surface(svg_with_surface.clone());
                    svg_with_surface
                },
            }
        } else {
            let dom = adapter::svg_to_skia(svg, self.get_font_mgr());
    
            let svg_with_surface = svg_dom_to_with_surface(dom, canvas, width, height);

            cache.insert(*svg.uuid(), SkiaCachedSvg::Surface(svg_with_surface.clone()));
            svg_with_surface
        }
    }

    pub fn get_font_mgr(&self) -> FontMgr {
        self.font_mgr.clone()
    }

    
}

impl RendererDataTrait for SkiaRendererData {
    fn register_font(&self, bytes: &[u8], alias: &str) {
        let typeface = self.font_mgr.new_from_data(bytes, None).unwrap();
        self.font_provider.clone().register_typeface(typeface.clone(), alias);

        if self.default_font.borrow().is_none() {
            self.default_font.replace(Some(typeface.clone()));
        }

        self.font_map.borrow_mut().insert(
            alias.to_string(),
            typeface
        );
    }

    fn register_default_font(&self, bytes: &[u8], alias: &str) {
        let typeface = self.font_mgr.new_from_data(bytes, None).unwrap();
        self.font_provider.clone().register_typeface(typeface.clone(), alias);

        self.default_font.replace(Some(typeface.clone()));

        self.font_map.borrow_mut().insert(
            alias.to_string(),
            typeface
        );
    }

    fn load_image(&self, image: &CacheableImage) {
        let skia_image = adapter::image_to_skia(image);

        self.image_cache.borrow_mut().insert(*image.uuid(), skia_image);
    }

    fn unload_image(&self, image: &CacheableImage) {
        self.image_cache.borrow_mut().remove(image.uuid());
    }

    fn load_svg(&self, svg: &CacheableSvg) {
        let dom = adapter::svg_to_skia(svg, self.get_font_mgr());
        
        self.svg_cache.borrow_mut().insert(*svg.uuid(), SkiaCachedSvg::Dom(dom));
    }

    fn unload_svg(&self, svg: &CacheableSvg) {
        self.svg_cache.borrow_mut().remove(svg.uuid());
    }

    fn transform_with(&self, _renderer: &crate::renderer::Renderer) -> Option<super::RendererData> {
        None
    }
}



#[derive(Debug, Clone)]
pub(crate) enum SkiaCachedSvg {
    Dom(Dom),
    Surface(SvgWithSurface)
}

#[derive(Debug, Clone)]
pub(crate) struct SvgWithSurface {
    pub dom: Dom,
    pub surface: skia_safe::Surface,
    pub width: u32,
    pub height: u32
}

// Amazing function name, I know...
fn svg_dom_to_with_surface(dom: Dom, canvas: &Canvas, width: u32, height: u32) -> SvgWithSurface {
    let size = dom.inner().fContainerSize;

    let mut surface = canvas.new_surface(&canvas.image_info(), None).unwrap();
    let svg_canvas = surface.canvas();

    svg_canvas.scale((width as f32 / size.fWidth, height as f32 / size.fHeight));
    dom.render(svg_canvas);

    SvgWithSurface {
        dom,
        surface,
        width,
        height
    }
}
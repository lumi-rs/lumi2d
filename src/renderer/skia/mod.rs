use std::{cell::RefCell, collections::HashMap};

use crate::{backend::windows::{WindowTrait, Window}, Object};

use super::{errors::RendererError, images::CacheableImage, svgs::CacheableSvg, RResult, RendererTrait};

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
use skia_safe::{svg::Dom, textlayout::{FontCollection, TypefaceFontProvider}, wrapper::PointerWrapper, Canvas, Color4f, FontMgr, Typeface};
use strum::{EnumIter, IntoEnumIterator};
use uuid::Uuid;

#[cfg(feature = "skia-opengl")]
use opengl::SkiaOpenGLSurface;
#[cfg(feature = "skia-vulkan")]
use vulkan::SkiaVulkanBackend;


#[derive(Debug)]
pub struct SkiaRenderer {
    skia_backend: SkiaRenderingBackends,
    font_map: RefCell<HashMap<String, Typeface>>,
    font_mgr: FontMgr,
    font_collection: FontCollection,
    font_provider: TypefaceFontProvider,
    default_font: RefCell<Option<Typeface>>,
    image_cache: RefCell<HashMap<Uuid, skia_safe::Image>>,
    svg_cache: RefCell<HashMap<Uuid, SkiaCachedSvg>>
}

impl SkiaRenderer {
    pub fn new(window: &Window) -> RResult<Self> {
        let mut font_collection = FontCollection::new();
        let font_mgr = FontMgr::new();
        let font_provider = TypefaceFontProvider::new();
        font_collection.set_default_font_manager(Some(font_provider.clone().into()), None);

        Ok(SkiaRenderer {
            skia_backend: SkiaRenderingBackends::create(window)?,
            font_map: RefCell::new(HashMap::new()),
            font_mgr,
            font_collection,
            font_provider,
            default_font: RefCell::new(None),
            image_cache: RefCell::new(HashMap::new()),
            svg_cache: RefCell::new(HashMap::new())
        })
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

    pub(in super) fn get_or_load_svg(&self, svg: &CacheableSvg, canvas: &Canvas, width: u32, height: u32) -> SvgWithSurface {
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

impl RendererTrait for SkiaRenderer {
    fn render(&self, window: &Window, objects: Vec<&Object>) -> RResult<()> {
        self.skia_backend.render(window, |canvas: &Canvas| {
            canvas.draw_color(Color4f::new(0.1, 0.1, 0.1, 1.0), None);

            let scale = window.current_scale();
            canvas.scale((scale, scale));

            for object in objects {
                adapter::draw_object(self, canvas, object, scale);
            }
        })
    }

    fn recreate(&self, window: &Window) {
        self.skia_backend.recreate(window)
    }

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



#[derive(Debug, Clone)]
pub(in super) enum SkiaCachedSvg {
    Dom(Dom),
    Surface(SvgWithSurface)
}

#[derive(Debug, Clone)]
pub(in super) struct SvgWithSurface {
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
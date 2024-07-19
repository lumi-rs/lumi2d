use std::{cell::RefCell, collections::HashMap};

use crate::{backend::windows::BackendWindows, Objects};

use super::{errors::RendererError, images::CacheableImage, svgs::CacheableSvg, RResult, Renderer};

pub mod errors;
pub mod adapter;

#[cfg(feature = "skia-opengl")]
pub mod opengl;
#[cfg(feature = "skia-vulkan")]
pub mod vulkan;


use enum_dispatch::enum_dispatch;
use errors::SkiaRendererError;
use log::warn;
use skia_safe::{Canvas, Color4f, FontMgr, Typeface};
use strum::{EnumIter, IntoEnumIterator};
use uuid::Uuid;

#[cfg(feature = "skia-opengl")]
use opengl::SkiaOpenGLSurface;
#[cfg(feature = "skia-vulkan")]
use vulkan::SkiaVulkanBackend;


pub struct SkiaRenderer {
    skia_backend: SkiaRenderingBackends,
    font_map: RefCell<HashMap<String, Typeface>>,
    font_mgr: FontMgr,
    default_font: RefCell<Option<Typeface>>,
    image_cache: RefCell<HashMap<Uuid, skia_safe::Image>>,
//    svg_cache: RefCell<HashMap<Uuid, skia_safe::svg::Dom>>
}

impl SkiaRenderer {
    pub fn new(window: &BackendWindows) -> RResult<Self> {
        Ok(SkiaRenderer {
            skia_backend: SkiaRenderingBackends::create(window)?,
            font_map: RefCell::new(HashMap::new()),
            font_mgr: FontMgr::new(),
            default_font: RefCell::new(None),
            image_cache: RefCell::new(HashMap::new()),
//            svg_cache: RefCell::new(HashMap::new())
        })
    }

    pub fn get_font(&self, alias: Option<String>) -> Option<Typeface> {
        if let Some(alias) = alias {
            if let Some(font) = self.font_map.borrow().get(&alias) {
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
/*
    pub fn get_or_load_svg(&self, svg: &CacheableSvg) -> skia_safe::svg::Dom {
        let mut cache = self.svg_cache.borrow_mut();

        if let Some(i) = cache.get(svg.uuid()) {
            i.clone()
        } else {
            let skia_svg = adapter::svg_to_skia(svg, self.get_font_mgr());
    
            cache.insert(*svg.uuid(), skia_svg.clone());
            skia_svg
        }
    } */

    pub fn get_font_mgr(&self) -> FontMgr {
        self.font_mgr.clone()
    }
}

impl Renderer for SkiaRenderer {
    fn render(&self, window: &BackendWindows, objects: Vec<Objects>) -> RResult<()> {
        self.skia_backend.render(window, |canvas: &Canvas| {
            canvas.draw_color(Color4f::new(0.1, 0.1, 0.1, 1.0), None);

            for object in objects {
                adapter::draw_object(self, canvas, object);
            }
        })
    }

    fn recreate(&self, window: &BackendWindows) {
        self.skia_backend.recreate(window)
    }

    fn register_font(&self, bytes: &[u8], alias: &str) {
        let typeface = self.font_mgr.new_from_data(bytes, None).unwrap();

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
        let skia_svg = adapter::svg_to_skia(svg, self.get_font_mgr());

//        self.svg_cache.borrow_mut().insert(*svg.uuid(), skia_svg);
    }

    fn unload_svg(&self, svg: &CacheableSvg) {
//        self.svg_cache.borrow_mut().remove(svg.uuid());
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
    pub fn create(window: &BackendWindows) -> RResult<SkiaRenderingBackends> {
        let backends = SkiaRenderingBackendTypes::iter();
        for typ in backends {
            match Self::create_type(&typ, window) {
                Ok(backend) => return Ok(backend),
                Err(err) => warn!("Error initalizing Skia {typ:?} backend: {err}; attempting next backend..."),
            }
        }
        Err(RendererError::Skia(SkiaRendererError::NoBackend))
    }

    pub fn create_type(typ: &SkiaRenderingBackendTypes, window: &BackendWindows) -> RResult<SkiaRenderingBackends> {
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
    fn render(&self, window: &BackendWindows, canvas: impl FnOnce(&Canvas)) -> RResult<()>;
    fn recreate(&self, window: &BackendWindows);
}
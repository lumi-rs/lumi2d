use std::{cell::RefCell, collections::HashMap, fs, path::Path};

use crate::{backend::windows::BackendWindows, Objects};

use super::{errors::RendererError, RResult, Renderer};

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

#[cfg(feature = "skia-opengl")]
use opengl::SkiaOpenGLSurface;
#[cfg(feature = "skia-vulkan")]
use vulkan::SkiaVulkanBackend;


pub struct SkiaRenderer {
    skia_backend: SkiaRenderingBackends,
    font_map: RefCell<HashMap<String, Typeface>>,
    font_mgr: FontMgr,
    default_font: RefCell<Option<Typeface>>
}

impl SkiaRenderer {
    pub fn new(window: &BackendWindows) -> RResult<Self> {
        Ok(SkiaRenderer {
            skia_backend: SkiaRenderingBackends::create(window)?,
            font_map: RefCell::new(HashMap::new()),
            font_mgr: FontMgr::new(),
            default_font: RefCell::new(None)
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

    fn register_font(&self, font_file: impl AsRef<Path>, alias: &str) {
        let contents = fs::read(font_file).unwrap();
        let typeface = self.font_mgr.new_from_data(&contents, None).unwrap();

        if self.default_font.borrow().is_none() {
            self.default_font.replace(Some(typeface.clone()));
        }

        self.font_map.borrow_mut().insert(
            alias.to_string(),
            typeface
        );
    }

    fn register_default_font(&self, font_file: impl AsRef<Path>, alias: &str) {
        let contents = fs::read(font_file).unwrap();
        let typeface = self.font_mgr.new_from_data(&contents, None).unwrap();

        self.default_font.replace(Some(typeface.clone()));

        self.font_map.borrow_mut().insert(
            alias.to_string(),
            typeface
        );
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
use std::{ffi::c_void, sync::{RwLock, RwLockReadGuard}};

use enum_dispatch::enum_dispatch;
use renderer_data::{RendererData, RendererDataTrait};
use strum::{EnumIter, IntoEnumIterator};
use windowing::{window::BackendEvent, WindowBackend};

use crate::renderer::Renderer;

use self::{errors::BackendError, windowing::window::{Window, WindowDetails}};

#[cfg(feature = "b-glfw")]
use windowing::glfw::*;
#[cfg(feature = "b-winit")]
use windowing::winit::*;

pub mod renderer_data;
pub mod windowing;
pub mod events;
pub mod keys;
pub mod errors;


pub type BResult<T> = Result<T, BackendError>;


#[derive(Debug, EnumIter)]
pub enum BackendType {
    #[cfg(feature = "b-winit")]
    Winit,
    #[cfg(feature = "b-glfw")]
    Glfw,
}

impl Default for BackendType {
    fn default() -> Self {
        BackendType::iter().next().expect("Lumi2D was compiled without any enabled backends!")
    }
}

#[derive(Debug)]
pub struct Backend {
    window_backend: WindowBackend,
    // An Optional so it can be taken out for transforming, should never actually be 'None'
    renderer_data: RwLock<RendererData>
}

#[enum_dispatch]
pub trait BackendTrait {
    fn create_window(&self, info: WindowDetails) -> Window;
    fn gl_proc_address(&self, proc_address: &str) -> *const c_void;
    fn exit(&self);
    fn subscribe_events(&self, callback: impl FnMut(Vec<BackendEvent>));
    fn flush_events(&self) -> Vec<BackendEvent>;
}

impl Backend {
    pub fn create(callback: impl FnOnce(Backend) + Copy + Send + 'static) -> BResult<()> {
        WindowBackend::create(move |window_backend| {
            let backend = Self {
                window_backend,
                renderer_data: RwLock::new(RendererData::placeholder())
            };

            callback(backend);
        })
    }
    
    pub fn data(&self) -> RwLockReadGuard<RendererData> {
        self.renderer_data.read().unwrap()
    }

    pub fn renderer_data(&self) -> RwLockReadGuard<RendererData> {
        self.renderer_data.read().unwrap()
    }

    pub fn transform_renderer_data(&self, renderer: &Renderer) {
        let mut data = self.renderer_data.write().unwrap();

        if let Some(new) = data.transform_with(renderer) {
            *data = new;
        };
    }
}

impl BackendTrait for Backend {
    fn create_window(&self, info: WindowDetails) -> Window {
        self.window_backend.create_window(info)
    }

    fn gl_proc_address(&self, proc_address: &str) ->  *const c_void {
        self.window_backend.gl_proc_address(proc_address)
    }

    fn exit(&self) {
        self.window_backend.exit()
    }

    fn subscribe_events(&self, callback: impl FnMut(Vec<BackendEvent>)) {
        self.window_backend.subscribe_events(callback)
    }

    fn flush_events(&self) -> Vec<BackendEvent> {
        self.window_backend.flush_events()
    }
}
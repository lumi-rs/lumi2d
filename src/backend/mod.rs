use std::{cell::{Ref, RefCell}, ffi::c_void};

use crossbeam_channel::Sender;
use enum_dispatch::enum_dispatch;
use events::Event;
use renderer_data::RendererData;
use strum::{EnumIter, IntoEnumIterator};
use windowing::WindowBackend;

use crate::{renderer::Renderer, traits::RendererTrait};

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


#[derive(Debug, EnumIter, Clone, Copy)]
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
pub struct Backend<T> {
    renderer_data: RefCell<RendererData>,
    window_backend: WindowBackend<T>
}

#[enum_dispatch]
pub trait BackendTrait<T> {
    fn create_window(&self, info: WindowDetails) -> Window;
    fn gl_proc_address(&self, proc_address: &str) -> *const c_void;
    fn subscribe_events(&self, callback: impl FnMut(Vec<Event<T>>));
    fn unsubscribe(&self);
    fn flush_events(&self) -> Vec<Event<T>>;
    fn send_event(&self, event: Event<T>);
    fn send_custom(&self, custom_event: T) {
        self.send_event(Event::Custom(custom_event));
    }
    fn sender(&self) -> Sender<Event<T>>;
}


impl Backend<()> {
    pub fn create(callback: impl FnOnce(Backend<()>) + Copy + Send + 'static) -> BResult<()> {
        Self::create_custom(callback)
    }
}

impl<T> Backend<T> {
    pub fn create_custom(callback: impl FnOnce(Backend<T>) + Copy + Send + 'static) -> BResult<()> {
        WindowBackend::create(move |window_backend| {
            let backend = Self {
                window_backend,
                renderer_data: RefCell::new(RendererData::placeholder())
            };

            callback(backend);
        })
    }
    
    pub fn data(&self) -> Ref<RendererData> {
        self.renderer_data.borrow()
    }

    pub fn renderer_data(&self) -> Ref<RendererData> {
        self.renderer_data.borrow()
    }

    pub fn transform_renderer_data(&self, renderer: &Renderer) {
        let mut data = self.renderer_data.borrow_mut();

        if let Some(new) = renderer.transform_data(&data) {
            *data = new;
        };
    }
}

impl<T> BackendTrait<T> for Backend<T> {
    fn create_window(&self, info: WindowDetails) -> Window {
        self.window_backend.create_window(info)
    }

    fn gl_proc_address(&self, proc_address: &str) -> *const c_void {
        self.window_backend.gl_proc_address(proc_address)
    }

    fn subscribe_events(&self, callback: impl FnMut(Vec<Event<T>>)) {
        self.window_backend.subscribe_events(callback)
    }

    fn unsubscribe(&self) {
        self.window_backend.unsubscribe();
    }

    fn flush_events(&self) -> Vec<Event<T>> {
        self.window_backend.flush_events()
    }

    fn send_event(&self, event: Event<T>) {
        self.window_backend.send_event(event)
    }

    fn sender(&self) -> Sender<Event<T>> {
        self.window_backend.sender()
    }
}
use std::ffi::c_void;

use enum_dispatch::enum_dispatch;
use log::*;
use strum::{EnumIter, IntoEnumIterator};
use windows::BackendEvent;

use self::{errors::{BackendError, BackendInitError}, windows::{Window, WindowDetails}};
#[cfg(feature = "b-glfw")]
use self::glfw::*;
#[cfg(feature = "b-winit")]
use self::winit::*;


pub mod windows;
pub mod events;
pub mod keys;
pub mod errors;
#[cfg(feature = "b-winit")]
pub mod winit;
#[cfg(feature = "b-winit")]
pub mod winit_window;
#[cfg(feature = "b-glfw")]
pub mod glfw;


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
#[enum_dispatch(BackendTrait)]
pub enum Backend {
    #[cfg(feature = "b-winit")]
    Winit(WinitBackend),
    #[cfg(feature = "b-glfw")]
    Glfw(GlfwBackend)
}

impl Backend {
    pub fn create(callback: impl FnOnce(Backend) + Copy + Send + 'static) -> BResult<()> {
        let backends = BackendType::iter();
        for typ in backends {
            match Self::create_type(&typ, callback) {
                Ok(()) => return Ok(()),
                Err(err) => warn!("Error initalizing {typ:?} backend: {err}; attempting next backend..."),
            }
        }
        Err(BackendError::Init(BackendInitError::NoBackend))
    }

    pub fn create_type(backend: &BackendType, callback: impl FnOnce(Backend) + Send + 'static) -> BResult<()> {
        match backend {
            #[cfg(feature = "b-glfw")]
            BackendType::Glfw => {
                GlfwBackend::create(callback)?;
            },
            #[cfg(feature = "b-winit")]
            BackendType::Winit => {
                WinitBackend::create(callback)?;
            }
        }
        Ok(())
    }
}

#[enum_dispatch]
pub trait BackendTrait {
    fn create_window(&self, info: WindowDetails) -> Window;
    fn gl_proc_address(&self, proc_address: &str) -> *const c_void;
    fn exit(&self);
    fn subscribe_events(&self, callback: impl FnMut(Vec<BackendEvent>));
    fn flush_events(&self) -> Vec<BackendEvent>;
}
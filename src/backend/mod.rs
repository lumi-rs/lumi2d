use std::ffi::c_void;

use enum_dispatch::enum_dispatch;
use log::*;
use strum::{EnumIter, IntoEnumIterator};

use self::{errors::{BackendError, BackendInitError}, windows::{BackendWindows, WindowDetails}};
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
#[cfg(feature = "b-glfw")]
pub mod glfw;


pub type BResult<T> = Result<T, BackendError>;


#[derive(Debug, EnumIter)]
pub enum BackendTypes {
    #[cfg(feature = "b-winit")]
    Winit,
    #[cfg(feature = "b-glfw")]
    Glfw,
}

impl Default for BackendTypes {
    fn default() -> Self {
        BackendTypes::iter().next().expect("Lumi2D was compiled without any enabled backends!")
    }
}


#[derive(Debug)]
#[enum_dispatch(Backend)]
pub enum Backends {
    #[cfg(feature = "b-winit")]
    Winit(WinitBackend),
    #[cfg(feature = "b-glfw")]
    Glfw(GlfwBackend),
}

impl Backends {
    pub fn create(callback: impl FnOnce(Backends) + Copy + Send + 'static) -> BResult<()> {
        let backends = BackendTypes::iter();
        for typ in backends {
            match Self::create_type(&typ, callback) {
                Ok(()) => return Ok(()),
                Err(err) => warn!("Error initalizing {typ:?} backend: {err}; attempting next backend..."),
            }
        }
        Err(BackendError::Init(BackendInitError::NoBackend))
    }


    pub fn create_type(backend: &BackendTypes, callback: impl FnOnce(Backends) + Send + 'static) -> BResult<()> {
        match backend {
            #[cfg(feature = "b-glfw")]
            BackendTypes::Glfw => {
                GlfwBackend::create(callback)?;
            },
            #[cfg(feature = "b-winit")]
            BackendTypes::Winit => {
                WinitBackend::create(callback)?;
            }
        }
        Ok(())
    }
}

#[enum_dispatch]
pub trait Backend {
    fn create_window(&self, info: WindowDetails) -> BackendWindows;
    fn gl_proc_address(&self, proc_address: &str) -> *const c_void;
    fn exit(&self);
}
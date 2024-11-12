pub mod window;

#[cfg(feature = "b-winit")]
pub mod winit;
#[cfg(feature = "b-winit")]
pub mod winit_window;
#[cfg(feature = "b-glfw")]
pub mod glfw;

use enum_dispatch::enum_dispatch;
use log::*;
use strum::IntoEnumIterator;

#[cfg(feature = "b-glfw")]
use self::glfw::*;
#[cfg(feature = "b-winit")]
use self::winit::*;

use super::{errors::{BackendError, BackendInitError}, BResult, BackendType};

#[derive(Debug)]
#[enum_dispatch(BackendTrait<T>)]
pub enum WindowBackend<T> {
    #[cfg(feature = "b-winit")]
    Winit(WinitBackend<T>),
    #[cfg(feature = "b-glfw")]
    Glfw(GlfwBackend<T>)
}

impl<T> WindowBackend<T> {
    pub fn create(callback: impl FnOnce(WindowBackend<T>) + Copy + Send + 'static) -> BResult<()> {
        let backends = BackendType::iter();
        for typ in backends {
            match Self::create_type(&typ, callback) {
                Ok(()) => return Ok(()),
                Err(err) => warn!("Error initalizing {typ:?} backend: {err}; attempting next backend..."),
            }
        }
        Err(BackendError::Init(BackendInitError::NoBackend))
    }

    pub fn create_type(backend: &BackendType, callback: impl FnOnce(WindowBackend<T>) + Send + 'static) -> BResult<()> {
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
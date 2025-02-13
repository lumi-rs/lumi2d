use enum_dispatch::enum_dispatch;
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};

use crate::{renderer::{RResult, Renderer}, structs::Dimensions, types::{Backend, RendererData}};



#[derive(Debug, Clone)]
pub struct WindowDetails {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub mode: WindowModes
}

impl Default for WindowDetails {
    fn default() -> Self {
        Self {
            width: 800, height: 600, title: String::from("Lumi2D Window"), mode: WindowModes::Maximized
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WindowModes {
    Fullscreen,
    Borderless,
    Maximized,
    Windowed
}

#[derive(Debug)]
#[enum_dispatch(WindowTrait)]
pub enum Window {
    #[cfg(feature = "b-glfw")]
    GlfwWindow(super::glfw::GlfwWindow),
    #[cfg(feature = "b-winit")]
    WinitWindow(super::winit_window::WinitWindow),
}

impl Window {
    pub fn create_renderer<T>(&self, backend: &Backend<T>) -> RResult<Renderer> {
        Renderer::create(backend, self)
    }
}

#[enum_dispatch]
pub trait WindowTrait {
    fn handles(&self) -> Result<WindowHandles, HandleError>;
    fn physical_dimensions(&self) -> Dimensions;
    fn dimensions(&self) -> Dimensions;
    fn set_mode(&self, window_mode: WindowModes);
    fn target_scale(&self) -> f32;
    fn current_scale(&self) -> f32;
    fn set_scale(&self, scale: f32);
    fn id(&self) -> WindowId;
    fn close(self, renderer_data: &RendererData);
}



#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum WindowId {
    #[cfg(feature = "b-winit")]
    Winit(winit::window::WindowId)
}



#[derive(Debug, Clone)]
pub struct WindowHandles<'a> {
    window: WindowHandle<'a>,
    display: DisplayHandle<'a>
}

impl<'a> WindowHandles<'a> {
    pub fn from(window: &'a(impl HasWindowHandle + HasDisplayHandle)) -> Result<Self, HandleError> {
        Ok(Self {
            window: window.window_handle()?,
            display: window.display_handle()?
        })
    }
}

impl HasWindowHandle for WindowHandles<'_> {
    fn window_handle(&self) -> Result<WindowHandle, HandleError> {
        Ok(self.window)
    }
}

impl HasDisplayHandle for WindowHandles<'_> {
    fn display_handle(&self) -> Result<DisplayHandle, HandleError> {
        Ok(self.display)
    }
}

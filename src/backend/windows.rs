use enum_dispatch::enum_dispatch;
use log::warn;
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};

use crate::{renderer::{RResult, Renderer, Renderers}, Objects};

use super::events::WindowEvents;
#[cfg(feature = "b-glfw")]
use super::GlfwWindow;
#[cfg(feature = "b-winit")]
use super::WinitWindow;



#[derive(Debug)]
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

#[derive(Debug)]
pub enum WindowModes {
    Fullscreen,
    Borderless,
    Maximized,
    Windowed
}

#[derive(Debug)]
#[enum_dispatch(BackendWindow)]
pub enum BackendWindows<'a> {
    #[cfg(feature = "b-glfw")]
    GlfwWindow(GlfwWindow<'a>),
    #[cfg(feature = "b-winit")]
    WinitWindow(WinitWindow<'a>),
}

impl BackendWindows<'_> {
    pub fn run(&self, renderer: &Renderers, mut frame_callback: impl FnMut(Vec<WindowEvents>) -> Vec<Objects>) {
        loop {
            let events = self.flush_events();
            
            if events.contains(&WindowEvents::Redraw) {
                renderer.recreate(self);
            }

            let objects = frame_callback(events);
            if let Err(err) = renderer.render(self, objects) {
                warn!("Rendering error occured: {err}");
            };
        }
    }

    pub fn create_renderer(&self) -> RResult<Renderers> {
        Renderers::create(self)
    }
}

#[enum_dispatch]
pub trait BackendWindow {
    fn handles(&self) -> Result<WindowHandles, HandleError>;
    fn physical_dimensions(&self) -> Dimensions;
    fn dimensions(&self) -> Dimensions;
    fn set_mode(&self, window_mode: WindowModes);
    fn flush_events(&self) -> Vec<WindowEvents>;
    fn target_scale(&self) -> f32;
    fn current_scale(&self) -> f32;
    fn set_scale(&self, scale: f32);
    fn send_event(&self, event: WindowEvents);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32
}

impl Dimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
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

impl<'a> HasWindowHandle for WindowHandles<'a> {
    fn window_handle(&self) -> Result<WindowHandle, HandleError> {
        Ok(self.window)
    }
}

impl<'a> HasDisplayHandle for WindowHandles<'a> {
    fn display_handle(&self) -> Result<DisplayHandle, HandleError> {
        Ok(self.display)
    }
}

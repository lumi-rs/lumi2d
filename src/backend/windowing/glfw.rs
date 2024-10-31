use std::{cell::RefCell, ffi::c_void};

use glfw::{Glfw, WindowEvent};
use log::*;
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasRawWindowHandle, HasWindowHandle, RawWindowHandle, WindowHandle};

use super::{events::WindowEvents, windows::{BackendWindow, BackendWindows, Dimensions, WindowDetails, WindowHandles}, Backend};


#[derive(Debug)]
pub struct GlfwBackend {
    glfw: RefCell<Glfw>
}

impl GlfwBackend {
    pub fn create() -> Self {
        let glfw = glfw::init_no_callbacks().unwrap();

        GlfwBackend { glfw: RefCell::new(glfw) }
    }
}

impl Backend for GlfwBackend {
    fn create_window(&self, det: WindowDetails) -> BackendWindows {
        let (window, events) = self.glfw.borrow_mut().create_window(det.width, det.height, &det.title, glfw::WindowMode::Windowed).unwrap();
        
        BackendWindows::GlfwWindow(GlfwWindow { backend: &self, window, events })
    }

    fn gl_proc_address(&self, proc_name: &str) -> *const c_void {
        self.glfw.borrow().get_proc_address_raw(proc_name)
    }
}


#[derive(Debug)]
pub struct GlfwWindow<'backend> {
    backend: &'backend GlfwBackend,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>
}

impl BackendWindow for GlfwWindow<'_> {
    fn handles(&self) -> Result<WindowHandles, HandleError> {
        WindowHandles::from(&self.window)
    }
    
    fn dimensions(&self) -> super::windows::Dimensions {
        let (w, h) = self.window.get_framebuffer_size();
        Dimensions::new(w as _, h as _)
    }

    fn flush_events(&self) -> Vec<WindowEvents> {
        let events = glfw::flush_messages(&self.events);
        
        for event in events {
            debug!("{:?}", event)
        }

        self.backend.glfw.borrow_mut().poll_events();
        Vec::new()
    }
}
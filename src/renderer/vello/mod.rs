use std::{cell::{Cell, RefCell}, fmt::Debug};

use vello::{peniko::color::AlphaColor, util::{RenderContext, RenderSurface}, AaConfig, Scene};

use crate::{backend::renderer_data::vello::VelloRendererData, traits::RendererDataTrait, types::{Dimensions, Object, RResult, RendererData, Window, WindowTrait}};

use super::RendererTrait;


pub mod text;
pub mod adapter;


pub struct VelloRenderer {
    scene: RefCell<Scene>,
    surface: RefCell<RenderSurface<'static>>,
    renderer: RefCell<vello::Renderer>,
    temp_context: Cell<Option<RenderContext>>
}

impl VelloRenderer {
    pub fn new(window: &Window, renderer_data: &RendererData) -> RResult<Self> {
        let scene = RefCell::new(Scene::new());

        let mut temp_context = None;
        let render_context = if let RendererData::Vello(vello) = renderer_data {
            &mut vello.context.borrow_mut()
        } else {
            temp_context = Some(RenderContext::new());
            temp_context.as_mut().unwrap()
        };

        let Dimensions { width, height} = window.physical_dimensions();
        let handles = window.handles().unwrap();
        let surface_future = render_context.create_surface(
            handles,
            width,
            height,
            vello::wgpu::PresentMode::Fifo
        );
        let surface = pollster::block_on(surface_future).unwrap();

        let renderer = RefCell::new(Self::create_vello_renderer(render_context, &surface));

        Ok(VelloRenderer {
            scene,
            renderer,
            surface: unsafe { std::mem::transmute(RefCell::new(surface)) },
            temp_context: Cell::new(temp_context)
        })
    }

    fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> vello::Renderer {
        vello::Renderer::new(
            &render_cx.devices[surface.dev_id].device,
            vello::RendererOptions::default(),
        )
        .expect("Couldn't create renderer")
    }
}

impl RendererTrait for VelloRenderer {
    fn render(&self, window: &Window, data: &RendererData, objects: Vec<&Object>) -> RResult<()> {
        let mut scene = self.scene.borrow_mut();
        scene.reset();

        let Dimensions { width, height } = window.physical_dimensions();
        if width == 0 || height == 0 { return Ok(()) } // Skip frame if window size is zero (e.g. when minimized)
        let device_handle = &data.try_as_vello_ref().unwrap().context.borrow().devices[self.surface.borrow().dev_id];

        let data = data.try_as_vello_ref().unwrap();
        let scale = window.current_scale();
        let window_id = window.id();

        for object in objects {
            adapter::draw_object(self, data, &mut scene, object, scale, &window_id);
        }

        self.renderer.borrow_mut()
        .render_to_texture(
            &device_handle.device,
            &device_handle.queue,
            &scene,
            &self.surface.borrow().target_view,
            &vello::RenderParams {
                base_color: AlphaColor::new([0.1, 0.1, 0.1, 1.0]), // Background color
                width,
                height,
                antialiasing_method: AaConfig::Area,
            },
        ).unwrap();

        // Get the surface's texture
        let surface_texture = self.surface.borrow()
            .surface
            .get_current_texture()
            .expect("failed to get surface texture");

        // Perform the copy
        let mut encoder = device_handle
            .device
            .create_command_encoder(&vello::wgpu::CommandEncoderDescriptor {
                label: Some("Surface Blit"),
            });
        self.surface.borrow().blitter.copy(
            &device_handle.device,
            &mut encoder,
            &self.surface.borrow().target_view,
            &surface_texture
                .texture
                .create_view(&vello::wgpu::TextureViewDescriptor::default()),
        );
        device_handle.queue.submit([encoder.finish()]);
        // Queue the texture to be presented on the surface
        surface_texture.present();

        device_handle.device.poll(vello::wgpu::Maintain::Poll);

        Ok(())
    }

    fn recreate(&self, window: &Window, data: &RendererData) {
        let Dimensions { width, height } = window.physical_dimensions();

        data
        .try_as_vello_ref()
        .unwrap()
        .context
        .borrow()
        .resize_surface(&mut self.surface.borrow_mut(), width, height);
    }

    fn transform_data(&self, data: &RendererData) -> Option<RendererData> {
        return match data {
            RendererData::Placeholder(placeholder) => {
                let context = self.temp_context
                .take()
                .unwrap_or_else(|| RenderContext::new());

                let new = VelloRendererData::new(context);
                
                let default_index = placeholder.default_index.get() as usize;
                for (index, (alias, bytes)) in placeholder.fonts.borrow_mut().drain(..).enumerate() {
                    if index == default_index {
                        new.register_default_font(&bytes, &alias);
                    } else {
                        new.register_font(&bytes, &alias);
                    }
                }

                Some(new.into())
            },
            _ => None
        }
    }
}


impl Debug for VelloRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VelloRenderer")
        .finish_non_exhaustive()
    }
}
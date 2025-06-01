use std::{cell::{Cell, RefCell}, fmt::Debug, sync::Arc};

use log::*;
use skia_safe::{gpu::{vk::BackendContext, DirectContext}, Canvas, SurfaceProps, SurfacePropsFlags};
use vulkano::{
    device::{physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags},
    image::{view::ImageView, Image, ImageUsage}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    swapchain::{PresentMode, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo},
    sync::{self, GpuFuture},
    format::Format,
    Handle, Validated, VulkanError, VulkanLibrary, VulkanObject
};
use crate::{backend::windowing::window::{WindowTrait, Window}, renderer::{errors::RendererError, skia::errors::VulkanErr, RResult}};

use super::SkiaRenderingBackend;


const IMAGE_FORMAT: Format = Format::R8G8B8A8_UNORM;
const SKIA_FORMAT: skia_safe::gpu::vk::Format = skia_safe::gpu::vk::Format::R8G8B8A8_UNORM;
const SKIA_TYPE: skia_safe::ColorType = skia_safe::ColorType::RGBA8888;


pub struct SkiaVulkanBackend {
    direct_context: RefCell<DirectContext>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    recreate_swapchain: Cell<bool>,
    swapchain: RefCell<Arc<Swapchain>>,
    swapchain_images: RefCell<Vec<Arc<Image>>>,
    swapchain_image_views: RefCell<Vec<Arc<ImageView>>>,
    gpu_future: RefCell<Option<Box<dyn GpuFuture>>>,
}

impl SkiaVulkanBackend {
    pub fn new(window: &Window) -> RResult<SkiaVulkanBackend> {
        let handles = window.handles().or(Err(RendererError::WindowHandles))?;
        let dim = window.physical_dimensions();
        let present_mode = if crate::vsync() { PresentMode::Fifo } else { PresentMode::Immediate };

        let vulkan = VulkanLibrary::new().map_err(VulkanErr::InitLibrary)?;
        let req_extensions = Surface::required_extensions(&handles).or(Err(RendererError::WindowHandles))?;
        debug!("Using Vulkan extensions: {:?}", req_extensions);

        let instance = Instance::new(
            vulkan.clone(), 
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: req_extensions,
                application_name: Some("Lumi2D".to_string()),
                ..Default::default()
            }
        ).map_err(VulkanErr::InitInstance)?;

        let surface = unsafe {
            Surface::from_window_ref(instance.clone(), &handles)
            .map_err(VulkanErr::CreateSurface)?
        };

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .map_err(VulkanErr::Other)?
        .filter(|device| device.supported_extensions().contains(&device_extensions))
        .filter_map(|device| {
            device.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, queue)| {
                    queue.queue_flags.intersects(QueueFlags::GRAPHICS)
                    && device.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (device, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::IntegratedGpu => 0,
            PhysicalDeviceType::DiscreteGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .ok_or(VulkanErr::NoDevice)?;

        debug!(
            "Using Vulkan device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        ).or(Err(VulkanErr::NoDevice))?;

        let queue = queues.next().ok_or(VulkanErr::NoQueue)?;
        let surface_capabilities = device
        .physical_device()
        .surface_capabilities(&surface, Default::default())
        .unwrap();
        // let image_format = device
        // .physical_device()
        // .surface_formats(&surface, Default::default())
        // .unwrap()[0]
        // .0;

        let (swapchain, images) = {
            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    image_format: IMAGE_FORMAT,
                    present_mode,
                    min_image_count: surface_capabilities.min_image_count.max(2),
                    image_extent: [dim.width, dim.height],
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),
                    ..Default::default()
                },
            )
            .map_err(VulkanErr::CreateSwapchain)?
        };

        let mut swapchain_image_views = Vec::with_capacity(images.len());

        for image in &images {
            swapchain_image_views.push(
                ImageView::new_default(image.clone())
                .map_err(VulkanErr::CreateImageView)?
            );
        }

        let get_proc = |of| unsafe {
            let result = match of {
                skia_safe::gpu::vk::GetProcOf::Instance(instance, name) => {
                    vulkan.get_instance_proc_addr(ash::vk::Instance::from_raw(instance as _), name)
                }
                skia_safe::gpu::vk::GetProcOf::Device(device, name) => {
                    (instance.fns().v1_0.get_device_proc_addr)(
                        ash::vk::Device::from_raw(device as _),
                        name,
                    )
                }
            };
    
            match result {
                Some(f) => f as _,
                None => {
                    warn!("Failed to resolve function {}", of.name().to_str().unwrap());
                    core::ptr::null()
                }
            }
        };
    
        let backend_context = unsafe {
            BackendContext::new(
                instance.handle().as_raw() as _,
                physical_device.handle().as_raw() as _,
                device.handle().as_raw() as _,
                (queue.handle().as_raw() as _, queue.queue_index() as _),
                &get_proc,
            )
        };
    
        let direct_context = skia_safe::gpu::direct_contexts::make_vulkan(&backend_context, None).ok_or(VulkanErr::SkiaContext)?;

        Ok(Self {
            direct_context: RefCell::new(direct_context),
            queue,
            gpu_future: RefCell::new(Some(sync::now(device.clone()).boxed())),
            device,
            recreate_swapchain: Cell::new(false),
            swapchain: RefCell::new(swapchain),
            swapchain_images: RefCell::new(images),
            swapchain_image_views: RefCell::new(swapchain_image_views),
        })
    }

    fn recreate_swapchain(&self, image_extent: [u32; 2]) -> RResult<()> {
        let mut swapchain = self.swapchain.borrow_mut();
        let (new_swapchain, new_images) = swapchain
            .recreate(SwapchainCreateInfo {
                image_extent,
                ..swapchain.create_info()
            })
            .map_err(VulkanErr::CreateSwapchain)?;
        *swapchain = new_swapchain;

        let mut new_swapchain_image_views = Vec::with_capacity(new_images.len());

        for image in &new_images {
            new_swapchain_image_views.push(
                ImageView::new_default(image.clone())
                .map_err(VulkanErr::CreateImageView)?
            );
        }

        *self.swapchain_images.borrow_mut() = new_images;
        *self.swapchain_image_views.borrow_mut() = new_swapchain_image_views;

        self.recreate_swapchain.set(false);

        Ok(())
    }
}

impl SkiaRenderingBackend for SkiaVulkanBackend {
    fn recreate(&self, _: &Window) {
        self.recreate_swapchain.set(true);
    }

    fn render(&self, window: &Window, canvas: impl FnOnce(&Canvas)) -> RResult<()> {
        let dimensions = window.physical_dimensions();
        if dimensions.width == 0 || dimensions.height == 0 { return Ok(()) } // Skip frame if window size is zero (e.g. when minimized)

        let image_extent = [dimensions.width, dimensions.height];

        let direct_context = &mut self.direct_context.borrow_mut();

        self.gpu_future.borrow_mut().as_mut().unwrap().cleanup_finished();

        if self.recreate_swapchain.get() {
            self.recreate_swapchain(image_extent)?;
        }

        let (image_index, suboptimal, acquire_future) =
            match vulkano::swapchain::acquire_next_image(self.swapchain.borrow().clone(), None).map_err(Validated::unwrap) {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    return Ok(()); // Try again next frame
                }
                Err(err) => Err(VulkanErr::Other(err))?,
            };

        if suboptimal {
            self.recreate_swapchain.set(true)
        }
        
        let image_view = self.swapchain_image_views.borrow()[image_index as usize].clone();
        let alloc = skia_safe::gpu::vk::Alloc::default();

        debug_assert_eq!(image_view.format(), IMAGE_FORMAT);

        let image_info = &unsafe {
            skia_safe::gpu::vk::ImageInfo::new(
                image_view.image().handle().as_raw() as _,
                alloc,
                skia_safe::gpu::vk::ImageTiling::OPTIMAL,
                skia_safe::gpu::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                SKIA_FORMAT,
                1,
                self.queue.queue_family_index(),
                None,
                None,
                None,
            )
        };

        let render_target = &skia_safe::gpu::backend_render_targets::make_vk((dimensions.width as _, dimensions.height as _), image_info);

        let surface_props = SurfaceProps::new(SurfacePropsFlags::default(), skia_safe::PixelGeometry::RGBH);

        let mut skia_surface = skia_safe::gpu::surfaces::wrap_backend_render_target(
            direct_context,
            render_target,
            skia_safe::gpu::SurfaceOrigin::TopLeft,
            SKIA_TYPE,
            None,
            Some(&surface_props),
        ).ok_or(VulkanErr::SkiaSurface)?;

        canvas(skia_surface.canvas());

        direct_context.flush_and_submit();
        drop(skia_surface);


        let future = self.gpu_future
        .borrow_mut()
        .take()
        .unwrap()
        .join(acquire_future)
        .then_swapchain_present(
            self.queue.clone(),
            SwapchainPresentInfo::swapchain_image_index(self.swapchain.borrow().clone(), image_index),
        )
        .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(future) => {
                *self.gpu_future.borrow_mut() = Some(future.boxed());
                Ok(())
            }
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain.set(true);
                *self.gpu_future.borrow_mut() = Some(sync::now(self.device.clone()).boxed());
                Ok(())
            }
            Err(err) => {
                *self.gpu_future.borrow_mut() = Some(sync::now(self.device.clone()).boxed());
                Err(VulkanErr::Other(err).into())
            }
        }
    }
}

impl Debug for SkiaVulkanBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SkiaVulkanBackend")
        .field("direct_context", &self.direct_context)
        .field("device", &self.device)
        .finish_non_exhaustive()
    }
}
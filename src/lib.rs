pub mod backend;
pub mod renderer;

pub use renderer::objects::Objects;

/*
use std::{borrow::BorrowMut, cell::RefCell, sync::Arc};

use log::{*};
use simple_logger::SimpleLogger;
use vulkano::{device::{physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags}, image::{view::ImageView, ImageUsage}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, swapchain::{Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo}, sync::{self, GpuFuture}, Handle, Validated, VulkanLibrary, VulkanObject};

pub fn main() -> Result<(), String> {
    SimpleLogger::new().with_level(
        LevelFilter::Debug
    ).env().init().expect("Failed to initialize logger");


    let mut glfw = glfw::init_no_callbacks().unwrap();

    //glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    //glfw.window_hint(WindowHint::ContextVersion(3, 3));
    //glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    
    glfw.window_hint(WindowHint::Maximized(true));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    glfw.window_hint(WindowHint::TransparentFramebuffer(true));
    let (mut pwindow, events) = glfw.create_window(700, 400, "Test", glfw::WindowMode::Windowed).unwrap();
    let (f_w, f_h) = pwindow.get_framebuffer_size();
    pwindow.set_all_polling(true);
    glfw.make_context_current(Some(&pwindow));


    let vulkan = VulkanLibrary::new().unwrap();
    let req_extensions = Surface::required_extensions(&pwindow).unwrap();
    debug!("{:?}", req_extensions);

    let instance = Instance::new(vulkan.clone(), InstanceCreateInfo {
        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
        enabled_extensions: req_extensions,
        ..Default::default()
    }).unwrap();

    let window = Arc::new(pwindow);
    let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
    .enumerate_physical_devices()
    .unwrap()
    .filter(|p| p.supported_extensions().contains(&device_extensions))
    .filter_map(|p| {
        p.queue_family_properties()
            .iter()
            .enumerate()
            .position(|(i, q)| {
                q.queue_flags.intersects(QueueFlags::GRAPHICS)
                    && p.surface_support(i as u32, &surface).unwrap_or(false)
            })
            .map(|i| (p, i as u32))
    })
    .min_by_key(|(p, _)| match p.properties().device_type {
        PhysicalDeviceType::IntegratedGpu => 0,
        PhysicalDeviceType::DiscreteGpu => 1,
        PhysicalDeviceType::VirtualGpu => 2,
        PhysicalDeviceType::Cpu => 3,
        PhysicalDeviceType::Other => 4,
        _ => 5,
    })
    .expect("Vulkan: Failed to find suitable physical device");

    info!(
        "Using device: {} (type: {:?})",
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
    )
    .unwrap();

    let queue = queues.next().unwrap();

    let (swapchain, swapchain_images) = {
        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .unwrap();
        let image_format = vulkano::format::Format::B8G8R8A8_UNORM;

        Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count,
                image_format,
                image_extent: [f_w.try_into().unwrap(), f_h.try_into().unwrap()],
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            },
        )
        .unwrap()
    };

    let mut swapchain_image_views = Vec::with_capacity(swapchain_images.len());

    for image in &swapchain_images {
        swapchain_image_views.push(ImageView::new_default(image.clone()).map_err(|vke| {
            format!("fatal: Error creating image view for swap chain image: {vke}")
        }).unwrap());
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
                //println!("resolve of {} failed", of.name().to_str().unwrap());
                core::ptr::null()
            }
        }
    };

    let backend_context = unsafe {
        skia_safe::gpu::vk::BackendContext::new(
            instance.handle().as_raw() as _,
            physical_device.handle().as_raw() as _,
            device.handle().as_raw() as _,
            (queue.handle().as_raw() as _, queue_family_index as _),
            &get_proc,
        )
    };

    let (image_index, suboptimal, acquire_future) =
        match vulkano::swapchain::acquire_next_image(swapchain.clone(), None).map_err(Validated::unwrap) {
            Ok(r) => r,
            // Err(VulkanError::OutOfDate) => {
            //     return Ok(()); // Try again next frame
            // }
            Err(e) => return Err(format!("Vulkan: failed to acquire next image: {e}")),
        };

    let mut gr_context = skia_safe::gpu::DirectContext::new_vulkan(&backend_context, None).unwrap();
    let image_view = swapchain_image_views[image_index as usize].clone();
    let previous_frame_end = RefCell::new(Some(sync::now(device.clone()).boxed()));
    let alloc = skia_safe::gpu::vk::Alloc::default();


    let image_info = &unsafe {
        skia_safe::gpu::vk::ImageInfo::new(
            image_view.image().handle().as_raw() as _,
            alloc,
            skia_safe::gpu::vk::ImageTiling::OPTIMAL,
            skia_safe::gpu::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            skia_safe::gpu::vk::Format::B8G8R8A8_UNORM,
            1,
            None,
            None,
            None,
            None,
        )
    };

    let render_target = &skia_safe::gpu::backend_render_targets::make_vk((f_w, f_h), image_info);

    let mut skia_surface = skia_safe::gpu::surfaces::wrap_backend_render_target(
        gr_context.borrow_mut(),
        render_target,
        skia_safe::gpu::SurfaceOrigin::TopLeft,
        skia_safe::ColorType::BGRA8888,
        None,
        None,
    ).unwrap();

    let canvas = skia_surface.canvas();
    //canvas.clear(0xABCDEF);

    drop(skia_surface);
    gr_context.submit(None);


    let future = previous_frame_end
    .borrow_mut()
    .take()
    .unwrap()
    .join(acquire_future)
    .then_swapchain_present(
        queue.clone(),
        SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
    )
    .then_signal_fence_and_flush();

    //let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
    //    backends: Backends::PRIMARY,
    //    ..Default::default()
    //});
    //
    //instance.create_surface(&window).unwrap();




    info!("startin");

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            debug!("{:?}", event);
            //handle_window_event(&mut window, event);
        }

        glfw.poll_events();
    }

    Ok(())
}
    */
use thiserror::Error;



#[derive(Debug, Error)]
pub enum SkiaRendererError {
    #[error("No Skia rendering backend could be created")]
    NoBackend,
    #[cfg(feature = "skia-vulkan")]
    #[error(transparent)]
    Vulkan(#[from] VulkanErr)
}

#[cfg(feature = "skia-vulkan")]
#[derive(Debug, Error)]
pub enum VulkanErr {
    #[error("Failed to initialize Vk library: {0}")]
    InitLibrary(vulkano::LoadingError),
    #[error("Failed to initialize Vk instance: {0}")]
    InitInstance(vulkano::Validated<vulkano::VulkanError>),
    #[error("Failed to create Vk surface: {0}")]
    CreateSurface(vulkano::swapchain::FromWindowError),
    #[error("Failed to create Vk swapchain: {0}")]
    CreateSwapchain(vulkano::Validated<vulkano::VulkanError>),
    #[error("Failed to create ImageView for swapchain image: {0}")]
    CreateImageView(vulkano::Validated<vulkano::VulkanError>),

    #[error("No Vk device available")]
    NoDevice,
    #[error("No Vk Queue available")]
    NoQueue,
    #[error("Failed to create Skia DirectContext")]
    SkiaContext,
    #[error("Failed to create Skia Surface")]
    SkiaSurface,

    #[error("Vulkan error occured: {0}")]
    Other(#[from] vulkano::VulkanError)
}
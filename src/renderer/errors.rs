use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[cfg(feature = "r-skia")]
    #[error(transparent)]
    Skia(super::skia::errors::SkiaRendererError),
    #[cfg(feature = "r-wgpu")]
    #[error(transparent)]
    Wgpu(super::wgpu::errors::WgpuRendererError),
    
    #[error("No renderer could be created! Unable to continue!")]
    NoRenderer,
    #[error("Could not get required window handles!")]
    WindowHandles
}

#[cfg(feature = "r-skia")]
impl<T: Into<super::skia::errors::SkiaRendererError>> From<T> for RendererError {
    fn from(value: T) -> Self {
        Self::Skia(value.into())
    }
}

#[cfg(feature = "r-wgpu")]
impl<T: Into<super::wgpu::errors::WgpuRendererError>> From<T> for RendererError {
    fn from(value: T) -> Self {
        Self::Wgpu(value.into())
    }
}
use std::sync::Arc;

use uuid::Uuid;

use crate::backend::windows::Dimensions;

use super::RResult;


/// ### A cheap to clone, decoded Image container.
/// Stores:
/// * The pixels as bytes in an Arc<\[u8]>
/// * The format of the pixels
/// * The image's width and height
/// * A Uuid to allow cheap lookup in for example a HashMap
#[derive(Debug, Clone)]
pub struct CacheableImage {
    pixels: Arc<[u8]>,
    format: PixelFormat,
    dimensions: Dimensions,
    uuid: Uuid
}

#[derive(Debug, Clone)]
pub enum PixelFormat {
    RGB8,
    RGBA8,
    RGBA8Premul
}

impl CacheableImage {
    /// ### Creates a CachedImage from the image's encoded bytes.
    /// This uses either Skia's built in image decoding, if `r-skia` is enabled, or the `image` crate, if the `image` feature is enabled.  
    /// If neither are enabled, this will panic.
    #[allow(unreachable_code)]
    pub fn from_encoded(bytes: &[u8]) -> RResult<Self> {
        #[cfg(feature = "r-skia")]
        return Self::from_bytes_skia(bytes);
        #[cfg(feature = "image")]
        return Self::from_bytes_image(bytes);

        panic!("No decoding backend enabled!")
    }

    #[cfg(feature = "image")]
    pub fn from_image(image: image::DynamicImage) -> RResult<Self> {
        let dimensions = Dimensions::new(image.width(), image.height());
        let pixels = Arc::from_iter(image.into_rgba8().into_vec());

        Ok(Self::new(
            pixels,
            PixelFormat::RGBA8,
            dimensions
        ))
    }

    #[cfg(feature = "image")]
    pub fn from_bytes_image(bytes: &[u8]) -> RResult<Self> {
        use super::errors::RegisterError;

        let image = image::load_from_memory(bytes)
        .map_err(|err| RegisterError::Image(err.to_string()))?;

        Self::from_image(image)
    }

    #[cfg(feature = "r-skia")]
    pub fn from_bytes_skia(bytes: &[u8]) -> RResult<Self> {
        use super::errors::RegisterError;

        let data = unsafe { skia_safe::Data::new_bytes(bytes) };
        let image = skia_safe::Image::from_encoded(data)
        .ok_or(RegisterError::Image("Failed to decode image".to_string()))?;
        
        let info = image.image_info()
        .with_color_type(skia_safe::ColorType::RGBA8888)
        .with_alpha_type(skia_safe::AlphaType::Unpremul);
        let byte_size = info.compute_byte_size(info.min_row_bytes());

        let mut read_pixels = vec![0u8; byte_size];

        image.read_pixels(&info, &mut read_pixels, info.min_row_bytes(), (0, 0), skia_safe::image::CachingHint::Allow);

        let pixels = Arc::from_iter(read_pixels);

        Ok(Self::new(
            pixels,
            PixelFormat::RGBA8,
            Dimensions::new(info.width() as u32, info.height() as u32)
        ))
    }

    pub fn new(pixels: Arc<[u8]>, format: PixelFormat, dimensions: Dimensions) -> Self {
        Self { pixels, format, dimensions, uuid: Uuid::new_v4() }
    }

    pub fn pixels(&self) -> Arc<[u8]> {
        self.pixels.clone()
    }

    pub fn format(&self) -> &PixelFormat {
        &self.format
    }

    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}
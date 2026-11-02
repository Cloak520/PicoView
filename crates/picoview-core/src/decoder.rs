use std::path::Path;
use anyhow::{Context, Result};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageReader};

pub struct Decoder;

impl Decoder {
    /// Probe image dimensions quickly (often reading just header chunks)
    pub fn probe_dimensions<P: AsRef<Path>>(path: P) -> Result<(u32, u32)> {
        let reader = ImageReader::open(&path)
            .with_context(|| format!("Failed to open image file: {:?}", path.as_ref()))?
            .with_guessed_format()?;

        let dimensions = reader.into_dimensions()?;
        Ok(dimensions)
    }

    /// Load full dynamic image from path
    pub fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
        let img = image::open(&path)
            .with_context(|| format!("Failed to decode image: {:?}", path.as_ref()))?;
        Ok(img)
    }

    /// Resize using image crate standard fallback
    pub fn resize_fast(img: &DynamicImage, max_w: u32, max_h: u32) -> DynamicImage {
        img.resize(max_w, max_h, FilterType::Triangle)
    }
}

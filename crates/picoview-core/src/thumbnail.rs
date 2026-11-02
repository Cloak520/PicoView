use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use image::ImageFormat;

pub struct ThumbnailGenerator {
    cache_dir: PathBuf,
}

impl ThumbnailGenerator {
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Self {
        let dir = cache_dir.as_ref().to_path_buf();
        let _ = std::fs::create_dir_all(&dir);
        Self { cache_dir: dir }
    }

    /// Compute unique cached filename based on source path and size
    pub fn get_cache_path(&self, source_path: &str, size: u32) -> PathBuf {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&source_path, &mut hasher);
        std::hash::Hash::hash(&size, &mut hasher);
        let hash = std::hash::Hasher::finish(&hasher);
        self.cache_dir.join(format!("{:016x}_{}.webp", hash, size))
    }

    /// Generate and cache a thumbnail (returns path to cached thumbnail)
    pub fn generate_or_get(&self, source_path: &str, max_size: u32) -> Result<PathBuf> {
        let target_path = self.get_cache_path(source_path, max_size);
        if target_path.exists() {
            return Ok(target_path);
        }

        let img = image::open(source_path)
            .with_context(|| format!("Failed to read image for thumbnail: {}", source_path))?;

        let thumb = img.thumbnail(max_size, max_size);

        // Save as WebP for optimal size and fast decode
        thumb
            .save_with_format(&target_path, ImageFormat::WebP)
            .with_context(|| format!("Failed to save thumbnail to {:?}", target_path))?;

        Ok(target_path)
    }
}

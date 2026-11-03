use std::path::Path;
use anyhow::Result;
use jwalk::WalkDir;

use crate::models::MediaItem;

const SUPPORTED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff", "tif",
];

pub struct Scanner;

impl Scanner {
    /// Check if a path has an image extension
    pub fn is_image_file<P: AsRef<Path>>(path: P) -> bool {
        if let Some(ext) = path.as_ref().extension().and_then(|e| e.to_str()) {
            let lower = ext.to_ascii_lowercase();
            return SUPPORTED_EXTENSIONS.contains(&lower.as_str());
        }
        false
    }

    /// Fast scan of a single folder (non-recursive) for instant viewer navigation
    pub fn scan_folder_siblings<P: AsRef<Path>>(folder_path: P) -> Result<Vec<MediaItem>> {
        let path = folder_path.as_ref();
        let mut items = Vec::new();

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() && Self::is_image_file(&p) {
                    if let Some(item) = Self::path_to_media_item(&p) {
                        items.push(item);
                    }
                }
            }
        }

        // Sort naturally by filename
        items.sort_by(|a, b| a.filename.to_lowercase().cmp(&b.filename.to_lowercase()));
        Ok(items)
    }

    /// Parallel recursive scan of entire directories using jwalk
    pub fn scan_directory_parallel<P: AsRef<Path>>(root_dir: P) -> Result<Vec<MediaItem>> {
        let mut items = Vec::new();

        for entry in WalkDir::new(root_dir)
            .sort(true)
            .into_iter()
            .flatten()
        {
            if entry.file_type.is_file() {
                let p = entry.path();
                if Self::is_image_file(&p) {
                    if let Some(item) = Self::path_to_media_item(&p) {
                        items.push(item);
                    }
                }
            }
        }

        Ok(items)
    }

    /// Construct MediaItem from file metadata
    pub fn path_to_media_item(path: &Path) -> Option<MediaItem> {
        let metadata = std::fs::metadata(path).ok()?;
        let filename = path.file_name()?.to_string_lossy().to_string();
        let parent_dir = path.parent()?.to_string_lossy().to_string();
        let size_bytes = metadata.len();
        let modified_ms = metadata
            .modified()
            .ok()?
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_millis() as u64;

        // Fast metadata extraction: width & height are filled in by webview on load
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        let mime_type = match ext.as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "webp" => "image/webp",
            "gif" => "image/gif",
            "bmp" => "image/bmp",
            _ => "application/octet-stream",
        }
        .to_string();

        Some(MediaItem {
            id: None,
            path: path.to_string_lossy().to_string(),
            filename,
            parent_dir,
            size_bytes,
            modified_ms,
            width: 0,
            height: 0,
            mime_type,
            is_starred: false,
            rating: 0,
            date_taken: None,
        })
    }
}

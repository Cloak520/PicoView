use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: Option<i64>,
    pub path: String,
    pub filename: String,
    pub parent_dir: String,
    pub size_bytes: u64,
    pub modified_ms: u64,
    pub width: u32,
    pub height: u32,
    pub mime_type: String,
    pub is_starred: bool,
    pub rating: u8,
    pub date_taken: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExifData {
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub focal_length: Option<String>,
    pub f_number: Option<String>,
    pub exposure_time: Option<String>,
    pub iso: Option<u32>,
    pub date_time_original: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRecipe {
    pub photo_path: String,
    pub crop: Option<[f32; 4]>, // [x, y, width, height] normalized (0.0 - 1.0)
    pub rotation: i32,          // 0, 90, 180, 270 or arbitrary straighten degrees
    pub brightness: f32,       // -1.0 .. 1.0 (0.0 is default)
    pub contrast: f32,         // 0.0 .. 2.0 (1.0 is default)
    pub saturation: f32,       // 0.0 .. 2.0 (1.0 is default)
    pub is_starred: bool,
}

impl Default for ImageRecipe {
    fn default() -> Self {
        Self {
            photo_path: String::new(),
            crop: None,
            rotation: 0,
            brightness: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            is_starred: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderItem {
    pub path: String,
    pub name: String,
    pub item_count: usize,
    pub cover_photo: Option<String>,
}

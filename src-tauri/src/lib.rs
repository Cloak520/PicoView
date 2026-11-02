use std::path::Path;
use picoview_core::{ExifData, ExifParser, MediaItem, Scanner, ThumbnailGenerator};

// State holder for PicoView
pub struct AppState {
    pub thumb_gen: ThumbnailGenerator,
}

#[tauri::command]
fn scan_folder(folder_path: String) -> Result<Vec<MediaItem>, String> {
    Scanner::scan_folder_siblings(&folder_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_sibling_photos(photo_path: String) -> Result<Vec<MediaItem>, String> {
    let path = Path::new(&photo_path);
    if let Some(parent) = path.parent() {
        Scanner::scan_folder_siblings(parent).map_err(|e| e.to_string())
    } else {
        Err("Invalid photo path".to_string())
    }
}

#[tauri::command]
fn get_exif_data(photo_path: String) -> Result<ExifData, String> {
    ExifParser::parse(&photo_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn generate_thumbnail(
    state: tauri::State<AppState>,
    photo_path: String,
    max_size: Option<u32>,
) -> Result<String, String> {
    let size = max_size.unwrap_or(320);
    let cached_path = state
        .thumb_gen
        .generate_or_get(&photo_path, size)
        .map_err(|e| e.to_string())?;
    Ok(cached_path.to_string_lossy().to_string())
}

#[tauri::command]
fn delete_to_trash(photo_path: String) -> Result<(), String> {
    trash::delete(&photo_path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_cache_dir = std::env::temp_dir().join("picoview_cache");
    let thumb_gen = ThumbnailGenerator::new(app_cache_dir);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState { thumb_gen })
        .invoke_handler(tauri::generate_handler![
            scan_folder,
            get_sibling_photos,
            get_exif_data,
            generate_thumbnail,
            delete_to_trash,
        ])
        .run(tauri::generate_context!())
        .expect("error while running picoview tauri application");
}

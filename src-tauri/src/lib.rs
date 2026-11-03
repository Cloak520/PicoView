use std::path::Path;
use std::sync::Mutex;
use picoview_core::{ExifData, ExifParser, MediaItem, Scanner, ThumbnailGenerator};
use serde::Serialize;
use tauri::{Emitter, Manager};

// State holder for PicoView
pub struct AppState {
    pub thumb_gen: ThumbnailGenerator,
    pub pending_file: Mutex<Option<String>>,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
pub enum OpenTarget {
    Folder {
        folder_path: String,
        items: Vec<MediaItem>,
    },
    SinglePhoto {
        siblings: Vec<MediaItem>,
        current_index: usize,
    },
}

#[tauri::command]
fn open_target_path(target_path: String) -> Result<OpenTarget, String> {
    let path = Path::new(&target_path);
    if !path.exists() {
        return Err(format!("路径不存在: {}", target_path));
    }

    if path.is_dir() {
        let items = Scanner::scan_folder_siblings(path).map_err(|e| e.to_string())?;
        Ok(OpenTarget::Folder {
            folder_path: target_path,
            items,
        })
    } else if Scanner::is_image_file(path) {
        let parent = path.parent().ok_or_else(|| "无法获取上级目录".to_string())?;
        let siblings = Scanner::scan_folder_siblings(parent).map_err(|e| e.to_string())?;
        let norm_target = path.to_string_lossy().to_string();
        let current_index = siblings
            .iter()
            .position(|it| it.path == norm_target)
            .unwrap_or(0);
        Ok(OpenTarget::SinglePhoto {
            siblings,
            current_index,
        })
    } else {
        Err("不支持的文件类型，仅支持常见图片或文件夹".to_string())
    }
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

#[tauri::command]
fn exit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}

#[tauri::command]
fn get_cli_arg(state: tauri::State<AppState>) -> Option<String> {
    if let Ok(mut lock) = state.pending_file.lock() {
        if let Some(path) = lock.take() {
            return Some(path);
        }
    }
    std::env::args().skip(1).find(|arg| !arg.starts_with('-'))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_cache_dir = std::env::temp_dir().join("picoview_cache");
    let thumb_gen = ThumbnailGenerator::new(app_cache_dir);

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            thumb_gen,
            pending_file: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            exit_app,
            get_cli_arg,
            open_target_path,
            scan_folder,
            get_sibling_photos,
            get_exif_data,
            generate_thumbnail,
            delete_to_trash,
        ])
        .build(tauri::generate_context!())
        .expect("error while building picoview tauri application");

    app.run(|app_handle, event| {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if let tauri::RunEvent::Opened { urls } = event {
            for url in urls {
                if let Ok(file_path) = url.to_file_path() {
                    let path_str = file_path.to_string_lossy().to_string();
                    if let Some(state) = app_handle.try_state::<AppState>() {
                        if let Ok(mut lock) = state.pending_file.lock() {
                            *lock = Some(path_str.clone());
                        }
                    }
                    let _ = app_handle.emit("open-file", path_str);
                }
            }
        }
    });
}

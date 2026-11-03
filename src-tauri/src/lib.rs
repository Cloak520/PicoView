use std::path::Path;
use std::sync::Mutex;
use picoview_core::{ExifData, ExifParser, MediaItem, Scanner};
use serde::Serialize;
use tauri::{Emitter, Manager};

// State holder for PicoView
pub struct AppState {
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
fn get_exif_data(photo_path: String) -> Result<ExifData, String> {
    ExifParser::parse(&photo_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_to_trash(photo_path: String) -> Result<(), String> {
    trash::delete(&photo_path).map_err(|e| e.to_string())
}

pub fn setup_window_effects(window: &tauri::WebviewWindow) {
    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
        let _ = apply_vibrancy(window, NSVisualEffectMaterial::FullScreenUI, None, None);
    }
    #[cfg(target_os = "windows")]
    {
        use window_vibrancy::apply_mica;
        let _ = apply_mica(window, Some(true));
    }
}

pub fn fit_window_to_monitor(window: &tauri::WebviewWindow) {
    let monitor = window
        .current_monitor()
        .ok()
        .flatten()
        .or_else(|| window.primary_monitor().ok().flatten());
    if let Some(monitor) = monitor {
        let size = monitor.size();
        let pos = monitor.position();
        let _ = window.set_position(tauri::Position::Physical(*pos));
        let _ = window.set_size(tauri::Size::Physical(*size));
    }
}

#[tauri::command]
fn fit_to_screen(window: tauri::WebviewWindow) {
    fit_window_to_monitor(&window);
    setup_window_effects(&window);
    let _ = window.set_focus();
}

#[tauri::command]
fn copy_file_to_clipboard(file_path: String) -> Result<(), String> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("文件不存在".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        let escaped = file_path.replace('"', "\\\"");
        let script = format!(r#"set the clipboard to (POSIX file "{}")"#, escaped);
        let _ = std::process::Command::new("osascript")
            .arg("-e")
            .arg(script)
            .status();
        Ok(())
    }
    #[cfg(target_os = "windows")]
    {
        let escaped = file_path.replace('\'', "''");
        let script = format!("Set-Clipboard -Path '{}'", escaped);
        let _ = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .status();
        Ok(())
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Ok(())
    }
}

#[tauri::command]
fn exit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}

#[tauri::command]
fn get_initial_target(state: tauri::State<AppState>) -> Option<OpenTarget> {
    let arg = {
        if let Ok(mut lock) = state.pending_file.lock() {
            if let Some(path) = lock.take() {
                Some(path)
            } else {
                std::env::args().skip(1).find(|arg| !arg.starts_with('-'))
            }
        } else {
            std::env::args().skip(1).find(|arg| !arg.starts_with('-'))
        }
    }?;

    open_target_path(arg).ok()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            pending_file: Mutex::new(None),
        })
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                fit_window_to_monitor(&window);
                setup_window_effects(&window);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fit_to_screen,
            exit_app,
            get_initial_target,
            open_target_path,
            get_exif_data,
            delete_to_trash,
            copy_file_to_clipboard,
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
                    if let Some(window) = app_handle.get_webview_window("main") {
                        fit_window_to_monitor(&window);
                        setup_window_effects(&window);
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    let _ = app_handle.emit("open-file", path_str);
                }
            }
        }
    });
}

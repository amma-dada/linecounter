// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod counter;
use counter::{
    count_file_lines, count_text_lines, scan_directory_tree, DirectorySummary, FileStats,
};
use std::path::Path;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_opener::OpenerExt;

fn setup_environment() {
    #[cfg(target_os = "linux")]
    {
        // Force GTK3 and GIO to use XDG Desktop Portal for file choosers (modern Libadwaita / GNOME dialog)
        std::env::set_var("GTK_USE_PORTAL", "1");
        std::env::set_var("GIO_USE_PORTALS", "1");

        // Set GLib program name and application name so GNOME Shell / Wayland matches window to com.linecounter.desktop
        unsafe {
            extern "C" {
                fn g_set_prgname(prgname: *const std::os::raw::c_char);
                fn g_set_application_name(app_name: *const std::os::raw::c_char);
            }
            if let Ok(prg) = std::ffi::CString::new("com.linecounter.desktop") {
                g_set_prgname(prg.as_ptr());
            }
            if let Ok(app) = std::ffi::CString::new("LineCounter") {
                g_set_application_name(app.as_ptr());
            }
        }

        // Suppress Mesa / Intel Vulkan DMA-BUF format modifier bugs and driver hangs
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        std::env::set_var("DRIRC_CONFIG_PATH", "/dev/null");
        std::env::set_var("LIBGL_DEBUG", "quiet");
        std::env::set_var("LIBGL_DRIRC_DISABLE", "1");
        std::env::set_var("MESA_LOG_LEVEL", "none");
        std::env::set_var("MESA_VK_LOG_LEVEL", "none");
        std::env::set_var("ANV_DEBUG", "none");
        std::env::set_var("GSK_RENDERER", "ngl");

        // Attempt creating dummy user drirc if missing
        if let Ok(home) = std::env::var("HOME") {
            let drirc_path = format!("{}/.drirc", home);
            if !std::path::Path::new(&drirc_path).exists() {
                let _ = std::fs::write(&drirc_path, "<driconf>\n</driconf>\n");
            }

            // Ensure DnD cursor fallbacks exist in user icons to prevent missing theme warnings
            let cdir = format!("{}/.local/share/icons/Adwaita/cursors", home);
            if let Ok(()) = std::fs::create_dir_all(&cdir) {
                use std::os::unix::fs::symlink;
                if !std::path::Path::new(&format!("{}/dnd-none", cdir)).exists() {
                    let _ = symlink("/usr/share/icons/Adwaita/cursors/no-drop", format!("{}/dnd-none", cdir));
                }
                if !std::path::Path::new(&format!("{}/dnd-move", cdir)).exists() {
                    let _ = symlink("/usr/share/icons/Adwaita/cursors/move", format!("{}/dnd-move", cdir));
                }
                if !std::path::Path::new(&format!("{}/dnd-copy", cdir)).exists() {
                    let _ = symlink("/usr/share/icons/Adwaita/cursors/copy", format!("{}/dnd-copy", cdir));
                }
            }
        }
    }
}

#[tauri::command]
async fn count_path(
    path: String,
    include_trailing_empty: Option<bool>,
    treat_whitespace_as_empty: Option<bool>,
) -> Result<FileStats, String> {
    tokio::task::spawn_blocking(move || {
        let p = Path::new(&path);
        if !p.exists() {
            return Err(format!("File does not exist: {}", path));
        }
        count_file_lines(
            p,
            None,
            include_trailing_empty.unwrap_or(false),
            treat_whitespace_as_empty.unwrap_or(true),
            true,
        )
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
fn count_text(
    text: String,
    filename: Option<String>,
    include_trailing_empty: Option<bool>,
    treat_whitespace_as_empty: Option<bool>,
) -> FileStats {
    let fname = filename.unwrap_or_else(|| "Pasted Text".to_string());
    count_text_lines(
        &text,
        &fname,
        include_trailing_empty.unwrap_or(false),
        treat_whitespace_as_empty.unwrap_or(true),
        true,
    )
}

#[tauri::command]
async fn scan_directory(
    path: String,
    recursive: Option<bool>,
    include_trailing_empty: Option<bool>,
    treat_whitespace_as_empty: Option<bool>,
) -> Result<DirectorySummary, String> {
    tokio::task::spawn_blocking(move || {
        let p = Path::new(&path);
        scan_directory_tree(
            p,
            recursive.unwrap_or(true),
            include_trailing_empty.unwrap_or(false),
            treat_whitespace_as_empty.unwrap_or(true),
        )
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn batch_scan_paths(
    paths: Vec<String>,
    include_trailing_empty: Option<bool>,
    treat_whitespace_as_empty: Option<bool>,
) -> Vec<FileStats> {
    tokio::task::spawn_blocking(move || {
        use rayon::prelude::*;
        let trail = include_trailing_empty.unwrap_or(false);
        let ws = treat_whitespace_as_empty.unwrap_or(true);

        paths
            .par_iter()
            .filter_map(|p_str| {
                let p = Path::new(p_str);
                if p.is_file() {
                    count_file_lines(p, None, trail, ws, false).ok()
                } else {
                    None
                }
            })
            .collect()
    })
    .await
    .unwrap_or_default()
}

#[tauri::command]
fn inspect_path(path: String) -> String {
    let p = Path::new(&path);
    if p.is_dir() {
        "directory".to_string()
    } else if p.is_file() {
        "file".to_string()
    } else {
        "unknown".to_string()
    }
}

#[tauri::command]
fn open_external_url(app: AppHandle, url: String) -> Result<(), String> {
    app.opener()
        .open_url(&url, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn choose_file_dialog(app: AppHandle) -> Option<String> {
    app.dialog().file().blocking_pick_file().map(|f| f.to_string())
}

#[tauri::command]
async fn choose_folder_dialog(app: AppHandle) -> Option<String> {
    app.dialog().file().blocking_pick_folder().map(|f| f.to_string())
}

#[tauri::command]
async fn choose_files_dialog(app: AppHandle) -> Vec<String> {
    app.dialog()
        .file()
        .blocking_pick_files()
        .unwrap_or_default()
        .into_iter()
        .map(|f| f.to_string())
        .collect()
}

#[tauri::command]
async fn save_file_content(path: String, content: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let p = Path::new(&path);
        std::fs::write(p, content.as_bytes()).map_err(|e| format!("Failed to save file: {}", e))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
fn show_main_window(window: tauri::Window) {
    let _ = window.show();
    let _ = window.set_focus();
}

fn main() {
    setup_environment();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "linux")]
            unsafe {
                extern "C" {
                    fn gtk_window_set_default_icon_name(name: *const std::os::raw::c_char);
                    fn gtk_window_set_default_icon_from_file(
                        filename: *const std::os::raw::c_char,
                        err: *mut *mut std::ffi::c_void,
                    ) -> i32;
                }
                if let Ok(icon) = std::ffi::CString::new("com.linecounter.desktop") {
                    gtk_window_set_default_icon_name(icon.as_ptr());
                }
                if let Ok(home) = std::env::var("HOME") {
                    let candidates = [
                        format!("{}/.local/share/icons/hicolor/256x256/apps/com.linecounter.desktop.png", home),
                        format!("{}/.local/share/icons/hicolor/128x128/apps/com.linecounter.desktop.png", home),
                        format!("{}/.local/share/pixmaps/com.linecounter.desktop.png", home),
                        format!("{}/.local/share/pixmaps/linecounter.png", home),
                    ];
                    for p in &candidates {
                        if std::path::Path::new(p).exists() {
                            if let Ok(c_p) = std::ffi::CString::new(p.as_str()) {
                                gtk_window_set_default_icon_from_file(c_p.as_ptr(), std::ptr::null_mut());
                                break;
                            }
                        }
                    }
                }
            }

            if let Some(window) = app.get_webview_window("main") {
                if let Some(icon) = app.default_window_icon() {
                    let _ = window.set_icon(icon.clone());
                }
                let win_clone = window.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(600));
                    let _ = win_clone.show();
                });
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event {
                let string_paths: Vec<String> = paths
                    .iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect();
                let _ = window.emit("tauri-file-drop", string_paths);
            }
        })
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            count_path,
            count_text,
            scan_directory,
            batch_scan_paths,
            inspect_path,
            open_external_url,
            choose_file_dialog,
            choose_folder_dialog,
            choose_files_dialog,
            save_file_content,
        ])
        .run(tauri::generate_context!())
        .expect("error while running LineCounter desktop application");
}

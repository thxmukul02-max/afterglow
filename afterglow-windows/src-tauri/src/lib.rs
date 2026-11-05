use std::path::Path;
use tauri::{Emitter, Manager};

const EXTS: [&str; 12] = [
    "mp4", "m4v", "mkv", "webm", "mov", "mp3", "m4a", "aac", "wav", "ogg", "opus", "flac",
];

// Keep only existing media files from the command-line arguments (first one is the .exe itself).
fn media_args<I: IntoIterator<Item = String>>(args: I) -> Vec<String> {
    args.into_iter()
        .skip(1)
        .filter(|a| {
            let p = Path::new(a);
            p.is_file()
                && p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| EXTS.iter().any(|x| *x == e.to_lowercase()))
                    .unwrap_or(false)
        })
        .collect()
}

// Files Windows passed when Afterglow was started (double-click / Open with).
#[tauri::command]
fn get_launch_files() -> Vec<String> {
    media_args(std::env::args())
}

// Looks for movie.srt or movie.vtt next to movie.mp4 and returns its bytes.
#[tauri::command]
fn find_sub(path: String) -> Option<Vec<u8>> {
    let base = Path::new(&path);
    for ext in ["srt", "vtt"] {
        let candidate = base.with_extension(ext);
        if candidate.is_file() {
            if let Ok(bytes) = std::fs::read(&candidate) {
                if bytes.len() < 5_000_000 {
                    return Some(bytes);
                }
            }
        }
    }
    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // If Afterglow is already open, send the new file to that window instead of opening a second one.
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            let files = media_args(args);
            if !files.is_empty() {
                let _ = app.emit("open-files", files);
            }
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![get_launch_files, find_sub])
        .run(tauri::generate_context!())
        .expect("error while running Afterglow");
}

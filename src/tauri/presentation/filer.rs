use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

// Open the file explorer at the specified directory or select the file if a file path is given.
pub fn open_filer(app: &AppHandle, dir: &str) -> Result<(), String> {
    let p = PathBuf::from(dir);

    if p.is_dir() {
        // open directory
        app.opener()
            .open_path(p.to_string_lossy().to_string(), None::<&str>)
            .map_err(|e| e.to_string())
    } else {
        // open directory and select file
        app.opener()
            .reveal_item_in_dir(p)
            .map_err(|e| e.to_string())
    }
}

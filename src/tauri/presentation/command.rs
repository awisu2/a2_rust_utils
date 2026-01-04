use crate::tauri::presentation::path::{self, AppDirs, SystemDirs};
use tauri::AppHandle;

#[tauri::command]
pub fn app_dirs(app_handle: &AppHandle) -> Result<AppDirs, String> {
    path::app_dirs(&app_handle)
}

#[tauri::command]
pub fn system_dirs(app_handle: &AppHandle) -> Result<SystemDirs, String> {
    path::system_dirs(&app_handle)
}

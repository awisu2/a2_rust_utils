mod config;
mod file;
mod filemeta;
mod os;
mod path;
mod stores;
mod thumbnail;

use std::path::PathBuf;

use self::{file::FileInfo, filemeta::FileMeta};

// config =====
#[tauri::command]
pub fn write_config(app: tauri::AppHandle, config: config::Config) -> Result<(), String> {
    config::write(app, config)
}

#[tauri::command]
pub fn read_config(app: tauri::AppHandle) -> Result<config::Config, String> {
    config::read(app)
}

#[tauri::command]
pub fn get_config_path(app: tauri::AppHandle) -> Result<PathBuf, String> {
    match config::get_config_path(app) {
        Some(path) => Ok(path),
        None => Err(String::from("no config_path")),
    }
}

// files =====
#[tauri::command]
pub fn read_dir(dir: &str) -> Result<Vec<file::FileInfo>, String> {
    file::read_dir(dir)
}

#[tauri::command]
pub fn get_fileinfo(path: &str) -> file::FileInfo {
    file::str_to_fileinfo(path)
}

#[tauri::command]
pub fn is_exists_file(path: &str) -> bool {
    file::is_exists_file(path)
}

// favorites =====
#[tauri::command]
pub fn read_filemetas(app: tauri::AppHandle) -> Result<Vec<FileMeta>, String> {
    filemeta::read(app)
}

#[tauri::command]
pub fn write_filemetas(app: tauri::AppHandle, favorites: Vec<FileMeta>) -> Result<(), String> {
    filemeta::write(app, favorites)
}

#[tauri::command]
pub fn get_thumbnailable_path(dir: &str, deep: i8) -> Result<FileInfo, String> {
    thumbnail::get_thumbnailable_path(dir, deep)
}

#[tauri::command]
pub fn open_filer(dir: &str) -> Result<(), String> {
    os::open_filer(dir)
}

#[tauri::command]
pub fn rename(from: &str, to: &str) -> Result<(), String> {
    file::rename(from, to)
}

#[tauri::command]
pub fn remove_dir_all(path: &str) -> Result<(), String> {
    file::remove_dir_all(path)
}

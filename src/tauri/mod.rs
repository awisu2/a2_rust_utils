use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDirs {
    pub config_dir: PathBuf,     // App configuration directory
    pub cache_dir: PathBuf,      // App cache directory
    pub log_dir: PathBuf,        // App log directory
    pub local_data_dir: PathBuf, // data directory.
    pub data_dir: PathBuf, // data directory. roaming on Windows (roaming is only windows concept)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemDirs {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub local_data_dir: PathBuf,
    pub desktop_dir: PathBuf,
    pub document_dir: PathBuf,
    pub download_dir: PathBuf,
    pub executable_dir: PathBuf,
    pub font_dir: PathBuf,
    pub home_dir: PathBuf,
    pub picture_dir: PathBuf,
    pub runtime_dir: PathBuf,
    pub template_dir: PathBuf,
    pub video_dir: PathBuf,
    pub resource_dir: PathBuf,
    pub temp_dir: PathBuf,
}

pub fn app_dirs(handle: tauri::AppHandle) -> Result<AppDirs> {
    let path = handle.path();

    Ok(AppDirs {
        config_dir: path.app_config_dir()?,
        data_dir: path.app_data_dir()?,
        cache_dir: path.app_cache_dir()?,
        log_dir: path.app_log_dir()?,
        local_data_dir: path.app_local_data_dir()?,
    })
}

pub fn system_dirs(handle: tauri::AppHandle) -> Result<SystemDirs> {
    let path = handle.path();

    Ok(SystemDirs {
        config_dir: path.config_dir()?,
        data_dir: path.data_dir()?,
        local_data_dir: path.local_data_dir()?,
        desktop_dir: path.desktop_dir()?,
        document_dir: path.document_dir()?,
        download_dir: path.download_dir()?,
        executable_dir: path.executable_dir()?,
        font_dir: path.font_dir()?,
        home_dir: path.home_dir()?,
        picture_dir: path.picture_dir()?,
        runtime_dir: path.runtime_dir()?,
        template_dir: path.template_dir()?,
        video_dir: path.video_dir()?,
        resource_dir: path.resource_dir()?,
        temp_dir: path.temp_dir()?,
    })
}

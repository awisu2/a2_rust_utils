use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDirs {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub log_dir: PathBuf,
    pub local_data_dir: PathBuf,
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

pub fn app_dirs(handle: tauri::AppHandle) -> AppDirs {
    let path = handle.path();

    AppDirs {
        config_dir: path.app_config_dir().unwrap_or_default(),
        data_dir: path.app_data_dir().unwrap_or_default(),
        cache_dir: path.app_cache_dir().unwrap_or_default(),
        log_dir: path.app_log_dir().unwrap_or_default(),
        local_data_dir: path.app_local_data_dir().unwrap_or_default(),
    }
}

pub fn system_dirs(handle: tauri::AppHandle) -> SystemDirs {
    let path = handle.path();

    SystemDirs {
        config_dir: path.config_dir().unwrap_or_default(),
        data_dir: path.data_dir().unwrap_or_default(),
        local_data_dir: path.local_data_dir().unwrap_or_default(),
        desktop_dir: path.desktop_dir().unwrap_or_default(),
        document_dir: path.document_dir().unwrap_or_default(),
        download_dir: path.download_dir().unwrap_or_default(),
        executable_dir: path.executable_dir().unwrap_or_default(),
        font_dir: path.font_dir().unwrap_or_default(),
        home_dir: path.home_dir().unwrap_or_default(),
        picture_dir: path.picture_dir().unwrap_or_default(),
        runtime_dir: path.runtime_dir().unwrap_or_default(),
        template_dir: path.template_dir().unwrap_or_default(),
        video_dir: path.video_dir().unwrap_or_default(),
        resource_dir: path.resource_dir().unwrap_or_default(),
        temp_dir: path.temp_dir().unwrap_or_default(),
    }
}

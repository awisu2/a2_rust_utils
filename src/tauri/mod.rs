use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDirs {
    /// アプリ設定ファイル用ディレクトリ  
    /// 例: config.json など  
    /// windows: %APPDATA%\<AppName>  
    pub config_dir: PathBuf,

    /// キャッシュ用ディレクトリ  
    /// 再生成可能な一時データ向け  
    /// windows: %LOCALAPPDATA%\<AppName>\Cache  
    pub cache_dir: PathBuf,

    /// ログファイル用ディレクトリ  
    /// 実行ログ・デバッグログの保存先  
    /// windows: %LOCALAPPDATA%\<AppName>\Logs
    pub log_dir: PathBuf,

    /// 端末固有の永続データ用ディレクトリ  
    /// アプリの設定やキャッシュ以外のデータ保存先
    /// windows: %LOCALAPPDATA%\<AppName>\Data
    pub local_data_dir: PathBuf,

    /// 永続データ用ディレクトリ  (for windows: Roaming)
    /// ユーザ間で同期されるデータ保存先
    /// DB（SQLiteなど）の保存先として想定  
    /// windows: %APPDATA%\<AppName>
    pub data_dir: PathBuf,
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

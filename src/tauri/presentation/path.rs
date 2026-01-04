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

impl Default for AppDirs {
    fn default() -> Self {
        Self {
            config_dir: PathBuf::new(),
            data_dir: PathBuf::new(),
            cache_dir: PathBuf::new(),
            log_dir: PathBuf::new(),
            local_data_dir: PathBuf::new(),
        }
    }
}

impl Default for SystemDirs {
    fn default() -> Self {
        Self {
            config_dir: PathBuf::new(),
            data_dir: PathBuf::new(),
            local_data_dir: PathBuf::new(),
            desktop_dir: PathBuf::new(),
            document_dir: PathBuf::new(),
            download_dir: PathBuf::new(),
            executable_dir: PathBuf::new(),
            font_dir: PathBuf::new(),
            home_dir: PathBuf::new(),
            picture_dir: PathBuf::new(),
            runtime_dir: PathBuf::new(),
            template_dir: PathBuf::new(),
            video_dir: PathBuf::new(),
            resource_dir: PathBuf::new(),
            temp_dir: PathBuf::new(),
        }
    }
}

pub fn app_dirs(handle: &tauri::AppHandle) -> Result<AppDirs, String> {
    let path = handle.path();

    Ok(AppDirs {
        config_dir: path.app_config_dir().map_err(|e| e.to_string())?,
        data_dir: path.app_data_dir().map_err(|e| e.to_string())?,
        cache_dir: path.app_cache_dir().map_err(|e| e.to_string())?,
        log_dir: path.app_log_dir().map_err(|e| e.to_string())?,
        local_data_dir: path.app_local_data_dir().map_err(|e| e.to_string())?,
    })
}

pub fn system_dirs(handle: &tauri::AppHandle) -> Result<SystemDirs, String> {
    let path = handle.path();

    Ok(SystemDirs {
        config_dir: path.config_dir().map_err(|e| e.to_string())?,
        data_dir: path.data_dir().map_err(|e| e.to_string())?,
        local_data_dir: path.local_data_dir().map_err(|e| e.to_string())?,
        desktop_dir: path.desktop_dir().map_err(|e| e.to_string())?,
        document_dir: path.document_dir().map_err(|e| e.to_string())?,
        download_dir: path.download_dir().map_err(|e| e.to_string())?,
        executable_dir: path.executable_dir().map_err(|e| e.to_string())?,
        font_dir: path.font_dir().map_err(|e| e.to_string())?,
        home_dir: path.home_dir().map_err(|e| e.to_string())?,
        picture_dir: path.picture_dir().map_err(|e| e.to_string())?,
        runtime_dir: path.runtime_dir().map_err(|e| e.to_string())?,
        template_dir: path.template_dir().map_err(|e| e.to_string())?,
        video_dir: path.video_dir().map_err(|e| e.to_string())?,
        resource_dir: path.resource_dir().map_err(|e| e.to_string())?,
        temp_dir: path.temp_dir().map_err(|e| e.to_string())?,
    })
}

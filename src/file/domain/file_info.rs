use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,

    pub dir: PathBuf,
    pub file_name: String,
    pub extension: String,

    pub modified: u64,
    pub created: u64,

    pub is_dir: bool,
    pub is_file: bool,
    pub is_image: bool,
    pub is_movie: bool,
}

impl Default for FileInfo {
    fn default() -> Self {
        let info = FileInfo {
            path: PathBuf::new(),
            dir: PathBuf::new(),
            file_name: String::new(),
            extension: String::new(),
            is_dir: false,
            is_file: false,
            is_image: false,
            is_movie: false,
            modified: 0,
            created: 0,
        };
        info
    }
}

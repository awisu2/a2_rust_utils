use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub file_name: String,
    pub extension: String,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_image: bool,
    pub is_movie: bool,

    pub modified: u64,
    pub created: u64,

    pub parent_path: PathBuf,
}

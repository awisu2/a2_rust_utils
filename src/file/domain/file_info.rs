use serde::{Deserialize, Serialize};
use std::{fs::DirEntry, path::PathBuf};

use crate::file::{convert_meta, is_movie, is_image};

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

impl From<PathBuf> for FileInfo {
    fn from(pathbuf: PathBuf) -> Self {
        let path = pathbuf.as_path();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let extension = path.extension().unwrap_or_default().to_string_lossy().into_owned();

        let is_dir = path.is_dir();
        let is_movie = is_movie(&extension);
        let is_image = is_image(&extension);
        let (modified, created) = convert_meta(pathbuf.metadata());

        let dir = pathbuf
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default();

        FileInfo {
            path: pathbuf.clone(),
            file_name,
            extension: extension,
            is_dir: is_dir,
            is_file: path.is_file(),
            is_movie: is_movie,
            is_image: is_image,
            modified: modified,
            created: created,
            dir: dir,
        }
    }
}

impl From<DirEntry> for FileInfo {
    fn from(entry: DirEntry) -> Self {
        let path_buf = entry.path();
        path_buf.into()
    }
}

use serde::{Deserialize, Serialize};
use std::{fs::DirEntry, path::PathBuf};

use crate::file::{domain::file_meta::FileMeta, is_image, is_movie, osstr_opt_into_string};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,

    pub dir: PathBuf,
    pub file_name: String,
    pub extension: String,

    pub meta: FileMeta,

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
            meta: FileMeta::default(),
            is_image: false,
            is_movie: false,
        };
        info
    }
}

impl From<PathBuf> for FileInfo {
    fn from(pathbuf: PathBuf) -> Self {
        let path = pathbuf.as_path();

        let dir = pathbuf
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default();
        let file_name = osstr_opt_into_string(path.file_name());
        let extension = osstr_opt_into_string(path.extension());
        let is_image_ = is_image(&extension);
        let is_movie_ = is_movie(&extension);

        let meta = FileMeta::from(path);

        FileInfo {
            path: pathbuf.clone(),
            dir: dir,
            file_name,
            extension: extension,
            meta: meta,
            is_image: is_image_,
            is_movie: is_movie_,
        }
    }
}

impl From<DirEntry> for FileInfo {
    fn from(entry: DirEntry) -> Self {
        let path_buf = entry.path();
        path_buf.into()
    }
}

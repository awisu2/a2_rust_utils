use serde::{Deserialize, Serialize};
use std::fs::FileType;
use std::{fs::DirEntry, path::PathBuf};

use crate::file::{is_image, is_movie, FileMeta};
use crate::file::{osstr_into_string, osstr_opt_into_string};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,

    pub dir: PathBuf,
    pub file_name: String,
    pub extension: String,

    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub is_image: bool,
    pub is_movie: bool,

    pub meta: Option<FileMeta>,
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
            is_symlink: false,
            is_image: false,
            is_movie: false,

            meta: None,
        };
        info
    }
}

impl From<DirEntry> for FileInfo {
    fn from(entry: DirEntry) -> Self {
        let pathbuf = entry.path();
        let file_type = entry.file_type().unwrap();
        let ext = match pathbuf.extension() {
            Some(e) => e.to_string_lossy().to_lowercase(),
            None => String::new(),
        };
        let (is_file, is_image, is_movie) = file_type_to_info(file_type, &ext);

        FileInfo {
            path: entry.path(),
            dir: pathbuf
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default(),
            file_name: osstr_opt_into_string(pathbuf.file_name()),

            extension: ext,
            is_dir: file_type.is_dir(),
            is_file: is_file,
            is_symlink: file_type.is_symlink(),
            is_image: is_image,
            is_movie: is_movie,

            meta: None,
        }
    }
}

impl From<PathBuf> for FileInfo {
    fn from(pathbuf: PathBuf) -> Self {
        // cost of IO
        let file_type = pathbuf.metadata().unwrap().file_type();
        let ext = match pathbuf.extension() {
            Some(e) => e.to_string_lossy().to_lowercase(),
            None => String::new(),
        };
        let (is_file, is_image, is_movie) = file_type_to_info(file_type, &ext);

        FileInfo {
            path: pathbuf.clone(),
            dir: pathbuf
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default(),
            file_name: osstr_opt_into_string(pathbuf.file_name()),
            extension: ext,
            is_dir: file_type.is_dir(),
            is_file: is_file,
            is_symlink: file_type.is_symlink(),
            is_image: is_image,
            is_movie: is_movie,

            meta: None,
        }
    }
}

fn file_type_to_info(file_type: FileType, ext: &str) -> (bool, bool, bool) {
    let is_file = file_type.is_file();

    let (is_image, is_movie) = if is_file {
        let (is_image, is_movie) = { (is_image(ext), is_movie(ext)) };

        (is_image, is_movie)
    } else {
        (false, false)
    };

    (is_file, is_image, is_movie)
}

impl FileInfo {
    pub fn path_string(&self) -> String {
        self.path.to_string_lossy().into_owned()
    }

    pub fn dir_string(&self) -> String {
        self.dir.to_string_lossy().into_owned()
    }
}

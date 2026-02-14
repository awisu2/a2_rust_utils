use serde::{Deserialize, Serialize};
use std::fs::FileType;
use std::path::Path;
use std::{fs::DirEntry, path::PathBuf};

use crate::file::path_ex::PathEx;
use crate::file::{is_image, is_movie, FileMeta};
use crate::file::{is_zip, osstr_opt_into_string};

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
    pub is_zip: bool,

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
            is_zip: false,

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
        let (is_dir, is_file, is_image, is_movie, is_zip) = file_type_to_info(file_type, &ext);

        FileInfo {
            path: PathBuf::from(entry.path().to_string_lossy().to_string()),
            dir: pathbuf
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default(),
            file_name: osstr_opt_into_string(pathbuf.file_name()),

            extension: ext.clone(),
            is_dir: is_dir,
            is_file: is_file,
            is_symlink: file_type.is_symlink(),
            is_image: is_image,
            is_movie: is_movie,
            is_zip: is_zip,

            meta: None,
        }
    }
}

impl From<&Path> for FileInfo {
    fn from(pathbuf: &Path) -> Self {
        FileInfo::from_pathbuf(pathbuf, false)
    }
}

fn file_type_to_info(file_type: FileType, ext: &str) -> (bool, bool, bool, bool, bool) {
    let is_file = file_type.is_file();
    let is_dir = file_type.is_dir();

    let (is_image, is_movie, is_zip) = if is_file {
        let (is_image, is_movie) = { (is_image(ext), is_movie(ext)) };
        let is_zip = is_zip(ext);

        (is_image, is_movie, is_zip)
    } else {
        (false, false, false)
    };

    (is_dir, is_file, is_image, is_movie, is_zip)
}

impl FileInfo {
    pub fn path_string(&self) -> String {
        self.path.to_string_ex()
    }

    pub fn dir_string(&self) -> String {
        self.dir.to_string_ex()
    }

    pub fn from_str(path: &str, is_load_meta: bool) -> Self {
        let pathbuf = PathBuf::from(path);
        FileInfo::from_pathbuf(pathbuf.as_path(), is_load_meta)
    }

    pub fn from_pathbuf(path: &Path, is_load_meta: bool) -> Self {
        let mut info = FileInfo::from_pathbuf_(path);
        if is_load_meta {
            info.load_meta();
        }
        info
    }

    // only get from pathbuf, without load meta, for performance
    fn from_pathbuf_(pathbuf: &Path) -> Self {
        let path_str = pathbuf.to_string_ex();

        let dir = match pathbuf.parent() {
            Some(p) => p.to_path_buf(),
            None => PathBuf::new(),
        };

        let ext = pathbuf
            .extension()
            .map(|e| e.to_string_ex().to_lowercase())
            .unwrap_or_default();

        let is_dir = path_str.ends_with("/") || path_str.ends_with(std::path::MAIN_SEPARATOR);
        let is_file = !is_dir;

        let is_zip = is_zip(&ext);
        let is_image = is_image(&ext);
        let is_movie = is_movie(&ext);

        FileInfo {
            path: pathbuf.to_path_buf(),
            dir: dir,
            file_name: osstr_opt_into_string(pathbuf.file_name()),
            extension: ext,

            is_dir,
            is_file,
            is_symlink: false, // 判別不可

            is_image,
            is_movie,
            is_zip,

            meta: None,
        }
    }

    pub fn load_meta(&mut self) -> Self {
        // load meta (IO cost) =====
        let pathbuf = self.path.to_path_buf();
        let meta = match pathbuf.metadata() {
            Ok(meta) => meta,
            Err(_) => return self.clone(),
        };

        // get type from meta =====
        let file_type = meta.file_type();
        let ext = match pathbuf.extension() {
            Some(e) => e.to_string_lossy().to_lowercase(),
            None => String::new(),
        };
        (
            self.is_dir,
            self.is_file,
            self.is_image,
            self.is_movie,
            self.is_zip,
        ) = file_type_to_info(file_type, &ext);

        // set loaded meta =====
        let meta = FileMeta::from(&meta);
        self.meta = Some(meta);
        self.clone()
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    // test from PathBuf
    #[test]
    fn test_file_info_from_pathbuf() {
        // generate test file
        let test_dir = "test_data";
        let test_file = "image1.jpg";
        std::fs::create_dir_all(test_dir).unwrap();
        std::fs::write(format!("{}/{}", test_dir, test_file), b"test").unwrap();

        let pathbuf = PathBuf::from(format!("{}/{}", test_dir, test_file));

        let file_info = FileInfo::from_pathbuf(pathbuf.as_path(), true);

        assert_eq!(file_info.file_name, "image1.jpg");
        assert_eq!(file_info.extension, "jpg");
        assert_eq!(file_info.is_file, true);
        assert_eq!(file_info.is_image, true);
        assert_eq!(file_info.is_movie, false);
        assert_eq!(file_info.is_dir, false);
        assert_eq!(file_info.is_zip, false);
        assert_eq!(file_info.path_string(), "test_data/image1.jpg");
        assert_eq!(file_info.dir_string(), "test_data");
        assert_eq!(file_info.meta.is_some(), true);

        // clean up
        std::fs::remove_file(format!("{}/{}", test_dir, test_file)).unwrap();
        std::fs::remove_dir(test_dir).unwrap();
    }

    #[test]
    fn test_file_info_dir_from_pathbuf() {
        // generate test file
        let test_dir: &str = &(format!("test_data_dir_{}", DIR_SEPARATOR));
        std::fs::create_dir_all(test_dir).unwrap();

        let pathbuf = PathBuf::from(test_dir);

        let file_info = FileInfo::from_pathbuf(pathbuf.as_path(), true);

        assert_eq!(file_info.file_name, test_dir);
        assert_eq!(file_info.extension, "");
        assert_eq!(file_info.is_file, false);
        assert_eq!(file_info.is_image, false);
        assert_eq!(file_info.is_movie, false);
        assert_eq!(file_info.is_dir, true);
        assert_eq!(file_info.path_string(), test_dir);
        assert_eq!(file_info.dir_string(), "");
        assert_eq!(file_info.meta.is_some(), true);
        assert_eq!(file_info.is_zip, false);

        // clean up
        std::fs::remove_dir(test_dir).unwrap();
    }
}

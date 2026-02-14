use serde::{Deserialize, Serialize};
use std::fs::FileType;
use std::{fs::DirEntry, path::PathBuf};

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
        let (is_file, is_image, is_movie, is_zip) = file_type_to_info(file_type, &ext);

        FileInfo {
            path: entry.path(),
            dir: pathbuf
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default(),
            file_name: osstr_opt_into_string(pathbuf.file_name()),

            extension: ext.clone(),
            is_dir: file_type.is_dir(),
            is_file: is_file,
            is_symlink: file_type.is_symlink(),
            is_image: is_image,
            is_movie: is_movie,
            is_zip: is_zip,

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
        let (is_file, is_image, is_movie, is_zip) = file_type_to_info(file_type, &ext);

        let meta = FileMeta::from(pathbuf.as_path());

        FileInfo {
            path: pathbuf.clone(),
            dir: pathbuf
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default(),
            file_name: osstr_opt_into_string(pathbuf.file_name()),
            extension: ext.clone(),
            is_dir: file_type.is_dir(),
            is_file: is_file,
            is_symlink: file_type.is_symlink(),
            is_image: is_image,
            is_movie: is_movie,
            is_zip: is_zip,

            meta: Some(meta),
        }
    }
}

fn file_type_to_info(file_type: FileType, ext: &str) -> (bool, bool, bool, bool) {
    let is_file = file_type.is_file();

    let (is_image, is_movie, is_zip) = if is_file {
        let (is_image, is_movie) = { (is_image(ext), is_movie(ext)) };
        let is_zip = is_zip(ext);

        (is_image, is_movie, is_zip)
    } else {
        (false, false, false)
    };

    (is_file, is_image, is_movie, is_zip)
}

impl FileInfo {
    pub fn path_string(&self) -> String {
        self.path.to_string_lossy().into_owned()
    }

    pub fn dir_string(&self) -> String {
        self.dir.to_string_lossy().into_owned()
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

        let file_info = FileInfo::from(pathbuf);

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
        let test_dir = "test_dir";
        std::fs::create_dir_all(test_dir).unwrap();

        let pathbuf = PathBuf::from(test_dir);

        let file_info = FileInfo::from(pathbuf);

        assert_eq!(file_info.file_name, "test_dir");
        assert_eq!(file_info.extension, "");
        assert_eq!(file_info.is_file, false);
        assert_eq!(file_info.is_image, false);
        assert_eq!(file_info.is_movie, false);
        assert_eq!(file_info.is_dir, true);
        assert_eq!(file_info.path_string(), "test_dir");
        assert_eq!(file_info.dir_string(), "");
        assert_eq!(file_info.meta.is_some(), true);
        assert_eq!(file_info.is_zip, false);

        // clean up
        std::fs::remove_dir(test_dir).unwrap();
    }
}

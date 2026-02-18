use serde::{Deserialize, Serialize};
use std::fs::FileType;
use std::path::Path;
use std::{fs::DirEntry, path::PathBuf};

use crate::file::domain::zip_infos::ZipInfo;
use crate::file::path_ex::PathEx;
use crate::file::{is_image, is_movie, FileMeta};
use crate::file::{is_zip, OptionPathEx};

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
    pub zip_info: Option<ZipInfo>,
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
            zip_info: None,
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
            Some(e) => e.to_string_ex().to_lowercase(),
            None => String::new(),
        };
        let (is_dir, is_file, is_image, is_movie, is_zip) = file_type_to_info(file_type, &ext);

        FileInfo {
            path: entry.path(),
            dir: pathbuf
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default(),
            file_name: pathbuf.file_name().to_string_ex(),

            extension: ext.clone(),
            is_dir: is_dir,
            is_file: is_file,
            is_symlink: file_type.is_symlink(),
            is_image: is_image,
            is_movie: is_movie,
            is_zip: is_zip,
            zip_info: None,

            meta: None,
        }
    }
}

impl From<&ZipInfo> for FileInfo {
    fn from(zip_info: &ZipInfo) -> Self {
        let pathbuf = PathBuf::from(&zip_info.full_path());

        let (ext, is_image, is_movie, is_zip) = if zip_info.is_file {
            let ext = match Path::new(&zip_info.name).extension() {
                Some(e) => e.to_string_ex().to_lowercase(),
                None => String::new(),
            };
            let is_image = is_image(&ext);
            let is_movie = is_movie(&ext);
            let is_zip = is_zip(&ext);

            (ext, is_image, is_movie, is_zip)
        } else {
            (String::new(), false, false, false)
        };

        FileInfo {
            path: pathbuf.clone(),
            dir: pathbuf
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default(),
            file_name: pathbuf.file_name().to_string_ex(),

            extension: ext,
            is_dir: zip_info.is_dir,
            is_file: zip_info.is_file,
            is_symlink: false,
            is_image: is_image,
            is_movie: is_movie,
            is_zip: is_zip,
            zip_info: Some(zip_info.clone()),

            meta: None,
        }
    }
}

impl From<&Path> for FileInfo {
    fn from(pathbuf: &Path) -> Self {
        FileInfo::from_path(pathbuf)
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
    pub fn in_zip(&self) -> bool {
        self.zip_info.is_some()
    }

    pub fn path_string(&self) -> String {
        self.path.to_string_ex()
    }

    pub fn dir_string(&self) -> String {
        self.dir.to_string_ex()
    }

    pub fn from_str(path: &str) -> Self {
        let pathbuf = PathBuf::from(path);
        FileInfo::from_path(pathbuf.as_path())
    }

    pub fn from_path(path: &Path) -> Self {
        let path_str = path.to_string_ex();

        let dir = match path.parent() {
            Some(p) => p.to_path_buf(),
            None => PathBuf::new(),
        };

        let ext = path.extension().to_string_ex().to_ascii_lowercase();
        let is_entd_sep = path_str.ends_with("/") || path_str.ends_with(std::path::MAIN_SEPARATOR);
        let is_dir = is_entd_sep;
        let is_file = !is_dir;

        let is_zip = is_zip(&ext);
        let is_image = is_image(&ext);
        let is_movie = is_movie(&ext);

        // remove trailing separator for consistent path representation
        let new_path = if is_entd_sep {
            let new_path_str = path_str
                .trim_end_matches("/")
                .trim_end_matches(std::path::MAIN_SEPARATOR)
                .to_string();
            PathBuf::from(new_path_str)
        } else {
            path.to_path_buf()
        };

        let delimiter = ".zip/";
        let mut zip_info: Option<ZipInfo> = None;
        if path_str.contains(delimiter) {
            let (zip_path, name) = match path_str.split_once(delimiter) {
                Some((zip_path, name)) => (format!("{zip_path}.zip"), name),
                None => (path_str.clone(), ""),
            };
            zip_info = Some(ZipInfo {
                index: 0,
                zip_path: zip_path.to_string(),
                name: name.to_string(),
                is_dir: is_dir,
                is_file: is_file,
                size: 0,
            });
        }

        FileInfo {
            path: new_path,
            // path: path.to_path_buf(),
            dir: dir,
            file_name: path.file_name().to_string_ex(),
            extension: ext,

            is_dir,
            is_file,
            is_symlink: false, // 判別不可

            is_image,
            is_movie,
            is_zip,

            meta: None,
            zip_info: zip_info,
        }
    }

    // metadata を読み込んで更に詳細な情報を取得(ただしIOコストあり)
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
            Some(e) => e.to_string_ex().to_lowercase(),
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
    use crate::file::path_ex::DIR_SEPARATOR;

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

        let file_info = FileInfo::from_path(pathbuf.as_path()).load_meta();

        assert_eq!(file_info.file_name, "image1.jpg");
        assert_eq!(file_info.extension, "jpg");
        assert_eq!(file_info.is_file, true);
        assert_eq!(file_info.is_image, true);
        assert_eq!(file_info.is_movie, false);
        assert_eq!(file_info.is_dir, false);
        assert_eq!(file_info.is_zip, false);
        assert_eq!(file_info.zip_info.is_none(), true);
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
        let test_dir_base: &str = "test_data_dir_";
        let test_dir: &str = &(format!("{}{}", test_dir_base, DIR_SEPARATOR));
        std::fs::create_dir_all(test_dir).unwrap();

        let pathbuf = PathBuf::from(test_dir);

        let file_info = FileInfo::from_path(pathbuf.as_path());

        assert_eq!(file_info.file_name, test_dir_base);
        assert_eq!(file_info.extension, "");
        assert_eq!(file_info.is_file, false);
        assert_eq!(file_info.is_image, false);
        assert_eq!(file_info.is_movie, false);
        assert_eq!(file_info.is_dir, true);
        assert_eq!(file_info.zip_info.is_none(), true);
        assert_eq!(file_info.path_string(), test_dir_base);
        assert_eq!(file_info.dir_string(), "");
        assert_eq!(file_info.meta.is_none(), true);
        assert_eq!(file_info.is_zip, false);

        // clean up
        std::fs::remove_dir(test_dir).unwrap();
    }
}

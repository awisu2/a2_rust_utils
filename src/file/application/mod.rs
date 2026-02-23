use crate::file::domain::file_info::FileInfo;
use anyhow::{anyhow, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub mod zip_util;

static MOVIE_EXTENSIONS: &[&str] = &["mp4", "mpeg", "mpg", "avi", "mov"];
static IMAGE_EXTENSIONS: &[&str] = &["jpeg", "jpg", "gif", "webp", "png"];
static ZIP_EXTENSIONS: &[&str] = &["zip"];

pub fn is_movie(extension: &str) -> bool {
    MOVIE_EXTENSIONS.contains(&extension)
}

pub fn is_image(extension: &str) -> bool {
    IMAGE_EXTENSIONS.contains(&extension)
}

pub fn is_zip(extension: &str) -> bool {
    ZIP_EXTENSIONS.contains(&extension)
}

/// Read directory and return file infos.  
/// It include all type (file, dirctory, symlink, etc...) infos
pub fn read_dir(dir: &str) -> Result<Vec<FileInfo>> {
    let mut vec: Vec<FileInfo> = Vec::new();

    let path = Path::new(dir);
    let read_dir = fs::read_dir(path)?;
    for entry in read_dir {
        let entry = entry?;
        let file_info = FileInfo::from(entry);
        vec.push(file_info);
    }

    Ok(vec)
}

pub fn read_dir_deep(dir: &str, deep: usize) -> Result<Vec<FileInfo>> {
    read_dir_deep_(dir, deep, 0)
}

fn read_dir_deep_(dir: &str, max_deep: usize, deep: usize) -> Result<Vec<FileInfo>> {
    if deep > max_deep {
        return Ok(Vec::new());
    }

    let mut infos = read_dir(dir)?;
    if deep < max_deep {
        let mut children = Vec::<FileInfo>::new();
        let dirs: Vec<&FileInfo> = infos.iter().filter(|info| info.is_dir).collect();
        for dir in &dirs {
            let children_ = read_dir_deep_(&dir.path_string(), max_deep, deep + 1)?;
            children.extend(children_);
        }
        infos.extend(children);
    }
    return Ok(infos);
}

/// File existence check
pub fn is_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn rename(from: &str, to: &str) -> Result<()> {
    fs::rename(from, to).map_err(|e| anyhow!(e))
}

pub fn remove_dir_all(path: &str) -> Result<()> {
    fs::remove_dir_all(path).map_err(|e| anyhow!(e))
}

pub fn write(path: PathBuf, data: &[u8]) -> Result<()> {
    if let Some(dir) = path.parent() {
        // ディレクトリ作成(create_dir_all は既存でもエラーにならない)
        fs::create_dir_all(dir)?;
    }

    let mut file = File::create(path.as_path())?;
    file.write_all(data).map_err(|e| anyhow!(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_dir_no_target() {
        let dir = "notargetdir";
        let res = read_dir(dir);
        assert!(res.is_err());
    }

    #[test]
    fn test_write() {
        let path: PathBuf = PathBuf::from("./test_file");
        let data: &[u8] = b"Hello world";
        let res = write(path, data);
        assert!(res.is_ok());
    }

    #[test]
    fn test_read_dir() {
        let path: &str = "./local";
        let _ = read_dir(path);
    }

    #[test]
    fn test_read_dir_deep() {
        let test_dir = "test_read_dir_deep";
        let sub_dir = "subdir";
        let test_file = "file.txt";

        // create test directory structure
        std::fs::create_dir_all(format!("{}/{}", test_dir, sub_dir)).unwrap();
        std::fs::write(format!("{}/{}", test_dir, test_file), b"test").unwrap();
        std::fs::write(format!("{}/{}/{}", test_dir, sub_dir, test_file), b"test").unwrap();

        let infos = crate::file::application::read_dir_deep(test_dir, 2).unwrap();

        // There should be 4 entries: test_dir/, test_dir/file.txt, test_dir/subdir/, test_dir/subdir/file.txt
        assert_eq!(infos.len(), 3);

        // clean up
        std::fs::remove_dir_all(test_dir).unwrap();
    }
}

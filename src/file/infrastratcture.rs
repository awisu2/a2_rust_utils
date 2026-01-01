use anyhow::{anyhow, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::file::domain::FileInfo;

use crate::time::Timestamp;
use std::io;

static MOVIE_EXTENSIONS: &[&str] = &[
    "mp4",
    "mpeg",
    "mpg",
    "avi",
    "mov",
];

static IMAGE_EXTENSIONS: &[&str] = &[
    "jpeg",
    "jpg",
    "gif",
    "webp",
    "png",
];

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

/// File existence check
pub fn is_exists(path: &str) -> bool {
    Path::new(path).exists()
}

// get info from metadata
// Note: Errの場合気にせず0を返す 運用
pub fn convert_meta(meta: io::Result<fs::Metadata>) -> (u64, u64) {
    let _meta = match meta {
        Ok(m) => m,
        Err(_) => return (0, 0),
    };

    // get modified and created time
    let modified = _meta.modified().map(Timestamp::from_system_time).unwrap_or(0);
    let created = _meta.created().map(Timestamp::from_system_time).unwrap_or(0);
    (modified, created)
}

pub fn is_movie(extension: &str) -> bool {
    MOVIE_EXTENSIONS.contains(&extension)
}

pub fn is_image(extension: &str) -> bool {
    IMAGE_EXTENSIONS.contains(&extension)
}

pub fn rename(from: &str, to: &str) -> Result<()> {
    fs::rename(from, to)?;
    Ok(())
}

pub fn remove_dir_all(path: &str) -> Result<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}

pub fn write(path: PathBuf, data: &[u8]) -> Result<()> {
    let dir = path.parent().ok_or(anyhow!(""))?;

    // 既存ディレクトリがあってもエラーにはならない
    fs::create_dir_all(dir)?;

    // mkfile =====
    let mut file = File::create(path.as_path())?;
    file.write_all(data)?;
    Ok(())
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
}

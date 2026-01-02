use anyhow::{anyhow, Result};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::file::domain::file_meta::FileMeta;
use crate::{file::domain::file_info::FileInfo, time::Timestamp};

static MOVIE_EXTENSIONS: &[&str] = &["mp4", "mpeg", "mpg", "avi", "mov"];
static IMAGE_EXTENSIONS: &[&str] = &["jpeg", "jpg", "gif", "webp", "png"];

pub fn is_movie(extension: &str) -> bool {
    MOVIE_EXTENSIONS.contains(&extension)
}

pub fn is_image(extension: &str) -> bool {
    IMAGE_EXTENSIONS.contains(&extension)
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

/// File existence check
pub fn is_exists(path: &str) -> bool {
    Path::new(path).exists()
}

// get info from metadata
// Note: Errの場合気にせず0を返す 運用
// is_dir, is_file, is_symlink, modified, created, size
pub fn from_result_meta(meta: io::Result<fs::Metadata>) -> FileMeta {
    let _meta = match meta {
        Ok(m) => m,
        Err(_) => return FileMeta::default(),
    };
    from_meta(_meta)
}

pub fn from_meta(meta: fs::Metadata) -> FileMeta {
    // get modified and created time
    let modified = meta
        .modified()
        .map(Timestamp::from_system_time)
        .unwrap_or(0);
    let created = meta.created().map(Timestamp::from_system_time).unwrap_or(0);

    FileMeta {
        is_dir: meta.is_dir(),
        is_file: meta.is_file(),
        is_symlink: meta.is_symlink(),
        modified,
        created,
        size: meta.len(),
    }
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

pub fn osstr_into_string(v: &OsStr) -> String {
    v.to_string_lossy().into_owned()
}

pub fn osstr_opt_into_string(v: Option<&OsStr>) -> String {
    osstr_into_string(v.unwrap_or_default())
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

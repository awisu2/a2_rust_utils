use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::path;

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

/// Read directory and return file infos.  
/// It include all type (file, dirctory, symlink, etc...) infos
pub fn read_dir(dir: &str) -> Result<Vec<FileInfo>> {
    let mut vec: Vec<FileInfo> = Vec::new();

    let path = Path::new(dir);
    let read_dir = fs::read_dir(path).context("")?;
    for entry in read_dir {
        let entry = entry?;
        let file_info = pathbuf_to_fileinfo(entry.path());
        vec.push(file_info);
    }

    Ok(vec)
}

/// convert &str to PathBuf and get Fileinfo
pub fn str_to_fileinfo(str: &str) -> FileInfo {
    let pathbuf = PathBuf::from(str);
    pathbuf_to_fileinfo(pathbuf)
}

/// File existence check
pub fn is_exists_file(path: &str) -> bool {
    let pathbuf = PathBuf::from(path);
    pathbuf.exists()
}

/// Return FileInfo(custom format) convert from Pathbuf
fn pathbuf_to_fileinfo(pathbuf: PathBuf) -> FileInfo {
    let _pathbuf = pathbuf.clone();
    let path = _pathbuf.as_path();
    let file_name = path::opt_osstr_to_string(path.file_name(), "");
    let extension = path::opt_osstr_to_string(path.extension(), "");

    let is_dir = path.is_dir();
    let is_movie = is_movie(extension.as_str());
    let is_image = is_image(extension.as_str());

    let (modified, created) = match pathbuf.metadata() {
        Ok(v) => (
            system_time_to_u64(v.modified().unwrap()),
            system_time_to_u64(v.created().unwrap()),
        ),
        Err(_) => (0, 0),
    };
    let parnt_path = pathbuf.parent().unwrap().to_path_buf();

    FileInfo {
        path: pathbuf,
        file_name,
        extension: extension,
        is_dir: is_dir,
        is_file: path.is_file(),
        is_movie: is_movie,
        is_image: is_image,
        modified: modified,
        created: created,
        parent_path: parnt_path,
    }
}

fn is_movie(extension: &str) -> bool {
    return extension == "mp4"
        || extension == "mpeg"
        || extension == "mpg"
        || extension == "avi"
        || extension == "mov";
}

fn is_image(extension: &str) -> bool {
    return extension == "jpeg"
        || extension == "jpg"
        || extension == "gif"
        || extension == "webp"
        || extension == "png";
}

fn system_time_to_u64(system_time: SystemTime) -> u64 {
    let duration = match system_time.duration_since(UNIX_EPOCH) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    duration.as_secs()
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
}

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::commands::path;

#[derive(Serialize, Deserialize, Debug)]
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

pub fn read_dir(dir: &str) -> Result<Vec<FileInfo>, String> {
    let mut vec: Vec<FileInfo> = Vec::new();

    let path = Path::new(dir);
    if !path.is_dir() {
        return Err(format!("not directory."));
    }

    let read_dir = match fs::read_dir(path) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    for entry in read_dir {
        let entry = match entry {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        let file_info = pathbuf_to_fileinfo(entry.path());
        vec.push(file_info);
    }

    Ok(vec)
}

pub fn str_to_fileinfo(str: &str) -> FileInfo {
    let pathbuf = PathBuf::from(str);
    pathbuf_to_fileinfo(pathbuf)
}

pub fn is_exists_file(path: &str) -> bool {
    let pathbuf = PathBuf::from(path);
    pathbuf.exists()
}

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

pub fn rename(from: &str, to: &str) -> Result<(), String> {
    match fs::rename(from, to) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
}

pub fn remove_dir_all(path: &str) -> Result<(), String> {
    fs::remove_dir_all(path).map_err(|e| e.to_string())
}

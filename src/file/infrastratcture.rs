use anyhow::{anyhow, Context, Result};
use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::file::domain::FileInfo;

use crate::path::{osstr_opt_to_string, osstr_to_string};
use crate::time::Timestamp;
use std::io;

/// Read directory and return file infos.  
/// It include all type (file, dirctory, symlink, etc...) infos
pub fn read_dir(dir: &str) -> Result<Vec<FileInfo>> {
    let mut vec: Vec<FileInfo> = Vec::new();

    let path = Path::new(dir);
    let read_dir = fs::read_dir(path).context("")?;
    for entry in read_dir {
        let entry = entry?;
        let file_info = direntry_to_fileinfo(entry)?;
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
    let file_name = osstr_opt_to_string(path.file_name());
    let extension = osstr_opt_to_string(path.extension());

    let is_dir = path.is_dir();
    let is_movie = is_movie(&extension);
    let is_image = is_image(&extension);
    let (modified, created) = convert_meta(pathbuf.metadata());

    let dir = pathbuf
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default();

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
        dir: dir,
    }
}

fn direntry_to_fileinfo(entry: DirEntry) -> Result<FileInfo> {
    let file_type = entry.file_type()?;

    let file_name = osstr_to_string(&entry.file_name());

    let (modified, created) = convert_meta(entry.metadata());

    let pathbuf = entry.path();
    let path = pathbuf.as_path();
    let extension = osstr_opt_to_string(path.extension());
    let is_movie = is_movie(extension.as_str());
    let is_image = is_image(extension.as_str());

    let dir = pathbuf.parent().unwrap().to_path_buf();

    Ok(FileInfo {
        path: pathbuf,
        dir: dir,
        file_name: file_name,
        extension: extension,
        is_dir: file_type.is_dir(),
        is_file: file_type.is_file(),
        is_movie: is_movie,
        is_image: is_image,
        modified: modified,
        created: created,
    })
}

// get info from metadata
// Note: Errの場合気にせず0を返す 運用
fn convert_meta(meta: io::Result<fs::Metadata>) -> (u64, u64) {
    let _meta = match meta {
        Ok(m) => m,
        Err(_) => return (0, 0),
    };

    // get modified and created time
    let modified = _meta.modified().map(Timestamp::from_system_time).unwrap_or(0);
    let created = _meta.created().map(Timestamp::from_system_time).unwrap_or(0);
    (modified, created)
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

use anyhow::{anyhow, Context, Result};
use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::file::domain::FileInfo;

use crate::path::{osstr_opt_to_string, osstr_to_string};

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

fn direntry_to_fileinfo(entry: DirEntry) -> Result<FileInfo> {
    let file_type = entry.file_type()?;

    let file_name = osstr_to_string(&entry.file_name());

    let (modified, created) = match entry.metadata() {
        Ok(v) => (
            system_time_to_u64(v.modified().unwrap()),
            system_time_to_u64(v.created().unwrap()),
        ),
        Err(_) => (0, 0),
    };

    let pathbuf = entry.path();
    let path = pathbuf.as_path();
    let extension = osstr_opt_to_string(path.extension());
    let is_movie = is_movie(extension.as_str());
    let is_image = is_image(extension.as_str());

    let parent_path = pathbuf.parent().unwrap().to_path_buf();

    Ok(FileInfo {
        path: pathbuf,
        file_name: file_name,
        extension: extension,
        is_dir: file_type.is_dir(),
        is_file: file_type.is_file(),
        is_movie: is_movie,
        is_image: is_image,
        modified: modified,
        created: created,
        parent_path: parent_path,
    })
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

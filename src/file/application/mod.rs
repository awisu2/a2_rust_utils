use crate::file::domain::file_info::FileInfo;
use crate::file::FileMeta;
use anyhow::{anyhow, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{FILETIME, INVALID_HANDLE_VALUE};
#[cfg(target_os = "windows")]
use windows::Win32::Storage::FileSystem::{
    FindFirstFileW, FindNextFileW, FILE_ATTRIBUTE_DIRECTORY, WIN32_FIND_DATAW,
};

pub mod zip_util;

static MOVIE_EXTENSIONS: &[&str] = &["mp4", "mpeg", "mpg", "avi", "mov", "webm"];
static IMAGE_EXTENSIONS: &[&str] = &["jpeg", "jpg", "gif", "webp", "png"];
static ZIP_EXTENSIONS: &[&str] = &["zip"];
const WINDOWS_TO_UNIX_EPOCH: u64 = 116444736000000000;

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
#[cfg(not(target_os = "windows"))]
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

#[cfg(target_os = "windows")]
fn wide_cstr_to_osstring(buf: &[u16]) -> OsString {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    OsString::from_wide(&buf[..len])
}

// #[cfg(target_os = "windows")]
// fn filetime_to_u64(ft: FILETIME) -> u64 {
//     ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
// }

#[cfg(target_os = "windows")]
fn filetime_to_unix_seconds(ft: FILETIME) -> u64 {
    let v = ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64);
    (v - WINDOWS_TO_UNIX_EPOCH) / 10_000_000
}

#[cfg(target_os = "windows")]
pub fn read_dir(dir: &str) -> Result<Vec<FileInfo>> {
    // windows codes
    unsafe {
        let mut data = WIN32_FIND_DATAW::default();

        // search files under {dir} =====
        let pattern = format!(r"{}\*", dir);
        let pattern: Vec<u16> = pattern.encode_utf16().chain(Some(0)).collect();

        let handle = FindFirstFileW(PCWSTR(pattern.as_ptr()), &mut data)?;
        if handle == INVALID_HANDLE_VALUE {
            return Err(anyhow!("Failed to find first file"));
        }

        // convert =====
        let mut vec: Vec<FileInfo> = Vec::new();
        loop {
            let name = wide_cstr_to_osstring(&data.cFileName);
            let name_str = name.to_string_lossy();

            // "." と ".." を除外
            if name_str != "." && name_str != ".." {
                let full_path = std::path::Path::new(dir).join(&*name_str);
                let full_path_buf = PathBuf::from(full_path);

                let meta = FileMeta {
                    modified: filetime_to_unix_seconds(data.ftLastWriteTime),
                    created: filetime_to_unix_seconds(data.ftCreationTime),
                    size: ((data.nFileSizeHigh as u64) << 32) | (data.nFileSizeLow as u64),
                };

                let mut info = FileInfo::from_path(&full_path_buf);
                info.is_dir = data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY.0 != 0;
                info.is_file = !info.is_dir;
                info.meta = Some(meta);

                vec.push(info);
            }

            // finish all files =====
            if FindNextFileW(handle, &mut data).is_err() {
                break;
            }
        }

        Ok(vec)
    }
}

pub fn read_dir_deep(dir: &str, deep: usize) -> Result<Vec<FileInfo>> {
    read_dir_deep_(dir, deep, 0)
}

fn read_dir_deep_(dir: &str, max_deep: usize, deep: usize) -> Result<Vec<FileInfo>> {
    if deep >= max_deep {
        return Ok(Vec::new());
    }

    let mut infos = read_dir(dir)?;
    let next_deep = deep + 1;
    if next_deep < max_deep {
        let mut children = Vec::<FileInfo>::new();
        let dirs: Vec<&FileInfo> = infos.iter().filter(|info| info.is_dir).collect();
        for dir in &dirs {
            let children_ = read_dir_deep_(&dir.path_string(), max_deep, next_deep)?;
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

// Move file or directory. It also fix dir if needed.
pub fn move_file(from: &str, to: &str) -> Result<()> {
    if !fs::exists(from)? {
        return Err(anyhow!("Source path does not exist. Path: {}", from));
    }

    // create dir if not exists
    if let Some(parent) = Path::new(to).parent() {
        fs::create_dir_all(parent)?;
    }

    // move file and dir with inner files
    fs::rename(from, to).map_err(|e| anyhow!(e))
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
        // let res = read_dir(path);
        // loop {
        //     match res {
        //         Ok(infos) => {
        //             for info in infos {
        //                 println!("--{:?}", info.file_name);
        //             }
        //             break;
        //         }
        //         Err(e) => {
        //             println!("Error: {}", e);
        //             break;
        //         }
        //     }
        // }
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

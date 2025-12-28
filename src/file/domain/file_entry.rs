use crate::file::domain::file_info::FileInfo;
use serde::{Deserialize, Serialize};

// FileInfo の拡張情報をジェネリクスで持てるようにしたもの
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry<T> {
    pub info: FileInfo,
    pub meta: T,
    pub children: Vec<FileEntry<T>>,
}

impl<T: Default> Default for FileEntry<T> {
    fn default() -> Self {
        FileEntry {
            info: Default::default(),
            meta: Default::default(),
            children: Vec::new(),
        }
    }
}

impl<T> From<(FileInfo, T)> for FileEntry<T> {
    fn from(value: (FileInfo, T)) -> Self {
        FileEntry {
            info: value.0,
            meta: value.1,
            children: Vec::new(),
        }
    }
}

impl<T: Default> From<FileInfo> for FileEntry<T> {
    fn from(value: FileInfo) -> Self {
        FileEntry {
            info: value,
            meta: Default::default(),
            children: Vec::new(),
        }
    }
}

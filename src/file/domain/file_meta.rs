use std::{fs::Metadata, path::Path};

use serde::{Deserialize, Serialize};

use crate::time::Timestamp;

// get meta infor from fs::Metadata
// because only one IO operation per Metadata fetch,
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMeta {
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub modified: u64, // Timestamp
    pub created: u64,  // Timestamp
    pub size: u64,
}

impl Default for FileMeta {
    fn default() -> Self {
        FileMeta {
            is_dir: false,
            is_file: false,
            is_symlink: false,
            modified: 0,
            created: 0,
            size: 0,
        }
    }
}

impl From<Metadata> for FileMeta {
    fn from(meta: Metadata) -> Self {
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
}

impl From<&Path> for FileMeta {
    fn from(path_buf: &Path) -> Self {
        match path_buf.metadata() {
            Ok(meta) => FileMeta::from(meta),
            Err(_) => FileMeta::default(),
        }
    }
}

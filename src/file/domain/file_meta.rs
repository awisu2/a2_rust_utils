use std::{fs::Metadata, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    file::{is_image, is_movie},
    time::Timestamp,
};

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

    pub is_image: bool,
    pub is_movie: bool,
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
            is_image: false,
            is_movie: false,
        }
    }
}

impl From<&Path> for FileMeta {
    fn from(path: &Path) -> Self {
        let meta = match path.metadata() {
            Ok(meta) => meta,
            Err(_) => return FileMeta::default(),
        };

        let modified = meta
            .modified()
            .map(Timestamp::from_system_time)
            .unwrap_or(0);
        let created = meta.created().map(Timestamp::from_system_time).unwrap_or(0);

        let (is_image_, is_movie_) = if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            (is_image(&ext), is_movie(&ext))
        } else {
            (false, false)
        };

        FileMeta {
            is_dir: meta.is_dir(),
            is_file: meta.is_file(),
            is_symlink: meta.is_symlink(),
            modified,
            created,
            size: meta.len(),
            is_image: is_image_,
            is_movie: is_movie_,
        }
    }
}

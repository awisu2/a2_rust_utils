use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::time::Timestamp;

// get meta infor from fs::Metadata
// because only one IO operation per Metadata fetch,
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMeta {
    pub modified: u64, // Timestamp
    pub created: u64,  // Timestamp
    pub size: u64,
}

impl Default for FileMeta {
    fn default() -> Self {
        FileMeta {
            modified: 0,
            created: 0,
            size: 0,
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

        FileMeta {
            modified,
            created,
            size: meta.len(),
        }
    }
}

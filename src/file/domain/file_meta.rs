use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMeta {
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub modified: u64,
    pub created: u64,
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

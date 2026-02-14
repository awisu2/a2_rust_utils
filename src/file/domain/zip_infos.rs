use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZipInfo {
    pub path: String,
    pub entry_path: String,
}

use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use zip::read::ZipFile;

use crate::file::PathEx;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZipInfo {
    pub index: usize,
    pub zip_path: String,
    pub name: String,
    pub is_dir: bool,
    pub is_file: bool,
    pub size: u64,
}
impl ZipInfo {
    pub fn new(index: usize, zip_path: &str, name: &str) -> Self {
        ZipInfo {
            index,
            zip_path: zip_path.to_string_ex(),
            name: name.to_string_ex().remove_ends_separator(),
            is_dir: false,
            is_file: false,
            size: 0,
        }
    }

    pub fn set_metas(&mut self, entry: ZipFile<'_, BufReader<File>>) -> Self {
        self.is_dir = entry.is_dir();
        self.is_file = entry.is_file();
        self.size = entry.size();
        self.clone()
    }

    pub fn full_path(&self) -> String {
        format!("{}/{}", self.zip_path, self.name)
            .to_string_ex()
            .remove_ends_separator()
    }
}

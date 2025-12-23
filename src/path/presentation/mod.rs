use std::path::{Path, PathBuf};

pub fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

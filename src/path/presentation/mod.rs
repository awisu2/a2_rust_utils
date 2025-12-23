use std::{ffi::OsStr, path::Path};

// convert &Path to String.
// Pathbuf also can use this function
pub fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

// convert Option<&OsStr> to String
// this is for path.extension() and path.file_name()
pub fn osstr_opt_to_string(str: Option<&OsStr>) -> String {
    str.map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

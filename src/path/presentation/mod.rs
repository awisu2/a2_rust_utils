use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub fn osstr_to_string(osstr: &OsStr, def: &str) -> String {
    osstr
        .to_os_string()
        .to_str()
        .map(|v| String::from(v))
        .unwrap_or_else(|| String::from(def))
}

pub fn opt_osstr_to_string(opt_osstr: Option<&OsStr>, def: &str) -> String {
    let v = opt_osstr.unwrap_or_else(|| OsStr::new(""));
    osstr_to_string(v, def)
}

pub fn pathbuf_to_string(pathbuf: PathBuf, def: &str) -> String {
    osstr_to_string(pathbuf.as_os_str(), def)
}

pub fn path_to_string(path: &Path, def: &str) -> String {
    path.to_str()
        .map(|v| String::from(v))
        .unwrap_or(String::from(def))
}

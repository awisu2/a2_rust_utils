use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn osstr_to_string(osstr: &OsStr, def: &str) -> String {
    match osstr.to_os_string().to_str() {
        Some(v) => String::from(v),
        None => String::from(def),
    }
}

#[allow(dead_code)]
pub fn opt_osstr_to_string(opt_osstr: Option<&OsStr>, def: &str) -> String {
    let v = match opt_osstr {
        Some(v) => v,
        None => OsStr::new(""),
    };
    osstr_to_string(v, def)
}

#[allow(dead_code)]
pub fn pathbuf_to_string(pathbuf: PathBuf, def: &str) -> String {
    osstr_to_string(pathbuf.as_os_str(), def)
}

#[allow(dead_code)]
pub fn path_to_string(path: &Path, def: &str) -> String {
    path.to_str()
        .map(|v| String::from(v))
        .unwrap_or(String::from(def))
}

use std::path::Path;

use crate::file::PathUtil;

// convert Option<&Path> to String
// this is for path.parent()
pub fn path_opt_to_string(path: Option<&Path>) -> String {
    path.map(|p| p.to_string_ex()).unwrap_or_default()
}

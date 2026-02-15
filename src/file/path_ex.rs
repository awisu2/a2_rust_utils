pub const DIR_SEPARATOR: &str = "/";
pub const DIR_SEPARATOR_WINDOWS: &str = "\\";

pub trait PathEx {
    fn to_string_ex(&self) -> String;
    fn remove_ends_separator(&self) -> String;
}

impl<T> PathEx for T
where
    T: AsRef<std::ffi::OsStr>,
{
    fn to_string_ex(&self) -> String {
        self.as_ref()
            .to_string_lossy()
            .replace(DIR_SEPARATOR_WINDOWS, DIR_SEPARATOR)
    }

    fn remove_ends_separator(&self) -> String {
        let s = self.to_string_ex();
        s.trim_end_matches(DIR_SEPARATOR).to_string()
    }
}

pub trait OptionPathEx {
    fn to_string_ex(&self) -> String;
}

impl<T> OptionPathEx for Option<T>
where
    T: AsRef<std::ffi::OsStr>,
{
    fn to_string_ex(&self) -> String {
        match self {
            Some(v) => v.to_string_ex(),
            None => String::new(),
        }
    }
}

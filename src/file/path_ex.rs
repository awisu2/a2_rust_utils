pub const DIR_SEPARATOR: &str = "/";
pub const DIR_SEPARATOR_WINDOWS: &str = "\\";

pub trait PathEx {
    fn to_string_ex(&self) -> String;
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

pub trait MapErrorStr<T> {
    fn map_error_str(self) -> Result<T, String>;
}

impl<T> MapErrorStr<T> for anyhow::Result<T> {
    fn map_error_str(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

// test
#[cfg(test)]
mod tests {
    use crate::map::map_err_str::MapErrorStr;
    use anyhow::anyhow;

    #[test]
    fn test_map_error_str() {
        let result: anyhow::Result<i32> = Err(anyhow!("error"));
        let mapped: Result<i32, String> = result.map_error_str();
        assert_eq!(mapped, Err("error".to_string()));
    }
}

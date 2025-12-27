use anyhow::{anyhow, Result};
use std::sync::RwLock;

// Rwlock の拡張 lock時のエラーハンドリングの煩雑さを解消するためのユーティリティ
pub struct A2RwOptionLock<T> {
    inner: RwLock<Option<T>>,
}

impl<T: Clone> A2RwOptionLock<T> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(None),
        }
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn get_read_guard(&self) -> Result<std::sync::RwLockReadGuard<'_, Option<T>>> {
        let r_guard = self
            .inner
            .read()
            .map_err(|e| anyhow!("Failed to read lock {}. {}", self.type_name(), e))?;
        Ok(r_guard)
    }

    fn get_write_guard(&self) -> Result<std::sync::RwLockWriteGuard<'_, Option<T>>> {
        let w_guard = self
            .inner
            .write()
            .map_err(|e| anyhow!("Failed to write lock {}. {}", self.type_name(), e))?;
        Ok(w_guard)
    }

    pub fn set(&self, value: T) -> Result<()> {
        let mut w_guard = self.get_write_guard()?;
        *w_guard = Some(value);

        Ok(())
    }

    pub fn get(&self) -> Result<T> {
        self.get_read_guard()?
            .clone()
            .ok_or_else(|| anyhow!("{} not setted", self.type_name()))
    }

    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R> {
        let r_guard = self.get_read_guard()?;
        let value = r_guard
            .as_ref()
            .ok_or_else(|| anyhow!("{} not setted", self.type_name()))?;

        Ok(f(value))
    }
}

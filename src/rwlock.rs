// Rwlock の拡張 lock時のエラーハンドリングの煩雑さを解消するためのユーティリティ
use anyhow::{anyhow, Result};
use std::sync::RwLock;

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

    pub fn set(&self, value: T) -> Result<()> {
        let mut w_guard = self
            .inner
            .write()
            .map_err(|e| anyhow!("Failed to {} write lock. {}", self.type_name(), e))?;

        *w_guard = Some(value);
        Ok(())
    }

    pub fn get(&self) -> Result<T> {
        let r_guard = self
            .inner
            .read()
            .map_err(|e| anyhow!("Failed to {} read lock. {}", self.type_name(), e))?;

        r_guard.clone().ok_or_else(|| anyhow!("Value not set"))
    }

    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R> {
        let r_guard = self
            .inner
            .read()
            .map_err(|e| anyhow!("Failed to {} read lock. {}", self.type_name(), e))?;

        let value = r_guard.as_ref().ok_or_else(|| anyhow!("Value not set"))?;
        Ok(f(value))
    }
}

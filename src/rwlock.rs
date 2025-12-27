use anyhow::{anyhow, Result};
use std::sync::RwLock;

// Rwlock の拡張 lock時のエラーハンドリングの煩雑さを解消するためのユーティリティ
pub struct A2RwOptionLock<T> {
    inner: RwLock<Option<T>>,
}

impl<T> A2RwOptionLock<T> {
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

    pub fn get(&self) -> Result<T>
    where
        T: Clone,
    {
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

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> Result<R> {
        let mut w_guard = self.get_write_guard()?;
        let v = w_guard
            .as_mut()
            .ok_or_else(|| anyhow!("{} not setted", self.type_name()))?;
        Ok(f(v))
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    struct NoCloane {
        pub v: i32,
    }

    #[test]
    fn test_rwlock_option_lock() {
        let lock1 = A2RwOptionLock::<i32>::new();
        let lock2 = A2RwOptionLock::<NoCloane>::new();

        lock1.set(10).unwrap();
        let v = lock1.get();
        assert_eq!(v.unwrap(), 10);

        lock2.set(NoCloane { v: 20 }).unwrap();
        // NoClone cannot use get(), because T is not Clone
        // lock2.get();
        let v = lock2.with(|v| v.v);
        assert_eq!(v.unwrap(), 20);
    }
}

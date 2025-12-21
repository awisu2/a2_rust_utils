use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, Row, ToSql};
use std::{path::Path, sync::Mutex};

pub struct SqliteGateway {
    conn: Mutex<rusqlite::Connection>,
}

impl SqliteGateway {
    // impl: genericを省略する記述 本来は open<T: AsRef<Path>>(path: T) のように書く
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path.as_ref())?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn close(self) -> anyhow::Result<()> {
        let conn = self.conn.into_inner().map_err(|e| anyhow!("{}", e))?;
        conn.close().map_err(|(_, e)| anyhow!("{}", e))?;
        Ok(())
    }

    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        // lock() is sync::Mutex function, thread wait unlock if locked with other thread
        let conn = lock(&self.conn)?;
        let res = conn.execute(sql, params).map_err(|e| anyhow!("{}", e))?;
        Ok(res)
    }

    pub fn query_all<T>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
        mut map: impl FnMut(&Row) -> rusqlite::Result<T>,
    ) -> Result<Vec<T>> {
        let conn = lock(&self.conn)?;
        let mut stmt = conn.prepare(sql)?;
        let it = stmt.query_map(params, |row| map(row))?;

        let mut out = Vec::new();
        for x in it {
            out.push(x?);
        }
        Ok(out)
    }

    pub fn query_one<T>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
        map: impl FnOnce(&Row) -> rusqlite::Result<T>,
    ) -> Result<T> {
        let conn = lock(&self.conn)?;
        let res = conn.query_row(sql, params, map)?;
        Ok(res)
    }
}

// helper =====
// lock mutex and map error
fn lock<T>(m: &Mutex<T>) -> Result<std::sync::MutexGuard<'_, T>> {
    m.lock().map_err(|e| anyhow!("{}", e))
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_gateway() {
        let db_path = "test_gateway.db";

        // if exists, remove
        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).unwrap();
        }

        let gateway = SqliteGateway::open(db_path).unwrap();

        gateway
            .execute(
                "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
                &[],
            )
            .unwrap();

        gateway
            .execute("INSERT INTO users (name) VALUES (?1)", &[&"Alice"])
            .unwrap();
        gateway
            .execute("INSERT INTO users (name) VALUES (?1)", &[&"Bob"])
            .unwrap();

        let users: Vec<(i64, String)> = gateway
            .query_all("SELECT id, name FROM users ORDER BY id DESC", &[], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .unwrap();

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].1, "Bob");
        assert_eq!(users[1].1, "Alice");

        gateway
            .execute("DELETE FROM users WHERE id = ?1", &[&users[0].0])
            .unwrap();

        let remaining_users: Vec<(i64, String)> = gateway
            .query_all("SELECT id, name FROM users ORDER BY id DESC", &[], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .unwrap();

        assert_eq!(remaining_users.len(), 1);
        assert_eq!(remaining_users[0].1, "Alice");

        gateway.close().unwrap();

        std::fs::remove_file(db_path).unwrap();
    }
}

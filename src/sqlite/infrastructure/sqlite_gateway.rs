use anyhow::{anyhow, Result};
use rusqlite::{types::Value, Connection, Row, ToSql};
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

    fn _execute(&self, sql: &str, params: &[&dyn ToSql]) -> Result<usize> {
        // lock() is sync::Mutex function, thread wait unlock if locked with other thread
        let conn = Self::lock(&self.conn)?;
        let res = conn.execute(sql, params).map_err(|e| anyhow!("{}", e))?;
        Ok(res)
    }

    // &[&dyn ToSql] の lifetime管理が厳しいため、Value型受け取る
    pub fn execute(&self, sql: &str, params: &[Value]) -> Result<usize> {
        let refs = Self::map_values_to_refs(params);
        self._execute(sql, &refs)
    }

    fn _select_all<T>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
        mut map: impl FnMut(&Row) -> rusqlite::Result<T>,
    ) -> Result<Vec<T>> {
        let conn = Self::lock(&self.conn)?;
        let mut stmt = conn.prepare(sql)?;
        let it = stmt.query_map(params, |row| map(row))?;

        let mut out = Vec::new();
        for x in it {
            out.push(x?);
        }
        Ok(out)
    }

    // &[&dyn ToSql] の lifetime管理が厳しいため、Value型受け取る
    pub fn select_all<T>(
        &self,
        sql: &str,
        params: &[Value],
        map: impl FnMut(&Row) -> rusqlite::Result<T>,
    ) -> Result<Vec<T>> {
        let refs = Self::map_values_to_refs(params);
        self._select_all(sql, &refs, map)
    }

    fn _select_one<T>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
        map: impl FnOnce(&Row) -> rusqlite::Result<T>,
    ) -> Result<T> {
        let conn = Self::lock(&self.conn)?;
        let res = conn.query_row(sql, params, map)?;
        Ok(res)
    }

    // &[&dyn ToSql] の lifetime管理が厳しいため、Value型受け取る
    pub fn select_one<T>(
        &self,
        sql: &str,
        params: &[Value],
        map: impl FnOnce(&Row) -> rusqlite::Result<T>,
    ) -> Result<T> {
        let refs = Self::map_values_to_refs(params);
        self._select_one(sql, &refs, map)
    }

    // helpers =====
    // &[Value] を &[&dyn ToSql] に変換
    fn map_values_to_refs<'a>(values: &'a [Value]) -> Vec<&'a dyn ToSql> {
        values.iter().map(|v| v as &dyn ToSql).collect()
    }

    // lock mutex and map error
    fn lock<T>(m: &Mutex<T>) -> Result<std::sync::MutexGuard<'_, T>> {
        m.lock().map_err(|e| anyhow!("{}", e))
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sqlite::application::sample_data;

    #[test]
    fn test_sqlite_gateway() {
        let db_path = "test_gateway.db";

        // if exists, remove
        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).unwrap();
        }

        let gateway = SqliteGateway::open(db_path).unwrap();

        let (sql, params) = sample_data::SampleData::create_table_sql();
        gateway.execute(&sql, &params).unwrap();

        let (sql, params) = sample_data::SampleData::insert("Alice", 18);
        gateway.execute(&sql, &params).unwrap();
        let (sql, params) = sample_data::SampleData::insert("Bob", 30);
        gateway.execute(&sql, &params).unwrap();

        let (sql, params) = sample_data::SampleData::select_all();
        let records: Vec<sample_data::SampleData> = gateway
            .select_all(&sql, &params, sample_data::SampleData::map_from_row)
            .unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].name, "Alice");
        assert_eq!(records[1].name, "Bob");

        let (sql, params) = sample_data::SampleData::delete_by_id(records[0].id);
        gateway.execute(&sql, &params).unwrap();

        let (sql, params) = sample_data::SampleData::select_all();
        let records: Vec<sample_data::SampleData> = gateway
            .select_all(&sql, &params, sample_data::SampleData::map_from_row)
            .unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "Bob");

        gateway.close().unwrap();

        std::fs::remove_file(db_path).unwrap();
    }
}

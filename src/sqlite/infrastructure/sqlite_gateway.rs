use anyhow::{anyhow, Ok, Result};
use rusqlite::{types::Value, Connection, Row, ToSql};
use std::{path::Path, sync::Mutex};

pub struct SqliteGateway {
    // 同時更新ができないので Mutex で保護
    // pathの確定タイミングが不明なため Option
    pub conn: Mutex<Option<Connection>>,
    pub path: Option<String>,
}

impl Default for SqliteGateway {
    fn default() -> Self {
        SqliteGateway {
            conn: Mutex::new(None),
            path: None,
        }
    }
}

impl SqliteGateway {
    // impl: genericを省略する記述 本来は open<T: AsRef<Path>>(path: T) のように書く
    pub fn open(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let conn = Connection::open(path.as_ref())?;
        self.conn = Mutex::new(Some(conn));
        self.path = Some(path.as_ref().to_string_lossy().to_string());

        Ok(())
    }

    pub fn close(&mut self) -> anyhow::Result<()> {
        let conn = self.take_conn()?;
        conn.close().map_err(|(_, e)| anyhow!("{}", e))?;

        self.conn = Mutex::new(None);
        self.path = None;

        Ok(())
    }

    // &[&dyn ToSql] の lifetime管理が厳しいため、Value型受け取る
    pub fn execute(&self, sql: &str, params: &[Value]) -> Result<usize> {
        let refs = Self::map_values_to_refs(params);
        self.with_conn(|conn| Ok(conn.execute(sql, refs.as_slice())?))
    }

    // &[&dyn ToSql] の lifetime管理が厳しいため、Value型受け取る
    pub fn select_all<T>(
        &self,
        sql: &str,
        params: &[Value],
        mut map: impl FnMut(&Row) -> rusqlite::Result<T>,
    ) -> Result<Vec<T>> {
        let refs = Self::map_values_to_refs(params);
        // self._select_all(sql, &refs, map)

        self.with_conn(|conn| {
            let mut stmt = conn.prepare(sql)?;
            let it = stmt.query_map(refs.as_slice(), |row| map(row))?;

            let mut out = Vec::new();
            for x in it {
                out.push(x?);
            }
            Ok(out)
        })
    }

    // &[&dyn ToSql] の lifetime管理が厳しいため、Value型受け取る
    pub fn select_one<T>(
        &self,
        sql: &str,
        params: &[Value],
        map: impl FnOnce(&Row) -> rusqlite::Result<T>,
    ) -> Result<T> {
        let refs = Self::map_values_to_refs(params);
        self.with_conn(|conn| {
            let res = conn.query_row(sql, refs.as_slice(), map)?;
            Ok(res)
        })
    }

    // helpers =====
    // &[Value] を &[&dyn ToSql] に変換
    fn map_values_to_refs<'a>(values: &'a [Value]) -> Vec<&'a dyn ToSql> {
        values.iter().map(|v| v as &dyn ToSql).collect()
    }

    fn with_conn<R>(&self, f: impl FnOnce(&mut Connection) -> Result<R>) -> Result<R> {
        let mut guard = self.conn.lock().map_err(|e| anyhow!("{}", e))?;
        let conn = guard
            .as_mut()
            .ok_or_else(|| anyhow!("Connection is already closed"))?;
        f(conn)
    }

    fn take_conn(&self) -> Result<Connection> {
        let mut guard = self.conn.lock().map_err(|e| anyhow!("{}", e))?;
        guard
            .take()
            .ok_or_else(|| anyhow!("Connection is already closed"))
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sqlite::SampleData;

    #[test]
    fn test_sqlite_gateway() {
        let db_path = "test_gateway.db";

        // if exists, remove
        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).unwrap();
        }

        let mut gateway = SqliteGateway::default();

        gateway.open(db_path).unwrap();

        let (sql, params) = SampleData::create_table_sql();
        gateway.execute(&sql, &params).unwrap();

        let (sql, params) = SampleData::insert("Alice", 18);
        gateway.execute(&sql, &params).unwrap();
        let (sql, params) = SampleData::insert("Bob", 30);
        gateway.execute(&sql, &params).unwrap();

        let (sql, params) = SampleData::select_all();
        let records: Vec<SampleData> = gateway
            .select_all(&sql, &params, SampleData::map_from_row)
            .unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].name, "Alice");
        assert_eq!(records[1].name, "Bob");

        let (sql, params) = SampleData::delete_by_id(records[0].id);
        gateway.execute(&sql, &params).unwrap();

        let (sql, params) = SampleData::select_all();
        let records: Vec<SampleData> = gateway
            .select_all(&sql, &params, SampleData::map_from_row)
            .unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "Bob");

        gateway.close().unwrap();

        std::fs::remove_file(db_path).unwrap();
    }
}

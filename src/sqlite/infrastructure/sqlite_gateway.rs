use anyhow::{anyhow, Result};
use rusqlite::{types::Value, Connection, Row, ToSql};
use std::sync::Mutex;

pub struct SqliteGateway {
    // 同時更新ができないので Mutex で保護
    // pathの確定タイミングが不明なため Option
    pub conn: Mutex<Option<Connection>>,
    path: String,
}

impl SqliteGateway {
    pub fn new(path: &str) -> Self {
        SqliteGateway {
            conn: Mutex::new(None),
            path: path.to_string(),
        }
    }

    // impl: genericを省略する記述 本来は open<T: AsRef<Path>>(path: T) のように書く
    pub fn open(&self) -> Result<()> {
        self.close()?;

        let path = self.path.clone();
        let new_conn = Connection::open(path)?;

        // borrow: MutexGuard を取得
        let mut guard = self.conn.lock().map_err(|e| anyhow!("{}", e))?;
        guard.replace(new_conn);

        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        let mut guard = self.conn.lock().map_err(|e| anyhow!("{}", e))?;
        // take: Optionの中身を取り出してNoneにする。所有権も移動する。
        let conn = match guard.take() {
            None => return Ok(()),
            Some(conn) => conn,
        };

        conn.close()
            .map_err(|_| anyhow!("Failed to close connection"))?;

        Ok(())
    }

    // &[&dyn ToSql] の lifetime管理が厳しいため、Value型受け取る
    pub fn execute(&self, sql: &str, params: &[Value]) -> Result<usize> {
        let refs = Self::map_values_to_refs(params);
        self.with_conn(|conn| {
            let res = conn.execute(sql, refs.as_slice())?;
            Ok(res)
        })
    }

    // execute insert and return latest inserted rowid
    pub fn insert_row_id(&self, sql: &str, params: &[Value]) -> Result<i64> {
        let refs = Self::map_values_to_refs(params);
        self.with_conn(|conn| {
            conn.execute(sql, refs.as_slice())?;
            let id = conn.last_insert_rowid();
            Ok(id)
        })
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
            // prepare: SQL文をコンパイルしてステートメントを作成
            let mut stmt = conn.prepare(sql)?;

            // sqlを実行し、結果をカスタム処理(map変換)しつつイテレータで取得
            let it = stmt.query_map(refs.as_slice(), |row| map(row))?;

            // Vecに変換して返す
            let mut out = Vec::<T>::new();
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
    ) -> Result<Option<T>> {
        let refs = Self::map_values_to_refs(params);
        self.with_conn(|conn| match conn.query_row(sql, refs.as_slice(), map) {
            Ok(record) => Ok(Some(record)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!(e)),
        })
    }

    // helpers =====
    // &[Value] を &[&dyn ToSql] に変換
    fn map_values_to_refs<'a>(values: &'a [Value]) -> Vec<&'a dyn ToSql> {
        values.iter().map(|v| v as &dyn ToSql).collect()
    }

    fn with_conn<R>(&self, f: impl FnOnce(&Connection) -> Result<R>) -> Result<R> {
        let guard = self.conn.lock().map_err(|e| anyhow!("{}", e))?;
        let conn = match guard.as_ref() {
            None => return Err(anyhow!("Connection is already closed")),
            Some(c) => c,
        };

        f(conn)
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

        let gateway = SqliteGateway::new(db_path);
        let _ = gateway.open();

        let (sql, params) = SampleData::create_table_sql();
        gateway.execute(&sql, &params).unwrap();

        let (sql, params) = SampleData::insert("Alice", 18);
        gateway.execute(&sql, &params).unwrap();
        let (sql, params) = SampleData::insert("Bob", 30);
        let id = gateway.insert_row_id(&sql, &params).unwrap();

        let (sql, params) = SampleData::select_by_id(id);
        let row = gateway
            .select_one(&sql, &params, SampleData::map_from_row)
            .unwrap()
            .unwrap();
        assert_eq!("Bob", row.name);

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

use once_cell::sync::Lazy;
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SampleData {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

const TABLE: &'static str = "SampleData";
const COL_ID: &'static str = "id";
const COL_NAME: &'static str = "name";
const COL_AGE: &'static str = "age";

impl SampleData {
    pub fn map_from_row(row: &rusqlite::Row) -> rusqlite::Result<SampleData> {
        Ok(SampleData {
            id: row.get(COL_ID)?,
            name: row.get(COL_NAME)?,
            age: row.get(COL_AGE)?,
        })
    }

    pub fn create_table_sql() -> (&'static str, [Value; 0]) {
        (&CREATE_TABLE_SQL, [])
    }

    pub fn insert(name: &str, age: i32) -> (&'static str, [Value; 2]) {
        (
            &INSERT_SQL,
            [Value::Text(name.to_string()), Value::Integer(age as i64)],
        )
    }

    pub fn select_all() -> (&'static str, [Value; 0]) {
        (&SELECT_ALL_SQL, [])
    }

    pub fn select_by_id(id: i32) -> (&'static str, [Value; 1]) {
        (&SELECT_BY_ID_SQL, [Value::Integer(id as i64)])
    }

    pub fn delete_by_id(id: i32) -> (&'static str, [Value; 1]) {
        (&DELETE_BY_ID_SQL, [Value::Integer(id as i64)])
    }

    pub fn delete_all() -> (&'static str, [Value; 0]) {
        (&DELETE_ALL_SQL, [])
    }
}

pub static CREATE_TABLE_SQL: Lazy<String> = Lazy::new(|| {
    format!(
        "CREATE TABLE IF NOT EXISTS {table} (
            {id} INTEGER PRIMARY KEY,
            {name} TEXT NOT NULL,
            {age} INTEGER NOT NULL
        );",
        table = TABLE,
        id = COL_ID,
        name = COL_NAME,
        age = COL_AGE,
    )
});

pub static INSERT_SQL: Lazy<String> = Lazy::new(|| {
    format!(
        "INSERT INTO {table} ({name}, {age}) VALUES (?1, ?2);",
        table = TABLE,
        name = COL_NAME,
        age = COL_AGE,
    )
});

pub static SELECT_ALL_SQL: Lazy<String> = Lazy::new(|| {
    format!(
        "SELECT {id}, {name}, {age} FROM {table};",
        table = TABLE,
        id = COL_ID,
        name = COL_NAME,
        age = COL_AGE,
    )
});

pub static SELECT_BY_ID_SQL: Lazy<String> = Lazy::new(|| {
    format!(
        "SELECT {id}, {name}, {age} FROM {table} WHERE {id} = ?1;",
        table = TABLE,
        id = COL_ID,
        name = COL_NAME,
        age = COL_AGE,
    )
});

pub static DELETE_BY_ID_SQL: Lazy<String> = Lazy::new(|| {
    format!(
        "DELETE FROM {table} WHERE {id} = ?1;",
        table = TABLE,
        id = COL_ID,
    )
});

pub static DELETE_ALL_SQL: Lazy<String> =
    Lazy::new(|| format!("DELETE FROM {table};", table = TABLE,));

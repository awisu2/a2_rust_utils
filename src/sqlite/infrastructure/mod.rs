use anyhow::Result;
use rusqlite::{params, Connection};

fn sample(path: &str) -> Result<()> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        [],
    )?;

    conn.execute("INSERT INTO users (name) VALUES (?1)", params!["Alice"])?;
    conn.execute("INSERT INTO users (name) VALUES (?1)", params!["Bob"])?;

    let mut stmt = conn.prepare("SELECT id, name FROM users ORDER BY id DESC")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;

    for row in rows {
        let (id, name) = row?;
        println!("Found user: {} with id {}", name, id);
    }

    conn.execute("DELETE FROM users WHERE id = ?1", params![1])?;

    Ok(())
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let db_path = "test_sample.db";
        let result = sample(db_path);
        assert!(result.is_ok());
        std::fs::remove_file(db_path).unwrap();
    }
}

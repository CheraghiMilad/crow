use rusqlite::Connection;
use anyhow::{Result, Context}; // Import anyhow Result

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)
            .context("Failed to open database connection")?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                hash TEXT PRIMARY KEY,
                path TEXT NOT NULL
            )",
            [],
        ).context("Failed to create database table")?;
        
        Ok(Database { conn })
    }
    
    pub fn get_file_path(&self, hash: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT path FROM files WHERE hash = ?1")
            .context("Failed to prepare query")?;
        
        let mut rows = stmt.query([hash]).context("Failed to execute query")?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
    
    pub fn save_file(&self, hash: &str, path: &str) -> Result<()> {
        self.conn.execute("INSERT INTO files (hash, path) VALUES (?1, ?2)", [hash, path])
            .context("Failed to insert file into database")?;
        
        Ok(())
    }
}

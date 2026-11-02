use std::path::Path;
use anyhow::Result;
use rusqlite::{params, Connection};

use crate::models::{FolderItem, ImageRecipe, MediaItem};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;

        // Picasa-style high performance SQLite tuning
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -64000; -- 64MB cache
             PRAGMA foreign_keys = ON;",
        )?;

        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS photos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                filename TEXT NOT NULL,
                parent_dir TEXT NOT NULL,
                size_bytes INTEGER NOT NULL,
                modified_ms INTEGER NOT NULL,
                width INTEGER DEFAULT 0,
                height INTEGER DEFAULT 0,
                mime_type TEXT NOT NULL,
                is_starred INTEGER DEFAULT 0,
                rating INTEGER DEFAULT 0,
                date_taken TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_photos_parent_dir ON photos(parent_dir);
            CREATE INDEX IF NOT EXISTS idx_photos_date_taken ON photos(date_taken);
            CREATE INDEX IF NOT EXISTS idx_photos_starred ON photos(is_starred);

            CREATE TABLE IF NOT EXISTS watched_folders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                enabled INTEGER DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS recipes (
                photo_path TEXT PRIMARY KEY,
                crop_json TEXT,
                rotation INTEGER DEFAULT 0,
                brightness REAL DEFAULT 0.0,
                contrast REAL DEFAULT 1.0,
                saturation REAL DEFAULT 1.0,
                is_starred INTEGER DEFAULT 0,
                updated_at INTEGER DEFAULT (strftime('%s', 'now'))
            );",
        )?;
        Ok(())
    }

    pub fn upsert_photo(&self, item: &MediaItem) -> Result<()> {
        self.conn.execute(
            "INSERT INTO photos (path, filename, parent_dir, size_bytes, modified_ms, width, height, mime_type, is_starred, rating, date_taken)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
             ON CONFLICT(path) DO UPDATE SET
                size_bytes = excluded.size_bytes,
                modified_ms = excluded.modified_ms,
                width = excluded.width,
                height = excluded.height;",
            params![
                item.path,
                item.filename,
                item.parent_dir,
                item.size_bytes,
                item.modified_ms,
                item.width,
                item.height,
                item.mime_type,
                item.is_starred as i32,
                item.rating,
                item.date_taken
            ],
        )?;
        Ok(())
    }

    pub fn toggle_star(&self, path: &str) -> Result<bool> {
        let mut stmt = self.conn.prepare("SELECT is_starred FROM photos WHERE path = ?1")?;
        let current: Option<i32> = stmt.query_row(params![path], |row| row.get(0)).ok();
        let new_state = match current {
            Some(1) => 0,
            _ => 1,
        };

        self.conn.execute(
            "UPDATE photos SET is_starred = ?1 WHERE path = ?2",
            params![new_state, path],
        )?;

        Ok(new_state == 1)
    }
}

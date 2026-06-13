use crate::platform::storage::{HistoryItem, StorageError, StoragePlatform};
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct SqliteStorage {
    conn: Mutex<Connection>,
}

impl SqliteStorage {
    pub fn new(db_path: Option<PathBuf>) -> Result<Self, StorageError> {
        let conn = if let Some(path) = db_path {
            Connection::open(path).map_err(|e| StorageError::InitError(e.to_string()))?
        } else {
            // Use default path: ~/.maccy/maccy.db
            let home = dirs::home_dir().ok_or_else(|| StorageError::InitError("Cannot find home directory".to_string()))?;
            let db_dir = home.join(".maccy");
            std::fs::create_dir_all(&db_dir).map_err(|e| StorageError::InitError(e.to_string()))?;
            let db_path = db_dir.join("maccy.db");
            Connection::open(db_path).map_err(|e| StorageError::InitError(e.to_string()))?
        };

        let storage = Self {
            conn: Mutex::new(conn),
        };

        storage.initialize()?;
        Ok(storage)
    }

    fn create_tables(&self) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS history_items (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content_type TEXT NOT NULL,
                content_data BLOB,
                application TEXT,
                first_copied_at INTEGER NOT NULL,
                last_copied_at INTEGER NOT NULL,
                number_of_copies INTEGER NOT NULL DEFAULT 1,
                pin TEXT
            )",
            [],
        ).map_err(|e| StorageError::InitError(e.to_string()))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_last_copied_at ON history_items(last_copied_at DESC)",
            [],
        ).map_err(|e| StorageError::InitError(e.to_string()))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_pin ON history_items(pin)",
            [],
        ).map_err(|e| StorageError::InitError(e.to_string()))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_title ON history_items(title)",
            [],
        ).map_err(|e| StorageError::InitError(e.to_string()))?;

        Ok(())
    }
}

impl StoragePlatform for SqliteStorage {
    fn initialize(&self) -> Result<(), StorageError> {
        self.create_tables()
    }

    fn insert_item(&self, item: &HistoryItem) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        conn.execute(
            "INSERT INTO history_items (
                id, title, content_type, content_data, application,
                first_copied_at, last_copied_at, number_of_copies, pin
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                item.id,
                item.title,
                item.content_type,
                item.content_data,
                item.application,
                item.first_copied_at,
                item.last_copied_at,
                item.number_of_copies,
                item.pin,
            ],
        ).map_err(|e| StorageError::InsertError(e.to_string()))?;

        Ok(())
    }

    fn get_all_items(&self) -> Result<Vec<HistoryItem>, StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, title, content_type, content_data, application,
                    first_copied_at, last_copied_at, number_of_copies, pin
             FROM history_items
             ORDER BY last_copied_at DESC"
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;

        let items = stmt.query_map([], |row| {
            Ok(HistoryItem {
                id: row.get(0)?,
                title: row.get(1)?,
                content_type: row.get(2)?,
                content_data: row.get(3)?,
                application: row.get(4)?,
                first_copied_at: row.get(5)?,
                last_copied_at: row.get(6)?,
                number_of_copies: row.get(7)?,
                pin: row.get(8)?,
            })
        }).map_err(|e| StorageError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(items)
    }

    fn search_items(&self, query: &str) -> Result<Vec<HistoryItem>, StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        let search_pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, title, content_type, content_data, application,
                    first_copied_at, last_copied_at, number_of_copies, pin
             FROM history_items
             WHERE title LIKE ?1
             ORDER BY last_copied_at DESC
             LIMIT 100"
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;

        let items = stmt.query_map(params![search_pattern], |row| {
            Ok(HistoryItem {
                id: row.get(0)?,
                title: row.get(1)?,
                content_type: row.get(2)?,
                content_data: row.get(3)?,
                application: row.get(4)?,
                first_copied_at: row.get(5)?,
                last_copied_at: row.get(6)?,
                number_of_copies: row.get(7)?,
                pin: row.get(8)?,
            })
        }).map_err(|e| StorageError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(items)
    }

    fn delete_item(&self, id: &str) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        conn.execute(
            "DELETE FROM history_items WHERE id = ?1",
            params![id],
        ).map_err(|e| StorageError::DeleteError(e.to_string()))?;

        Ok(())
    }

    fn clear_all(&self) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        conn.execute("DELETE FROM history_items", [])
            .map_err(|e| StorageError::DeleteError(e.to_string()))?;

        Ok(())
    }

    fn update_pin(&self, id: &str, pin: Option<&str>) -> Result<(), StorageError> {
        let conn = self.conn.lock().map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        conn.execute(
            "UPDATE history_items SET pin = ?1 WHERE id = ?2",
            params![pin, id],
        ).map_err(|e| StorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_storage_initialization() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = SqliteStorage::new(Some(db_path));
        assert!(storage.is_ok(), "Storage should initialize successfully");
    }

    #[test]
    fn test_insert_and_retrieve_item() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = SqliteStorage::new(Some(db_path)).unwrap();

        let item = HistoryItem {
            id: "test-id-1".to_string(),
            title: "Test content".to_string(),
            content_type: "text/plain".to_string(),
            content_data: b"Test content".to_vec(),
            application: Some("TestApp".to_string()),
            first_copied_at: 1000,
            last_copied_at: 1000,
            number_of_copies: 1,
            pin: None,
        };

        let result = storage.insert_item(&item);
        assert!(result.is_ok(), "Should insert item successfully");

        let items = storage.get_all_items().unwrap();
        assert_eq!(items.len(), 1, "Should have 1 item");
        assert_eq!(items[0].id, "test-id-1", "Item ID should match");
        assert_eq!(items[0].title, "Test content", "Item title should match");
    }

    #[test]
    fn test_search_items() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = SqliteStorage::new(Some(db_path)).unwrap();

        for i in 0..5 {
            let item = HistoryItem {
                id: format!("test-id-{}", i),
                title: format!("Content {}", i),
                content_type: "text/plain".to_string(),
                content_data: format!("Content {}", i).into_bytes(),
                application: None,
                first_copied_at: 1000 + i,
                last_copied_at: 1000 + i,
                number_of_copies: 1,
                pin: None,
            };
            storage.insert_item(&item).unwrap();
        }

        let results = storage.search_items("Content 3").unwrap();
        assert_eq!(results.len(), 1, "Should find 1 matching item");
        assert_eq!(results[0].title, "Content 3", "Should find correct item");
    }

    #[test]
    fn test_delete_item() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = SqliteStorage::new(Some(db_path)).unwrap();

        let item = HistoryItem {
            id: "test-id-delete".to_string(),
            title: "To be deleted".to_string(),
            content_type: "text/plain".to_string(),
            content_data: b"To be deleted".to_vec(),
            application: None,
            first_copied_at: 1000,
            last_copied_at: 1000,
            number_of_copies: 1,
            pin: None,
        };

        storage.insert_item(&item).unwrap();
        assert_eq!(storage.get_all_items().unwrap().len(), 1, "Should have 1 item");

        let result = storage.delete_item("test-id-delete");
        assert!(result.is_ok(), "Should delete successfully");

        assert_eq!(storage.get_all_items().unwrap().len(), 0, "Should have 0 items");
    }

    #[test]
    fn test_clear_all() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = SqliteStorage::new(Some(db_path)).unwrap();

        for i in 0..3 {
            let item = HistoryItem {
                id: format!("test-id-{}", i),
                title: format!("Content {}", i),
                content_type: "text/plain".to_string(),
                content_data: format!("Content {}", i).into_bytes(),
                application: None,
                first_copied_at: 1000,
                last_copied_at: 1000,
                number_of_copies: 1,
                pin: None,
            };
            storage.insert_item(&item).unwrap();
        }

        assert_eq!(storage.get_all_items().unwrap().len(), 3, "Should have 3 items");

        let result = storage.clear_all();
        assert!(result.is_ok(), "Should clear successfully");

        assert_eq!(storage.get_all_items().unwrap().len(), 0, "Should have 0 items");
    }

    #[test]
    fn test_update_pin() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = SqliteStorage::new(Some(db_path)).unwrap();

        let item = HistoryItem {
            id: "test-pin".to_string(),
            title: "Pin test".to_string(),
            content_type: "text/plain".to_string(),
            content_data: b"Pin test".to_vec(),
            application: None,
            first_copied_at: 1000,
            last_copied_at: 1000,
            number_of_copies: 1,
            pin: None,
        };

        storage.insert_item(&item).unwrap();

        let result = storage.update_pin("test-pin", Some("pin_abc123"));
        assert!(result.is_ok(), "Should pin successfully");

        let items = storage.get_all_items().unwrap();
        assert_eq!(items[0].pin, Some("pin_abc123".to_string()), "Item should be pinned");

        let result = storage.update_pin("test-pin", None);
        assert!(result.is_ok(), "Should unpin successfully");

        let items = storage.get_all_items().unwrap();
        assert_eq!(items[0].pin, None, "Item should be unpinned");
    }

    #[test]
    fn test_order_by_last_copied_at() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = SqliteStorage::new(Some(db_path)).unwrap();

        for i in 0..3 {
            let item = HistoryItem {
                id: format!("test-id-{}", i),
                title: format!("Content {}", i),
                content_type: "text/plain".to_string(),
                content_data: format!("Content {}", i).into_bytes(),
                application: None,
                first_copied_at: 1000,
                last_copied_at: 1000 + i * 100,
                number_of_copies: 1,
                pin: None,
            };
            storage.insert_item(&item).unwrap();
        }

        let items = storage.get_all_items().unwrap();

        assert_eq!(items[0].last_copied_at, 1200, "Newest item should be first");
        assert_eq!(items[1].last_copied_at, 1100, "Second newest should be second");
        assert_eq!(items[2].last_copied_at, 1000, "Oldest should be last");
    }
}

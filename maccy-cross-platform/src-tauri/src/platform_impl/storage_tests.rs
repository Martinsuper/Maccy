#[cfg(test)]
mod tests {
    use crate::platform::storage::{HistoryItem, StoragePlatform};
    use crate::platform_impl::storage::SqliteStorage;
    use std::path::PathBuf;
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

        // Insert item
        let result = storage.insert_item(&item);
        assert!(result.is_ok(), "Should insert item successfully");

        // Retrieve items
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

        // Insert multiple items
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

        // Search for specific content
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

        // Delete item
        let result = storage.delete_item("test-id-delete");
        assert!(result.is_ok(), "Should delete successfully");

        assert_eq!(storage.get_all_items().unwrap().len(), 0, "Should have 0 items");
    }

    #[test]
    fn test_clear_all() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let storage = SqliteStorage::new(Some(db_path)).unwrap();

        // Insert multiple items
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

        // Clear all
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

        // Pin item
        let result = storage.update_pin("test-pin", Some("pin_abc123"));
        assert!(result.is_ok(), "Should pin successfully");

        let items = storage.get_all_items().unwrap();
        assert_eq!(items[0].pin, Some("pin_abc123".to_string()), "Item should be pinned");

        // Unpin item
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

        // Insert items with different timestamps
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

        // Should be ordered by last_copied_at DESC (newest first)
        assert_eq!(items[0].last_copied_at, 1200, "Newest item should be first");
        assert_eq!(items[1].last_copied_at, 1100, "Second newest should be second");
        assert_eq!(items[2].last_copied_at, 1000, "Oldest should be last");
    }
}

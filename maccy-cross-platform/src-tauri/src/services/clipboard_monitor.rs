use crate::platform::clipboard::{ClipboardContent, ClipboardError};
use crate::platform::storage::StoragePlatform;
use crate::platform_impl::storage::SqliteStorage;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct ClipboardMonitor {
    monitoring: Arc<AtomicBool>,
    storage: Arc<SqliteStorage>,
    check_interval_ms: u64,
}

impl ClipboardMonitor {
    pub fn new(storage: Arc<SqliteStorage>, check_interval_ms: u64) -> Self {
        Self {
            monitoring: Arc::new(AtomicBool::new(false)),
            storage,
            check_interval_ms,
        }
    }

    pub fn start(&self) -> Result<(), ClipboardError> {
        if self.monitoring.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.monitoring.store(true, Ordering::SeqCst);
        let monitoring = Arc::clone(&self.monitoring);
        let storage = Arc::clone(&self.storage);
        let interval = self.check_interval_ms;

        thread::spawn(move || {
            let mut last_known_count = 0u64;

            while monitoring.load(Ordering::SeqCst) {
                #[cfg(target_os = "macos")]
                {
                    use crate::platform_impl::macos::MacClipboard;
                    use crate::platform::clipboard::ClipboardPlatform;

                    let clipboard = MacClipboard::new();
                    if let Ok(current_count) = clipboard.get_change_count() {
                        if current_count != last_known_count {
                            last_known_count = current_count;

                            if let Ok(content) = clipboard.read_content() {
                                if let ClipboardContent::Text(text) = content {
                                    if !text.trim().is_empty() {
                                        Self::save_to_storage(&storage, &text);
                                    }
                                }
                            }
                        }
                    }
                }

                thread::sleep(Duration::from_millis(interval));
            }
        });

        Ok(())
    }

    pub fn stop(&self) -> Result<(), ClipboardError> {
        self.monitoring.store(false, Ordering::SeqCst);
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn save_to_storage(storage: &Arc<SqliteStorage>, text: &str) {
        // Check if the same content already exists (deduplication)
        if let Ok(existing_items) = storage.get_all_items() {
            for item in existing_items {
                if item.title == text {
                    // Update last_copied_at and increment number_of_copies
                    let now = chrono::Utc::now().timestamp();
                    let updated_item = crate::platform::storage::HistoryItem {
                        id: item.id,
                        title: item.title,
                        content_type: item.content_type,
                        content_data: item.content_data,
                        application: item.application,
                        first_copied_at: item.first_copied_at,
                        last_copied_at: now,
                        number_of_copies: item.number_of_copies + 1,
                        pin: item.pin,
                    };

                    // Delete old and insert updated
                    let _ = storage.delete_item(&updated_item.id);
                    let _ = storage.insert_item(&updated_item);
                    return;
                }
            }
        }

        // Create new history item
        let now = chrono::Utc::now().timestamp();
        let item = crate::platform::storage::HistoryItem {
            id: uuid::Uuid::new_v4().to_string(),
            title: text.to_string(),
            content_type: "text/plain".to_string(),
            content_data: text.as_bytes().to_vec(),
            application: None, // TODO: Get frontmost app
            first_copied_at: now,
            last_copied_at: now,
            number_of_copies: 1,
            pin: None,
        };

        if let Err(e) = storage.insert_item(&item) {
            log::error!("Failed to save clipboard content: {}", e);
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn save_to_storage(_storage: &Arc<SqliteStorage>, _text: &str) {
        log::warn!("Clipboard monitoring not implemented for this platform");
    }
}

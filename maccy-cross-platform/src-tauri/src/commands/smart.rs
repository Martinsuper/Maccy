use crate::platform::storage::StoragePlatform;
use crate::platform_impl::storage::SqliteStorage;
use std::sync::Arc;
use tauri::State;

/// PasteStack: Batch paste multiple items sequentially
#[tauri::command]
pub async fn paste_stack(
    storage: State<'_, Arc<SqliteStorage>>,
    item_ids: Vec<String>,
    delay_ms: u64,
) -> Result<u32, String> {
    use crate::platform::storage::StoragePlatform;
    use crate::services::paste_simulator::PasteSimulator;

    let items = storage.get_all_items().map_err(|e| e.to_string())?;
    let mut success_count = 0;

    for id in item_ids {
        if let Some(item) = items.iter().find(|i| i.id == id) {
            // Set clipboard content
            #[cfg(target_os = "macos")]
            {
                use crate::platform::clipboard::{ClipboardContent, ClipboardPlatform};
                use crate::platform_impl::macos::MacClipboard;

                let clipboard = MacClipboard::new();
                if clipboard.write_content(&ClipboardContent::Text(item.title.clone())).is_ok() {
                    // Wait for clipboard to update
                    std::thread::sleep(std::time::Duration::from_millis(100));

                    // Simulate paste
                    let simulator = PasteSimulator::new();
                    if simulator.paste().is_ok() {
                        success_count += 1;

                        // Wait before next paste
                        if delay_ms > 0 {
                            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
                        }
                    }
                }
            }
        }
    }

    Ok(success_count)
}

/// Auto-cleanup: Remove old items based on settings
#[tauri::command]
pub async fn auto_cleanup(
    storage: State<'_, Arc<SqliteStorage>>,
    max_age_days: u32,
    keep_pinned: bool,
) -> Result<u32, String> {
    use crate::platform::storage::StoragePlatform;

    let items = storage.get_all_items().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp();
    let max_age_seconds = (max_age_days as i64) * 24 * 60 * 60;
    let mut deleted_count = 0;

    for item in items {
        // Skip pinned items if configured
        if keep_pinned && item.pin.is_some() {
            continue;
        }

        // Check if item is older than max_age
        if now - item.last_copied_at > max_age_seconds {
            if storage.delete_item(&item.id).is_ok() {
                deleted_count += 1;
            }
        }
    }

    Ok(deleted_count)
}

/// Smart search: Search with fuzzy matching and ranking
#[tauri::command]
pub async fn smart_search(
    storage: State<'_, Arc<SqliteStorage>>,
    query: String,
    limit: usize,
) -> Result<Vec<crate::commands::history::HistoryItemDto>, String> {
    use crate::platform::storage::StoragePlatform;

    let items = storage.get_all_items().map_err(|e| e.to_string())?;
    let query_lower = query.to_lowercase();

    // Score and filter items
    let mut scored_items: Vec<_> = items
        .into_iter()
        .filter_map(|item| {
            let title_lower = item.title.to_lowercase();

            // Exact match gets highest score
            if title_lower.contains(&query_lower) {
                let score = if title_lower == query_lower {
                    1000
                } else if title_lower.starts_with(&query_lower) {
                    500
                } else {
                    100
                };

                // Boost pinned items
                let pin_boost = if item.pin.is_some() { 200 } else { 0 };

                // Boost recent items
                let now = chrono::Utc::now().timestamp();
                let age_hours = (now - item.last_copied_at) / 3600;
                let recency_boost = if age_hours < 1 {
                    100
                } else if age_hours < 24 {
                    50
                } else {
                    0
                };

                Some((item, score + pin_boost + recency_boost))
            } else {
                None
            }
        })
        .collect();

    // Sort by score descending
    scored_items.sort_by(|a, b| b.1.cmp(&a.1));

    // Take top N items
    let result: Vec<_> = scored_items
        .into_iter()
        .take(limit)
        .map(|(item, _)| item.into())
        .collect();

    Ok(result)
}

/// Get usage statistics
#[tauri::command]
pub async fn get_statistics(
    storage: State<'_, Arc<SqliteStorage>>,
) -> Result<serde_json::Value, String> {
    use crate::platform::storage::StoragePlatform;

    let items = storage.get_all_items().map_err(|e| e.to_string())?;

    let total_items = items.len();
    let pinned_items = items.iter().filter(|i| i.pin.is_some()).count();

    // Calculate total copies
    let total_copies: i32 = items.iter().map(|i| i.number_of_copies).sum();

    // Calculate average copies per item
    let avg_copies = if total_items > 0 {
        total_copies as f64 / total_items as f64
    } else {
        0.0
    };

    // Find most copied item
    let most_copied = items
        .iter()
        .max_by_key(|i| i.number_of_copies)
        .map(|i| i.title.clone());

    // Calculate storage size (approximate)
    let storage_bytes: usize = items
        .iter()
        .map(|i| i.title.len() + i.content_data.len())
        .sum();

    Ok(serde_json::json!({
        "total_items": total_items,
        "pinned_items": pinned_items,
        "total_copies": total_copies,
        "avg_copies_per_item": avg_copies,
        "most_copied_item": most_copied,
        "storage_bytes": storage_bytes,
        "storage_mb": format!("{:.2} MB", storage_bytes as f64 / 1024.0 / 1024.0)
    }))
}

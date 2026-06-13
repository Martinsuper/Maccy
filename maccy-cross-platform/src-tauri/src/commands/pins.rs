use crate::platform::storage::StoragePlatform;
use crate::platform_impl::storage::SqliteStorage;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn toggle_pin(
    storage: State<'_, Arc<SqliteStorage>>,
    id: String,
) -> Result<bool, String> {
    use crate::platform::storage::StoragePlatform;

    // Get current item to check pin status
    let items = storage.get_all_items().map_err(|e| e.to_string())?;
    let item = items.iter().find(|i| i.id == id);

    if let Some(item) = item {
        let new_pin = if item.pin.is_some() {
            None
        } else {
            // Generate random pin identifier (e.g., "pin_abc123")
            Some(format!("pin_{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap()))
        };

        storage.update_pin(&id, new_pin.as_deref()).map_err(|e| e.to_string())?;
        Ok(new_pin.is_some())
    } else {
        Err("Item not found".to_string())
    }
}

#[tauri::command]
pub async fn get_pinned_items(
    storage: State<'_, Arc<SqliteStorage>>,
) -> Result<Vec<crate::commands::history::HistoryItemDto>, String> {
    use crate::platform::storage::StoragePlatform;

    let items = storage.get_all_items().map_err(|e| e.to_string())?;
    let pinned: Vec<_> = items
        .into_iter()
        .filter(|item| item.pin.is_some())
        .map(|item| item.into())
        .collect();

    Ok(pinned)
}

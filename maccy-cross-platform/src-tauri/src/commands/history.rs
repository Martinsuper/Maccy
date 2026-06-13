use crate::platform::storage::HistoryItem;
use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Arc;

use crate::platform_impl::storage::SqliteStorage;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryItemDto {
    pub id: String,
    pub title: String,
    pub content_type: String,
    pub application: Option<String>,
    pub first_copied_at: i64,
    pub last_copied_at: i64,
    pub number_of_copies: i32,
    pub pin: Option<String>,
}

impl From<HistoryItem> for HistoryItemDto {
    fn from(item: HistoryItem) -> Self {
        Self {
            id: item.id,
            title: item.title,
            content_type: item.content_type,
            application: item.application,
            first_copied_at: item.first_copied_at,
            last_copied_at: item.last_copied_at,
            number_of_copies: item.number_of_copies,
            pin: item.pin,
        }
    }
}

impl From<HistoryItemDto> for HistoryItem {
    fn from(dto: HistoryItemDto) -> Self {
        Self {
            id: dto.id,
            title: dto.title,
            content_type: dto.content_type,
            content_data: vec![], // TODO: Handle content data properly
            application: dto.application,
            first_copied_at: dto.first_copied_at,
            last_copied_at: dto.last_copied_at,
            number_of_copies: dto.number_of_copies,
            pin: dto.pin,
        }
    }
}

#[tauri::command]
pub async fn get_history_items(
    storage: State<'_, Arc<SqliteStorage>>,
) -> Result<Vec<HistoryItemDto>, String> {
    use crate::platform::storage::StoragePlatform;

    storage
        .get_all_items()
        .map(|items| items.into_iter().map(|item| item.into()).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_history_item(
    storage: State<'_, Arc<SqliteStorage>>,
    item: HistoryItemDto,
) -> Result<(), String> {
    use crate::platform::storage::StoragePlatform;

    let history_item: HistoryItem = item.into();
    storage.insert_item(&history_item).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_history_item(
    storage: State<'_, Arc<SqliteStorage>>,
    id: String,
) -> Result<(), String> {
    use crate::platform::storage::StoragePlatform;

    storage.delete_item(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clear_history(
    storage: State<'_, Arc<SqliteStorage>>,
) -> Result<(), String> {
    use crate::platform::storage::StoragePlatform;

    storage.clear_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_history(
    storage: State<'_, Arc<SqliteStorage>>,
    query: String,
) -> Result<Vec<HistoryItemDto>, String> {
    use crate::platform::storage::StoragePlatform;

    storage
        .search_items(&query)
        .map(|items| items.into_iter().map(|item| item.into()).collect())
        .map_err(|e| e.to_string())
}

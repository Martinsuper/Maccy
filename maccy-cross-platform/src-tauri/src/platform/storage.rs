use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to initialize database: {0}")]
    InitError(String),
    #[error("Failed to query database: {0}")]
    QueryError(String),
    #[error("Failed to insert data: {0}")]
    InsertError(String),
    #[error("Failed to delete data: {0}")]
    DeleteError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub id: String,
    pub title: String,
    pub content_type: String,
    pub content_data: Vec<u8>,
    pub application: Option<String>,
    pub first_copied_at: i64,
    pub last_copied_at: i64,
    pub number_of_copies: i32,
    pub pin: Option<String>,
}

pub trait StoragePlatform: Send + Sync {
    fn initialize(&self) -> Result<(), StorageError>;
    fn insert_item(&self, item: &HistoryItem) -> Result<(), StorageError>;
    fn get_all_items(&self) -> Result<Vec<HistoryItem>, StorageError>;
    fn search_items(&self, query: &str) -> Result<Vec<HistoryItem>, StorageError>;
    fn delete_item(&self, id: &str) -> Result<(), StorageError>;
    fn clear_all(&self) -> Result<(), StorageError>;
    fn update_pin(&self, id: &str, pin: Option<&str>) -> Result<(), StorageError>;
}

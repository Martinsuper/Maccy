use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Failed to read clipboard: {0}")]
    ReadError(String),
    #[error("Failed to write clipboard: {0}")]
    WriteError(String),
    #[error("Unsupported content type: {0}")]
    UnsupportedType(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardContent {
    Text(String),
    Html(String),
    Rtf(Vec<u8>),
    Image(Vec<u8>),
    Files(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardMetadata {
    pub source_app: Option<String>,
    pub timestamp: i64,
    pub content_type: String,
}

pub trait ClipboardPlatform: Send + Sync {
    fn read_content(&self) -> Result<ClipboardContent, ClipboardError>;
    fn write_content(&self, content: &ClipboardContent) -> Result<(), ClipboardError>;
    fn get_change_count(&self) -> Result<u64, ClipboardError>;
    fn get_metadata(&self) -> Result<ClipboardMetadata, ClipboardError>;
    fn start_monitoring<F>(&self, callback: F) -> Result<(), ClipboardError>
    where
        F: Fn(ClipboardContent) + Send + Sync + 'static;
    fn stop_monitoring(&self) -> Result<(), ClipboardError>;
}

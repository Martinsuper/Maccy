use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrayError {
    #[error("Failed to create tray icon: {0}")]
    CreateError(String),
    #[error("Failed to update tray icon: {0}")]
    UpdateError(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrayMenuItem {
    pub id: String,
    pub label: String,
    pub enabled: bool,
    pub action: Option<String>,
}

pub trait TrayPlatform: Send + Sync {
    fn create_tray(&self, icon_path: &str) -> Result<(), TrayError>;
    fn set_menu(&self, items: &[TrayMenuItem]) -> Result<(), TrayError>;
    fn set_tooltip(&self, tooltip: &str) -> Result<(), TrayError>;
    fn set_icon(&self, icon_path: &str) -> Result<(), TrayError>;
}

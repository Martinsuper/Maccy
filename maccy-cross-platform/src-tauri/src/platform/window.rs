use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowError {
    #[error("Failed to create window: {0}")]
    CreateError(String),
    #[error("Failed to show window: {0}")]
    ShowError(String),
    #[error("Failed to hide window: {0}")]
    HideError(String),
    #[error("Failed to position window: {0}")]
    PositionError(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub always_on_top: bool,
    pub skip_taskbar: bool,
    pub transparent: bool,
    pub decorations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PopupPosition {
    Cursor,
    MenuBar,
    Center,
    LastPosition,
}

pub trait WindowPlatform: Send + Sync {
    fn create_popup_window(&self, config: &WindowConfig) -> Result<(), WindowError>;
    fn show_popup(&self) -> Result<(), WindowError>;
    fn hide_popup(&self) -> Result<(), WindowError>;
    fn position_popup(&self, position: &PopupPosition) -> Result<(), WindowError>;
    fn is_popup_visible(&self) -> Result<bool, WindowError>;
}

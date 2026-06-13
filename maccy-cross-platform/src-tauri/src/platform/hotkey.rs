use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HotkeyError {
    #[error("Failed to register hotkey: {0}")]
    RegisterError(String),
    #[error("Failed to unregister hotkey: {0}")]
    UnregisterError(String),
    #[error("Hotkey already in use: {0}")]
    AlreadyInUse(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotkey {
    pub id: String,
    pub key: String,
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Modifier {
    Command,
    Control,
    Alt,
    Shift,
}

pub trait HotkeyPlatform: Send + Sync {
    fn register_hotkey(&self, hotkey: &Hotkey) -> Result<(), HotkeyError>;
    fn unregister_hotkey(&self, id: &str) -> Result<(), HotkeyError>;
    fn start_listening<F>(&self, callback: F) -> Result<(), HotkeyError>
    where
        F: Fn(String) + Send + Sync + 'static;
    fn stop_listening(&self) -> Result<(), HotkeyError>;
}

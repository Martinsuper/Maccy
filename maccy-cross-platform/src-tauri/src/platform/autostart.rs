use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutoStartError {
    #[error("Failed to enable auto-start: {0}")]
    EnableError(String),
    #[error("Failed to disable auto-start: {0}")]
    DisableError(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
}

pub trait AutoStartPlatform: Send + Sync {
    fn is_enabled(&self) -> Result<bool, AutoStartError>;
    fn enable(&self) -> Result<(), AutoStartError>;
    fn disable(&self) -> Result<(), AutoStartError>;
}

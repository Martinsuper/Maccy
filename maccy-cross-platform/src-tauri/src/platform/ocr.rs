use thiserror::Error;

#[derive(Error, Debug)]
pub enum OcrError {
    #[error("OCR not supported on this platform")]
    NotSupported,
    #[error("Failed to process image: {0}")]
    ProcessError(String),
    #[error("Platform error: {0}")]
    PlatformError(String),
}

pub trait OcrPlatform: Send + Sync {
    fn is_supported(&self) -> bool;
    fn recognize_text(&self, image_data: &[u8]) -> Result<String, OcrError>;
}

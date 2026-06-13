// Windows clipboard implementation placeholder
// TODO: Implement Win32 clipboard APIs

use crate::platform::clipboard::{ClipboardContent, ClipboardError, ClipboardMetadata, ClipboardPlatform};

pub struct WindowsClipboard;

impl WindowsClipboard {
    pub fn new() -> Self {
        Self
    }
}

impl ClipboardPlatform for WindowsClipboard {
    fn read_content(&self) -> Result<ClipboardContent, ClipboardError> {
        Err(ClipboardError::PlatformError("Windows implementation pending".to_string()))
    }

    fn write_content(&self, _content: &ClipboardContent) -> Result<(), ClipboardError> {
        Err(ClipboardError::PlatformError("Windows implementation pending".to_string()))
    }

    fn get_change_count(&self) -> Result<u64, ClipboardError> {
        Err(ClipboardError::PlatformError("Windows implementation pending".to_string()))
    }

    fn get_metadata(&self) -> Result<ClipboardMetadata, ClipboardError> {
        Err(ClipboardError::PlatformError("Windows implementation pending".to_string()))
    }

    fn start_monitoring<F>(&self, _callback: F) -> Result<(), ClipboardError>
    where
        F: Fn(ClipboardContent) + Send + Sync + 'static,
    {
        Err(ClipboardError::PlatformError("Windows implementation pending".to_string()))
    }

    fn stop_monitoring(&self) -> Result<(), ClipboardError> {
        Err(ClipboardError::PlatformError("Windows implementation pending".to_string()))
    }
}

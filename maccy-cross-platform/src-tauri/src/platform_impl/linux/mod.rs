// Linux clipboard implementation placeholder
// TODO: Implement xclip/wl-clipboard integration

use crate::platform::clipboard::{ClipboardContent, ClipboardError, ClipboardMetadata, ClipboardPlatform};

pub struct LinuxClipboard;

impl LinuxClipboard {
    pub fn new() -> Self {
        Self
    }
}

impl ClipboardPlatform for LinuxClipboard {
    fn read_content(&self) -> Result<ClipboardContent, ClipboardError> {
        Err(ClipboardError::PlatformError("Linux implementation pending".to_string()))
    }

    fn write_content(&self, _content: &ClipboardContent) -> Result<(), ClipboardError> {
        Err(ClipboardError::PlatformError("Linux implementation pending".to_string()))
    }

    fn get_change_count(&self) -> Result<u64, ClipboardError> {
        Err(ClipboardError::PlatformError("Linux implementation pending".to_string()))
    }

    fn get_metadata(&self) -> Result<ClipboardMetadata, ClipboardError> {
        Err(ClipboardError::PlatformError("Linux implementation pending".to_string()))
    }

    fn start_monitoring<F>(&self, _callback: F) -> Result<(), ClipboardError>
    where
        F: Fn(ClipboardContent) + Send + Sync + 'static,
    {
        Err(ClipboardError::PlatformError("Linux implementation pending".to_string()))
    }

    fn stop_monitoring(&self) -> Result<(), ClipboardError> {
        Err(ClipboardError::PlatformError("Linux implementation pending".to_string()))
    }
}

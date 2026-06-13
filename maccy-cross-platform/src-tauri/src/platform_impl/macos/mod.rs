use crate::platform::clipboard::{ClipboardContent, ClipboardError, ClipboardMetadata, ClipboardPlatform};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct MacClipboard {
    monitoring: Arc<AtomicBool>,
}

impl MacClipboard {
    pub fn new() -> Self {
        Self {
            monitoring: Arc::new(AtomicBool::new(false)),
        }
    }

    #[cfg(target_os = "macos")]
    fn get_pasteboard_change_count() -> Result<u64, ClipboardError> {
        use cocoa::appkit::NSPasteboard;
        use cocoa::base::nil;

        unsafe {
            let pasteboard = NSPasteboard::generalPasteboard(nil);
            let count = pasteboard.changeCount();
            Ok(count as u64)
        }
    }

    #[cfg(target_os = "macos")]
    fn read_string_from_pasteboard() -> Result<String, ClipboardError> {
        use cocoa::appkit::NSPasteboard;
        use cocoa::base::nil;
        use cocoa::foundation::NSString;

        unsafe {
            let pasteboard = NSPasteboard::generalPasteboard(nil);
            let contents = pasteboard.stringForType(NSString::alloc(nil).init_str("public.utf8-plain-text"));

            if contents.is_null() {
                return Err(ClipboardError::ReadError("No text in clipboard".to_string()));
            }

            let bytes = contents.UTF8String();
            let string = std::ffi::CStr::from_ptr(bytes)
                .to_str()
                .map_err(|e| ClipboardError::ReadError(e.to_string()))?
                .to_string();

            Ok(string)
        }
    }

    #[cfg(target_os = "macos")]
    fn write_string_to_pasteboard(text: &str) -> Result<(), ClipboardError> {
        use cocoa::appkit::NSPasteboard;
        use cocoa::base::nil;
        use cocoa::foundation::{NSArray, NSString};

        unsafe {
            let pasteboard = NSPasteboard::generalPasteboard(nil);
            pasteboard.clearContents();

            let ns_string = NSString::alloc(nil).init_str(text);
            let array = NSArray::arrayWithObjects(nil, &[ns_string]);

            let success = pasteboard.writeObjects(array);
            if success == 0 {
                return Err(ClipboardError::WriteError("Failed to write to pasteboard".to_string()));
            }

            Ok(())
        }
    }
}

impl ClipboardPlatform for MacClipboard {
    fn read_content(&self) -> Result<ClipboardContent, ClipboardError> {
        #[cfg(target_os = "macos")]
        {
            let text = Self::read_string_from_pasteboard()?;
            Ok(ClipboardContent::Text(text))
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(ClipboardError::PlatformError("Not on macOS".to_string()))
        }
    }

    fn write_content(&self, content: &ClipboardContent) -> Result<(), ClipboardError> {
        #[cfg(target_os = "macos")]
        {
            match content {
                ClipboardContent::Text(text) => Self::write_string_to_pasteboard(text),
                _ => Err(ClipboardError::UnsupportedType("Only text supported in MVP".to_string())),
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(ClipboardError::PlatformError("Not on macOS".to_string()))
        }
    }

    fn get_change_count(&self) -> Result<u64, ClipboardError> {
        #[cfg(target_os = "macos")]
        {
            Self::get_pasteboard_change_count()
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(ClipboardError::PlatformError("Not on macOS".to_string()))
        }
    }

    fn get_metadata(&self) -> Result<ClipboardMetadata, ClipboardError> {
        Ok(ClipboardMetadata {
            source_app: None,
            timestamp: chrono::Utc::now().timestamp(),
            content_type: "text/plain".to_string(),
        })
    }

    fn start_monitoring<F>(&self, callback: F) -> Result<(), ClipboardError>
    where
        F: Fn(ClipboardContent) + Send + Sync + 'static,
    {
        if self.monitoring.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.monitoring.store(true, Ordering::SeqCst);
        let monitoring = Arc::clone(&self.monitoring);

        thread::spawn(move || {
            let mut last_known_count = 0u64;

            while monitoring.load(Ordering::SeqCst) {
                if let Ok(current_count) = Self::get_pasteboard_change_count() {
                    if current_count != last_known_count {
                        last_known_count = current_count;

                        if let Ok(content) = Self::read_string_from_pasteboard() {
                            callback(ClipboardContent::Text(content));
                        }
                    }
                }

                thread::sleep(Duration::from_millis(500));
            }
        });

        Ok(())
    }

    fn stop_monitoring(&self) -> Result<(), ClipboardError> {
        self.monitoring.store(false, Ordering::SeqCst);
        Ok(())
    }
}

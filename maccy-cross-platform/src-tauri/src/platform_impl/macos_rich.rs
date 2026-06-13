use crate::platform::clipboard::{ClipboardContent, ClipboardError, ClipboardMetadata, ClipboardPlatform};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct MacRichClipboard {
    monitoring: Arc<AtomicBool>,
}

impl MacRichClipboard {
    pub fn new() -> Self {
        Self {
            monitoring: Arc::new(AtomicBool::new(false)),
        }
    }

    #[cfg(target_os = "macos")]
    fn get_available_types() -> Vec<String> {
        use cocoa::appkit::NSPasteboard;
        use cocoa::base::nil;
        use cocoa::foundation::NSArray;
        use objc::{msg_send, sel, sel_impl};

        unsafe {
            let pasteboard = NSPasteboard::generalPasteboard(nil);
            let types: *mut NSArray = msg_send![pasteboard, types];

            if types.is_null() {
                return vec![];
            }

            let count: usize = msg_send![types, count];
            let mut result = Vec::new();

            for i in 0..count {
                let item: *mut objc::runtime::Object = msg_send![types, objectAtIndex: i];
                let bytes: *const std::os::raw::c_char = msg_send![item, UTF8String];
                if !bytes.is_null() {
                    let string = std::ffi::CStr::from_ptr(bytes)
                        .to_str()
                        .unwrap_or("")
                        .to_string();
                    result.push(string);
                }
            }

            result
        }
    }

    #[cfg(target_os = "macos")]
    fn read_image() -> Option<Vec<u8>> {
        use cocoa::appkit::NSPasteboard;
        use cocoa::base::nil;
        use cocoa::foundation::NSData;
        use objc::{msg_send, sel, sel_impl};

        unsafe {
            let pasteboard = NSPasteboard::generalPasteboard(nil);

            // Try to get PNG data
            let png_type = cocoa::foundation::NSString::alloc(nil).init_str("public.png");
            let data: *mut NSData = msg_send![pasteboard, dataForType: png_type];

            if !data.is_null() {
                let bytes: *const u8 = msg_send![data, bytes];
                let length: usize = msg_send![data, length];

                if !bytes.is_null() && length > 0 {
                    let slice = std::slice::from_raw_parts(bytes, length);
                    return Some(slice.to_vec());
                }
            }

            None
        }
    }

    #[cfg(target_os = "macos")]
    fn read_html() -> Option<String> {
        use cocoa::appkit::NSPasteboard;
        use cocoa::base::nil;
        use cocoa::foundation::NSString;
        use objc::{msg_send, sel, sel_impl};

        unsafe {
            let pasteboard = NSPasteboard::generalPasteboard(nil);
            let html_type = NSString::alloc(nil).init_str("public.html");
            let content: *mut NSString = msg_send![pasteboard, stringForType: html_type];

            if !content.is_null() {
                let bytes = content.UTF8String();
                if !bytes.is_null() {
                    return std::ffi::CStr::from_ptr(bytes)
                        .to_str()
                        .ok()
                        .map(|s| s.to_string());
                }
            }

            None
        }
    }

    #[cfg(target_os = "macos")]
    fn read_rtf() -> Option<Vec<u8>> {
        use cocoa::appkit::NSPasteboard;
        use cocoa::base::nil;
        use cocoa::foundation::NSData;
        use objc::{msg_send, sel, sel_impl};

        unsafe {
            let pasteboard = NSPasteboard::generalPasteboard(nil);
            let rtf_type = cocoa::foundation::NSString::alloc(nil).init_str("public.rtf");
            let data: *mut NSData = msg_send![pasteboard, dataForType: rtf_type];

            if !data.is_null() {
                let bytes: *const u8 = msg_send![data, bytes];
                let length: usize = msg_send![data, length];

                if !bytes.is_null() && length > 0 {
                    let slice = std::slice::from_raw_parts(bytes, length);
                    return Some(slice.to_vec());
                }
            }

            None
        }
    }
}

impl ClipboardPlatform for MacRichClipboard {
    fn read_content(&self) -> Result<ClipboardContent, ClipboardError> {
        #[cfg(target_os = "macos")]
        {
            // Try image first
            if let Some(image_data) = Self::read_image() {
                return Ok(ClipboardContent::Image(image_data));
            }

            // Try HTML
            if let Some(html) = Self::read_html() {
                return Ok(ClipboardContent::Html(html));
            }

            // Try RTF
            if let Some(rtf_data) = Self::read_rtf() {
                // RTF is binary data, treat as special content
                return Ok(ClipboardContent::Rtf(rtf_data));
            }

            // Fall back to plain text
            use cocoa::appkit::NSPasteboard;
            use cocoa::base::nil;
            use cocoa::foundation::NSString;
            use objc::{msg_send, sel, sel_impl};

            unsafe {
                let pasteboard = NSPasteboard::generalPasteboard(nil);
                let text_type = NSString::alloc(nil).init_str("public.utf8-plain-text");
                let content: *mut NSString = msg_send![pasteboard, stringForType: text_type];

                if !content.is_null() {
                    let bytes = content.UTF8String();
                    if !bytes.is_null() {
                        let string = std::ffi::CStr::from_ptr(bytes)
                            .to_str()
                            .map_err(|e| ClipboardError::ReadError(e.to_string()))?
                            .to_string();
                        return Ok(ClipboardContent::Text(string));
                    }
                }
            }

            Err(ClipboardError::ReadError("No supported content type found".to_string()))
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(ClipboardError::PlatformError("Not on macOS".to_string()))
        }
    }

    fn write_content(&self, content: &ClipboardContent) -> Result<(), ClipboardError> {
        // For now, only support writing plain text
        match content {
            ClipboardContent::Text(text) => {
                #[cfg(target_os = "macos")]
                {
                    use crate::platform_impl::macos::MacClipboard;
                    let clipboard = MacClipboard::new();
                    clipboard.write_content(&ClipboardContent::Text(text.clone()))
                }

                #[cfg(not(target_os = "macos"))]
                {
                    Err(ClipboardError::PlatformError("Not on macOS".to_string()))
                }
            }
            _ => Err(ClipboardError::UnsupportedType("Only text writing supported in MVP".to_string())),
        }
    }

    fn get_change_count(&self) -> Result<u64, ClipboardError> {
        #[cfg(target_os = "macos")]
        {
            use crate::platform_impl::macos::MacClipboard;
            let clipboard = MacClipboard::new();
            clipboard.get_change_count()
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
                #[cfg(target_os = "macos")]
                {
                    use crate::platform_impl::macos::MacClipboard;
                    use crate::platform::clipboard::ClipboardPlatform;

                    let clipboard = MacClipboard::new();
                    if let Ok(current_count) = clipboard.get_change_count() {
                        if current_count != last_known_count {
                            last_known_count = current_count;

                            if let Ok(content) = clipboard.read_content() {
                                callback(content);
                            }
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

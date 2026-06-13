use crate::platform::clipboard::{ClipboardContent, ClipboardPlatform};

#[cfg(target_os = "macos")]
use crate::platform_impl::macos::MacClipboard;

#[cfg(target_os = "windows")]
use crate::platform_impl::windows::WindowsClipboard;

#[cfg(target_os = "linux")]
use crate::platform_impl::linux::LinuxClipboard;

#[tauri::command]
pub async fn get_clipboard_content() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    let clipboard = MacClipboard::new();

    #[cfg(target_os = "windows")]
    let clipboard = WindowsClipboard::new();

    #[cfg(target_os = "linux")]
    let clipboard = LinuxClipboard::new();

    clipboard
        .read_content()
        .map(|content| match content {
            ClipboardContent::Text(text) => text,
            _ => String::from("Unsupported content type"),
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_clipboard_content(text: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let clipboard = MacClipboard::new();

    #[cfg(target_os = "windows")]
    let clipboard = WindowsClipboard::new();

    #[cfg(target_os = "linux")]
    let clipboard = LinuxClipboard::new();

    clipboard
        .write_content(&ClipboardContent::Text(text))
        .map_err(|e| e.to_string())
}

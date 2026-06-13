use crate::services::paste_simulator::PasteSimulator;
use crate::platform::clipboard::{ClipboardContent, ClipboardPlatform};

#[cfg(target_os = "macos")]
use crate::platform_impl::macos::MacClipboard;

#[tauri::command]
pub async fn paste_from_clipboard() -> Result<(), String> {
    let simulator = PasteSimulator::new();
    simulator.paste()
}

#[tauri::command]
pub async fn copy_and_paste(text: String) -> Result<(), String> {
    // Set clipboard content
    #[cfg(target_os = "macos")]
    {
        let clipboard = MacClipboard::new();
        clipboard
            .write_content(&ClipboardContent::Text(text))
            .map_err(|e| e.to_string())?;
    }

    // Wait for clipboard to update
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Simulate paste
    let simulator = PasteSimulator::new();
    simulator.paste()
}

#[tauri::command]
pub async fn check_accessibility_permission() -> bool {
    // TODO: Implement proper accessibility check
    true
}

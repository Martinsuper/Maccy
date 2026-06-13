use std::thread;
use std::time::Duration;

pub struct PasteSimulator;

impl PasteSimulator {
    pub fn new() -> Self {
        Self
    }

    #[cfg(target_os = "macos")]
    pub fn paste(&self) -> Result<(), String> {
        // Check accessibility permission
        if !Self::check_accessibility_permission() {
            return Err("Accessibility permission not granted. Please enable Maccy in System Preferences > Security & Privacy > Accessibility.".to_string());
        }

        // Wait a bit to ensure clipboard is ready
        thread::sleep(Duration::from_millis(100));

        // Simulate Cmd+V using AppleScript
        Self::simulate_cmd_v_applescript()
    }

    #[cfg(target_os = "macos")]
    fn check_accessibility_permission() -> bool {
        // TODO: Implement proper accessibility check using AXIsProcessTrustedWithOptions
        // For now, assume permission is granted
        true
    }

    #[cfg(target_os = "macos")]
    fn simulate_cmd_v_applescript() -> Result<(), String> {
        use std::process::Command;

        // Use osascript to simulate Cmd+V
        let script = r#"
            tell application "System Events"
                keystroke "v" using command down
            end tell
        "#;

        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .map_err(|e| format!("Failed to execute osascript: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("AppleScript failed: {}", stderr));
        }

        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    pub fn paste(&self) -> Result<(), String> {
        Err("Paste simulation not implemented for this platform".to_string())
    }
}

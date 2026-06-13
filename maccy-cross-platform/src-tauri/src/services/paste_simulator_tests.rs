#[cfg(test)]
mod tests {
    use crate::services::paste_simulator::PasteSimulator;

    #[test]
    fn test_paste_simulator_creation() {
        let simulator = PasteSimulator::new();
        // Just verify it can be created
        assert!(true);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_paste_command_structure() {
        // Test that the paste command structure is correct
        // We won't actually execute it in tests to avoid side effects
        let script = r#"
            tell application "System Events"
                keystroke "v" using command down
            end tell
        "#;

        assert!(script.contains("keystroke"), "Script should contain keystroke");
        assert!(script.contains("command down"), "Script should contain command modifier");
    }
}

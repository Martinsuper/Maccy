use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use super::tray::position_window_near_tray;

pub fn register_global_shortcut(app: &tauri::AppHandle) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    let shortcut_str = "Cmd+Shift+C";

    #[cfg(not(target_os = "macos"))]
    let shortcut_str = "Ctrl+Shift+C";

    let shortcut: Shortcut = shortcut_str.parse().unwrap();

    if let Err(e) = app.global_shortcut().on_shortcut(shortcut, |app, _shortcut, event| {
        if event.state == ShortcutState::Pressed {
            log::info!("Global shortcut pressed");
            if let Some(window) = app.get_webview_window("main") {
                match window.is_visible() {
                    Ok(true) => {
                        let _ = window.hide();
                    }
                    _ => {
                        position_window_near_tray(&window);
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        }
    }) {
        log::error!("Failed to register shortcut: {}", e);
    }

    log::info!("Global shortcut registered");
    Ok(())
}

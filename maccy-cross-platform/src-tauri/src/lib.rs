mod platform;
mod platform_impl;
mod commands;
mod models;
mod services;

use std::sync::Arc;
use tauri::Manager;
use platform_impl::storage::SqliteStorage;
use services::clipboard_monitor::ClipboardMonitor;
use services::tray::create_tray;
use services::hotkey::register_global_shortcut;
use services::settings::SettingsManager;

#[cfg(target_os = "macos")]
use platform_impl::macos::MacClipboard;

#[cfg(target_os = "windows")]
use platform_impl::windows::WindowsClipboard;

#[cfg(target_os = "linux")]
use platform_impl::linux::LinuxClipboard;

pub fn run() {
    env_logger::init();
    log::info!("Starting Maccy cross-platform clipboard manager");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            log::info!("Setting up Maccy application");

            // Initialize settings
            let settings_manager = Arc::new(SettingsManager::new()
                .expect("Failed to initialize settings"));
            app.manage(Arc::clone(&settings_manager));

            // Initialize storage
            let storage = Arc::new(SqliteStorage::new(None)
                .expect("Failed to initialize storage"));
            app.manage(Arc::clone(&storage));

            // Start clipboard monitoring
            let monitor = ClipboardMonitor::new(Arc::clone(&storage), 500);
            if let Err(e) = monitor.start() {
                log::error!("Failed to start clipboard monitor: {}", e);
            }
            app.manage(monitor);

            // Create system tray
            if let Err(e) = create_tray(app.handle()) {
                log::error!("Failed to create tray: {}", e);
            }

            // Register global shortcut
            if let Err(e) = register_global_shortcut(app.handle()) {
                log::error!("Failed to register global shortcut: {}", e);
            }

            // Initialize platform-specific clipboard
            #[cfg(target_os = "macos")]
            {
                let clipboard = MacClipboard::new();
                app.manage(clipboard);
            }

            #[cfg(target_os = "windows")]
            {
                let clipboard = WindowsClipboard::new();
                app.manage(clipboard);
            }

            #[cfg(target_os = "linux")]
            {
                let clipboard = LinuxClipboard::new();
                app.manage(clipboard);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::clipboard::get_clipboard_content,
            commands::clipboard::set_clipboard_content,
            commands::history::get_history_items,
            commands::history::add_history_item,
            commands::history::delete_history_item,
            commands::history::clear_history,
            commands::history::search_history,
            commands::paste::paste_from_clipboard,
            commands::paste::copy_and_paste,
            commands::paste::check_accessibility_permission,
            commands::pins::toggle_pin,
            commands::pins::get_pinned_items,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::reset_settings,
            commands::smart::paste_stack,
            commands::smart::auto_cleanup,
            commands::smart::smart_search,
            commands::smart::get_statistics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

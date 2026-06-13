use crate::services::settings::{AppSettings, SettingsManager};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_settings(
    settings_manager: State<'_, Arc<SettingsManager>>,
) -> Result<AppSettings, String> {
    Ok(settings_manager.get_settings())
}

#[tauri::command]
pub async fn update_settings(
    settings_manager: State<'_, Arc<SettingsManager>>,
    settings: AppSettings,
) -> Result<(), String> {
    settings_manager.update_settings(settings)
}

#[tauri::command]
pub async fn reset_settings(
    settings_manager: State<'_, Arc<SettingsManager>>,
) -> Result<(), String> {
    settings_manager.reset_to_default()
}

/*
 *    This file is part of Quick Reader.
 *
 *    Quick Reader is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    Quick Reader is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License
 *    along with Quick Reader.  If not, see <https://www.gnu.org/licenses/>.
 */

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;

use common::AppSettings;
use tauri::AppHandle;

#[tauri::command]
async fn load_settings(app_handle: AppHandle) -> Result<AppSettings, String> {
    let settings_dir = app_handle
        .path_resolver()
        .app_config_dir()
        .ok_or_else(|| "could not resolve config dir".to_string())?;

    let reader = std::fs::OpenOptions::new()
        .read(true)
        .open(settings_dir.join("settings.json"))
        .map_err(|e| format!("cannot open settings.json: {e}"))?;

    serde_json::from_reader(reader).map_err(|e| format!("error reading settings.json: {e}"))
}

#[tauri::command]
async fn save_settings(app_handle: AppHandle, settings_serialized: &str) -> Result<(), String> {
    let settings_dir = app_handle
        .path_resolver()
        .app_config_dir()
        .ok_or_else(|| "could not resolve config dir".to_string())?;

    if !settings_dir.is_dir() {
        std::fs::create_dir(&settings_dir).map_err(|e| format!("error creating dir: {e}"))?;
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(settings_dir.join("settings.json"))
        .map_err(|e| format!("error writing settings.json: {e}"))?;

    file.write_all(settings_serialized.as_bytes())
        .map_err(|e| format!("error saving settings.json: {e}"))
}

#[tauri::command]
async fn get_system_fonts() -> Result<Vec<String>, String> {
    font_kit::sources::fs::FsSource::new()
        .all_families()
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,
            get_system_fonts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

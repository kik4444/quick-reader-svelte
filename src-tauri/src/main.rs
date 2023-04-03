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

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod appsettings;
use crate::appsettings::AppSettings;
use quick_reader::Error;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State, Window};

type Settings = Mutex<AppSettings>;

#[tauri::command]
async fn get_settings(settings: State<'_, Settings>) -> Result<AppSettings, ()> {
    Ok(settings.lock().expect("mutex poisoned").clone())
}

#[tauri::command]
async fn set_settings(
    current_settings: State<'_, Settings>,
    new_settings: AppSettings,
) -> Result<(), ()> {
    *current_settings.lock().expect("mutex poisoned") = new_settings;
    Ok(())
}

#[tauri::command]
async fn get_system_fonts() -> Result<Vec<String>, Error> {
    font_kit::sources::fs::FsSource::new()
        .all_families()
        .map_err(Error::FontError)
}

fn update_window_dimensions(window: &Window, settings: State<Settings>) -> Result<(), Error> {
    let size = window.inner_size()?;

    let mut app_settings = settings.lock().expect("mutex poisoned");

    app_settings.window.width = size.width;
    app_settings.window.height = size.height;

    Ok(())
}

fn save_settings(app_handle: &AppHandle, settings: State<Settings>) -> Result<(), Error> {
    let settings_dir = app_handle
        .path_resolver()
        .app_config_dir()
        .ok_or_else(|| Error::TauriPath("Could not resolve app config dir".into()))?;

    if !settings_dir.is_dir() {
        std::fs::create_dir(&settings_dir)?;
    }

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(settings_dir.join("settings.json"))?;

    serde_json::to_writer_pretty(file, &*settings.lock().expect("mutex poisoned"))
        .map_err(Error::Serde)
}
fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            let app_settings = app
                .path_resolver()
                .app_config_dir()
                .map(|dir| dir.join("settings.json"))
                .and_then(|path| std::fs::read_to_string(path).ok())
                .and_then(|json_string| serde_json::from_str::<AppSettings>(&json_string).ok())
                .unwrap_or_default();

            _ = app
                .get_window("main")
                .expect("failed getting main window")
                .set_size(tauri::PhysicalSize::new(
                    app_settings.window.width,
                    app_settings.window.height,
                ));

            app.manage(Mutex::new(app_settings));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            set_settings,
            get_system_fonts
        ])
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
                let window = event.window();
                _ = update_window_dimensions(window, window.app_handle().state::<Settings>());
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::ExitRequested { .. } = event {
            _ = save_settings(app_handle, app_handle.state::<Settings>());
        }
    });
}

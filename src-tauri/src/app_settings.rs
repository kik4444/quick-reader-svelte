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

use serde::{Deserialize, Serialize};

// If serde encounters an error during deserialization of a child struct
// it will even use the default value of the parent struct.
// E.g. if an error occurs while deserializing SettingsFonts,
// even all of AppSettings will assume its default value if we apply unwrap_or_default

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
struct SettingsFonts {
    display_font_style: String,
    display_font_size: u8,
    textarea_font_style: String,
    textarea_font_size: u8,
}

impl Default for SettingsFonts {
    fn default() -> Self {
        Self {
            display_font_style: "Monospace".into(),
            display_font_size: 35,
            textarea_font_style: "Monospace".into(),
            textarea_font_size: 12,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SettingsWindow {
    pub width: u32,
    pub height: u32,
    #[serde(deserialize_with = "validate_style")]
    style: String,
    #[serde(deserialize_with = "validate_theme")]
    theme: String,
}

impl Default for SettingsWindow {
    fn default() -> Self {
        Self {
            width: 680,
            height: 560,
            style: "Auto".into(),
            theme: "Auto".into(),
        }
    }
}

fn validate_style<'de, D>(d: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let style = String::deserialize(d).unwrap_or_else(|_| String::new());

    Ok(match style.as_str() {
        "Windows Mica" | "MacOS Monterey" | "Linux Breeze" => style,
        _ => "Auto".into(),
    })
}

fn validate_theme<'de, D>(d: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let theme = String::deserialize(d).unwrap_or_else(|_| String::new());

    Ok(match theme.as_str() {
        "Dark" | "Light" => theme,
        _ => "Auto".into(),
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
struct SettingsPlayback {
    jump_back_chunks: u16,
    jump_forward_chunks: u16,
}

impl Default for SettingsPlayback {
    fn default() -> Self {
        Self {
            jump_back_chunks: 10,
            jump_forward_chunks: 10,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct AppSettings {
    fonts: SettingsFonts,
    pub window: SettingsWindow,
    playback: SettingsPlayback,
}

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

// This file is merely for sharing data between the backend and frontend

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Auto,
    Dark,
    Light,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Theme::Auto => "Auto",
                Theme::Dark => "Dark",
                Theme::Light => "Light",
            }
        )
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Auto" => Ok(Self::Auto),
            "Dark" => Ok(Self::Dark),
            "Light" => Ok(Self::Light),
            s => Err(format!("invalid theme {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub display_font_style: String,
    pub display_font_size: usize,
    pub textarea_font_style: String,
    pub textarea_font_size: usize,
    pub jump_back_chunks: usize,
    pub jump_forward_chunks: usize,
    pub theme: Theme,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            display_font_style: "Monospace".into(),
            display_font_size: 35,
            textarea_font_style: "Monospace".into(),
            textarea_font_size: 12,
            jump_back_chunks: 5,
            jump_forward_chunks: 5,
            theme: Theme::default(),
        }
    }
}

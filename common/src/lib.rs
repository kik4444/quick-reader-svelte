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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub display_font_style: String,
    pub display_font_size: u8,
    pub textarea_font_style: String,
    pub textarea_font_size: u8,
    pub jump_back_chunks: usize,
    pub jump_forward_chunks: usize,
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
        }
    }
}

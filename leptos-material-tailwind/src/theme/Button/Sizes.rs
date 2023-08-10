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

use phf::phf_map;

use crate::JoinFields;

pub(crate) struct Size {
    pub(crate) font_size: &'static str,
    pub(crate) py: &'static str,
    pub(crate) px: &'static str,
    pub(crate) border_radius: &'static str,
}

impl JoinFields for Size {
    fn fields_to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.font_size, self.py, self.px, self.border_radius
        )
    }
}

pub(crate) static SIZES: phf::Map<&'static str, Size> = phf_map! {
    "sm" => Size {
        font_size: "text-xs",
        py: "py-2",
        px: "px-4",
        border_radius: "rounded-lg",
    },
    "md" => Size {
        font_size: "text-xs",
        py: "py-3",
        px: "px-6",
        border_radius: "rounded-lg",
    },
    "lg" => Size {
        font_size: "text-xm",
        py: "py-3.5",
        px: "px-7",
        border_radius: "rounded-lg",
    },
};

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

use super::Base::{ButtonColors, ButtonColors::*, Theme};

pub(crate) fn get_theme(color: ButtonColors) -> Theme {
    match color {
        White => Theme {
            color: "text-white",
            hover: "hover:bg-white/10",
            active: "active:bg-white/30",
            ..Default::default()
        },
        BlueGray => Theme {
            color: "text-blue-gray-500",
            hover: "hover:bg-blue-gray-500/10",
            active: "active:bg-blue-gray-500/30",
            ..Default::default()
        },
        Gray => Theme {
            color: "text-gray-900",
            hover: "hover:bg-gray-900/10",
            active: "active:bg-gray-900/20",
            ..Default::default()
        },
        Brown => Theme {
            color: "text-brown-500",
            hover: "hover:bg-brown-500/10",
            active: "active:bg-brown-500/30",
            ..Default::default()
        },
        DeepOrange => Theme {
            color: "text-deep-orange-500",
            hover: "hover:bg-deep-orange-500/10",
            active: "active:bg-deep-orange-500/30",
            ..Default::default()
        },
        Orange => Theme {
            color: "text-orange-500",
            hover: "hover:bg-orange-500/10",
            active: "active:bg-orange-500/30",
            ..Default::default()
        },
        Amber => Theme {
            color: "text-amber-500",
            hover: "hover:bg-amber-500/10",
            active: "active:bg-amber-500/30",
            ..Default::default()
        },
        Yellow => Theme {
            color: "text-yellow-500",
            hover: "hover:bg-yellow-500/10",
            active: "active:bg-yellow-500/30",
            ..Default::default()
        },
        Lime => Theme {
            color: "text-lime-500",
            hover: "hover:bg-lime-500/10",
            active: "active:bg-lime-500/30",
            ..Default::default()
        },
        LightGreen => Theme {
            color: "text-light-green-500",
            hover: "hover:bg-light-green-500/10",
            active: "active:bg-light-green-500/30",
            ..Default::default()
        },
        Green => Theme {
            color: "text-green-500",
            hover: "hover:bg-green-500/10",
            active: "active:bg-green-500/30",
            ..Default::default()
        },
        Teal => Theme {
            color: "text-teal-500",
            hover: "hover:bg-teal-500/10",
            active: "active:bg-teal-500/30",
            ..Default::default()
        },
        Cyan => Theme {
            color: "text-cyan-500",
            hover: "hover:bg-cyan-500/10",
            active: "active:bg-cyan-500/30",
            ..Default::default()
        },
        LightBlue => Theme {
            color: "text-light-blue-500",
            hover: "hover:bg-light-blue-500/10",
            active: "active:bg-light-blue-500/30",
            ..Default::default()
        },
        Blue => Theme {
            color: "text-blue-500",
            hover: "hover:bg-blue-500/10",
            active: "active:bg-blue-500/30",
            ..Default::default()
        },
        Indigo => Theme {
            color: "text-indigo-500",
            hover: "hover:bg-indigo-500/10",
            active: "active:bg-indigo-500/30",
            ..Default::default()
        },
        DeepPurple => Theme {
            color: "text-deep-purple-500",
            hover: "hover:bg-deep-purple-500/10",
            active: "active:bg-deep-purple-500/30",
            ..Default::default()
        },
        Purple => Theme {
            color: "text-purple-500",
            hover: "hover:bg-purple-500/10",
            active: "active:bg-purple-500/30",
            ..Default::default()
        },
        Pink => Theme {
            color: "text-pink-500",
            hover: "hover:bg-pink-500/10",
            active: "active:bg-pink-500/30",
            ..Default::default()
        },
        Red => Theme {
            color: "text-red-500",
            hover: "hover:bg-red-500/10",
            active: "active:bg-red-500/30",
            ..Default::default()
        },
    }
}

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
            background: "bg-white",
            color: "text-blue-gray-900",
            shadow: "shadow-md shadow-blue-gray-500/10",
            hover: "hover:shadow-lg hover:shadow-blue-gray-500/20",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        BlueGray => Theme {
            background: "bg-blue-gray-500",
            color: "text-white",
            shadow: "shadow-md shadow-blue-gray-500/20",
            hover: "hover:shadow-lg hover:shadow-blue-gray-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Gray => Theme {
            background: "bg-gray-900",
            color: "text-white",
            shadow: "shadow-md shadow-gray-900/10",
            hover: "hover:shadow-lg hover:shadow-gray-900/20",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Brown => Theme {
            background: "bg-brown-500",
            color: "text-white",
            shadow: "shadow-md shadow-brown-500/20",
            hover: "hover:shadow-lg hover:shadow-brown-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        DeepOrange => Theme {
            background: "bg-deep-orange-500",
            color: "text-white",
            shadow: "shadow-md shadow-deep-orange-500/20",
            hover: "hover:shadow-lg hover:shadow-deep-orange-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Orange => Theme {
            background: "bg-orange-500",
            color: "text-white",
            shadow: "shadow-md shadow-orange-500/20",
            hover: "hover:shadow-lg hover:shadow-orange-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Amber => Theme {
            background: "bg-amber-500",
            color: "text-black",
            shadow: "shadow-md shadow-amber-500/20",
            hover: "hover:shadow-lg hover:shadow-amber-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Yellow => Theme {
            background: "bg-yellow-500",
            color: "text-black",
            shadow: "shadow-md shadow-yellow-500/20",
            hover: "hover:shadow-lg hover:shadow-yellow-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Lime => Theme {
            background: "bg-lime-500",
            color: "text-black",
            shadow: "shadow-md shadow-lime-500/20",
            hover: "hover:shadow-lg hover:shadow-lime-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        LightGreen => Theme {
            background: "bg-light-green-500",
            color: "text-white",
            shadow: "shadow-md shadow-light-green-500/20",
            hover: "hover:shadow-lg hover:shadow-light-green-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Green => Theme {
            background: "bg-green-500",
            color: "text-white",
            shadow: "shadow-md shadow-green-500/20",
            hover: "hover:shadow-lg hover:shadow-green-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Teal => Theme {
            background: "bg-teal-500",
            color: "text-white",
            shadow: "shadow-md shadow-teal-500/20",
            hover: "hover:shadow-lg hover:shadow-teal-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Cyan => Theme {
            background: "bg-cyan-500",
            color: "text-white",
            shadow: "shadow-md shadow-cyan-500/20",
            hover: "hover:shadow-lg hover:shadow-cyan-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        LightBlue => Theme {
            background: "bg-light-blue-500",
            color: "text-white",
            shadow: "shadow-md shadow-light-blue-500/20",
            hover: "hover:shadow-lg hover:shadow-light-blue-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Blue => Theme {
            background: "bg-blue-500",
            color: "text-white",
            shadow: "shadow-md shadow-blue-500/20",
            hover: "hover:shadow-lg hover:shadow-blue-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Indigo => Theme {
            background: "bg-indigo-500",
            color: "text-white",
            shadow: "shadow-md shadow-indigo-500/20",
            hover: "hover:shadow-lg hover:shadow-indigo-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        DeepPurple => Theme {
            background: "bg-deep-purple-500",
            color: "text-white",
            shadow: "shadow-md shadow-deep-purple-500/20",
            hover: "hover:shadow-lg hover:shadow-deep-purple-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Purple => Theme {
            background: "bg-purple-500",
            color: "text-white",
            shadow: "shadow-md shadow-purple-500/20",
            hover: "hover:shadow-lg hover:shadow-purple-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Pink => Theme {
            background: "bg-pink-500",
            color: "text-white",
            shadow: "shadow-md shadow-pink-500/20",
            hover: "hover:shadow-lg hover:shadow-pink-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
        Red => Theme {
            background: "bg-red-500",
            color: "text-white",
            shadow: "shadow-md shadow-red-500/20",
            hover: "hover:shadow-lg hover:shadow-red-500/40",
            focus: "focus:opacity-[0.85] focus:shadow-none",
            active: "active:opacity-[0.85] active:shadow-none",
            ..Default::default()
        },
    }
}

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
            border: "border border-white",
            color: "text-white",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-white/50",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        BlueGray => Theme {
            border: "border border-blue-gray-500",
            color: "text-blue-gray-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-blue-gray-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Gray => Theme {
            border: "border border-gray-900",
            color: "text-gray-900",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-gray-300",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Brown => Theme {
            border: "border border-brown-500",
            color: "text-brown-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-brown-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        DeepOrange => Theme {
            border: "border border-deep-orange-500",
            color: "text-deep-orange-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-deep-orange-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Orange => Theme {
            border: "border border-orange-500",
            color: "text-orange-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-orange-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Amber => Theme {
            border: "border border-amber-500",
            color: "text-amber-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-amber-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Yellow => Theme {
            border: "border border-yellow-500",
            color: "text-yellow-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-yellow-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Lime => Theme {
            border: "border border-lime-500",
            color: "text-lime-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-lime-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        LightGreen => Theme {
            border: "border border-light-green-500",
            color: "text-light-green-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-light-green-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Green => Theme {
            border: "border border-green-500",
            color: "text-green-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-green-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Teal => Theme {
            border: "border border-teal-500",
            color: "text-teal-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-teal-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Cyan => Theme {
            border: "border border-cyan-500",
            color: "text-cyan-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-cyan-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        LightBlue => Theme {
            border: "border border-light-blue-500",
            color: "text-light-blue-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-light-blue-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Blue => Theme {
            border: "border border-blue-500",
            color: "text-blue-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-blue-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Indigo => Theme {
            border: "border border-indigo-500",
            color: "text-indigo-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-indigo-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        DeepPurple => Theme {
            border: "border border-deep-purple-500",
            color: "text-deep-purple-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-deep-purple-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Purple => Theme {
            border: "border border-purple-500",
            color: "text-purple-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-purple-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Pink => Theme {
            border: "border border-pink-500",
            color: "text-pink-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-pink-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
        Red => Theme {
            border: "border border-red-500",
            color: "text-red-500",
            hover: "hover:opacity-75",
            focus: "focus:ring focus:ring-red-200",
            active: "active:opacity-[0.85]",
            ..Default::default()
        },
    }
}

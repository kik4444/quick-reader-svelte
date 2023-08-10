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

use super::Base::Theme;

pub(crate) static BUTTON_OUTLINED: phf::Map<&'static str, Theme> = phf_map! {
  "white" => Theme {
    border: "border border-white",
    color: "text-white",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-white/50",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "blue-gray" => Theme {
    border: "border border-blue-gray-500",
    color: "text-blue-gray-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-blue-gray-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "gray" => Theme {
    border: "border border-gray-900",
    color: "text-gray-900",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-gray-300",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "brown" => Theme {
    border: "border border-brown-500",
    color: "text-brown-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-brown-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "deep-orange" => Theme {
    border: "border border-deep-orange-500",
    color: "text-deep-orange-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-deep-orange-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "orange" => Theme {
    border: "border border-orange-500",
    color: "text-orange-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-orange-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "amber" => Theme {
    border: "border border-amber-500",
    color: "text-amber-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-amber-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "yellow" => Theme {
    border: "border border-yellow-500",
    color: "text-yellow-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-yellow-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "lime" => Theme {
    border: "border border-lime-500",
    color: "text-lime-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-lime-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "light-green" => Theme {
    border: "border border-light-green-500",
    color: "text-light-green-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-light-green-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "green" => Theme {
    border: "border border-green-500",
    color: "text-green-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-green-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "teal" => Theme {
    border: "border border-teal-500",
    color: "text-teal-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-teal-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "cyan" => Theme {
    border: "border border-cyan-500",
    color: "text-cyan-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-cyan-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "light-blue" => Theme {
    border: "border border-light-blue-500",
    color: "text-light-blue-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-light-blue-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "blue" => Theme {
    border: "border border-blue-500",
    color: "text-blue-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-blue-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "indigo" => Theme {
    border: "border border-indigo-500",
    color: "text-indigo-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-indigo-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "deep-purple" => Theme {
    border: "border border-deep-purple-500",
    color: "text-deep-purple-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-deep-purple-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "purple" => Theme {
    border: "border border-purple-500",
    color: "text-purple-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-purple-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "pink" => Theme {
    border: "border border-pink-500",
    color: "text-pink-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-pink-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "red" => Theme {
    border: "border border-red-500",
    color: "text-red-500",
    hover: "hover:opacity-75",
    focus: "focus:ring focus:ring-red-200",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
};

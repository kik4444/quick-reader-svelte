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

pub(crate) static BUTTON_FILLED: phf::Map<&'static str, Theme> = phf_map! {
  "white" => Theme {
    background: "bg-white",
    color: "text-blue-gray-900",
    shadow: "shadow-md shadow-blue-gray-500/10",
    hover: "hover:shadow-lg hover:shadow-blue-gray-500/20",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "blue-gray" => Theme {
    background: "bg-blue-gray-500",
    color: "text-white",
    shadow: "shadow-md shadow-blue-gray-500/20",
    hover: "hover:shadow-lg hover:shadow-blue-gray-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "gray" => Theme {
    background: "bg-gray-900",
    color: "text-white",
    shadow: "shadow-md shadow-gray-900/10",
    hover: "hover:shadow-lg hover:shadow-gray-900/20",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "brown" => Theme {
    background: "bg-brown-500",
    color: "text-white",
    shadow: "shadow-md shadow-brown-500/20",
    hover: "hover:shadow-lg hover:shadow-brown-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "deep-orange" => Theme {
    background: "bg-deep-orange-500",
    color: "text-white",
    shadow: "shadow-md shadow-deep-orange-500/20",
    hover: "hover:shadow-lg hover:shadow-deep-orange-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "orange" => Theme {
    background: "bg-orange-500",
    color: "text-white",
    shadow: "shadow-md shadow-orange-500/20",
    hover: "hover:shadow-lg hover:shadow-orange-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "amber" => Theme {
    background: "bg-amber-500",
    color: "text-black",
    shadow: "shadow-md shadow-amber-500/20",
    hover: "hover:shadow-lg hover:shadow-amber-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "yellow" => Theme {
    background: "bg-yellow-500",
    color: "text-black",
    shadow: "shadow-md shadow-yellow-500/20",
    hover: "hover:shadow-lg hover:shadow-yellow-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "lime" => Theme {
    background: "bg-lime-500",
    color: "text-black",
    shadow: "shadow-md shadow-lime-500/20",
    hover: "hover:shadow-lg hover:shadow-lime-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "light-green" => Theme {
    background: "bg-light-green-500",
    color: "text-white",
    shadow: "shadow-md shadow-light-green-500/20",
    hover: "hover:shadow-lg hover:shadow-light-green-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "green" => Theme {
    background: "bg-green-500",
    color: "text-white",
    shadow: "shadow-md shadow-green-500/20",
    hover: "hover:shadow-lg hover:shadow-green-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "teal" => Theme {
    background: "bg-teal-500",
    color: "text-white",
    shadow: "shadow-md shadow-teal-500/20",
    hover: "hover:shadow-lg hover:shadow-teal-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "cyan" => Theme {
    background: "bg-cyan-500",
    color: "text-white",
    shadow: "shadow-md shadow-cyan-500/20",
    hover: "hover:shadow-lg hover:shadow-cyan-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "light-blue" => Theme {
    background: "bg-light-blue-500",
    color: "text-white",
    shadow: "shadow-md shadow-light-blue-500/20",
    hover: "hover:shadow-lg hover:shadow-light-blue-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "blue" => Theme {
    background: "bg-blue-500",
    color: "text-white",
    shadow: "shadow-md shadow-blue-500/20",
    hover: "hover:shadow-lg hover:shadow-blue-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "indigo" => Theme {
    background: "bg-indigo-500",
    color: "text-white",
    shadow: "shadow-md shadow-indigo-500/20",
    hover: "hover:shadow-lg hover:shadow-indigo-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "deep-purple" => Theme {
    background: "bg-deep-purple-500",
    color: "text-white",
    shadow: "shadow-md shadow-deep-purple-500/20",
    hover: "hover:shadow-lg hover:shadow-deep-purple-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "purple" => Theme {
    background: "bg-purple-500",
    color: "text-white",
    shadow: "shadow-md shadow-purple-500/20",
    hover: "hover:shadow-lg hover:shadow-purple-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "pink" => Theme {
    background: "bg-pink-500",
    color: "text-white",
    shadow: "shadow-md shadow-pink-500/20",
    hover: "hover:shadow-lg hover:shadow-pink-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "red" => Theme {
    background: "bg-red-500",
    color: "text-white",
    shadow: "shadow-md shadow-red-500/20",
    hover: "hover:shadow-lg hover:shadow-red-500/40",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
};

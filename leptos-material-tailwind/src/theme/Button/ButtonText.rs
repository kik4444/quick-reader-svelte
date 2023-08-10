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

pub(crate) static BUTTON_TEXT: phf::Map<&'static str, Theme> = phf_map! {
  "white" => Theme {
    color: "text-white",
    hover: "hover:bg-white/10",
    active: "active:bg-white/30",
    ..Theme::const_default()
  },
  "blue-gray" => Theme {
    color: "text-blue-gray-500",
    hover: "hover:bg-blue-gray-500/10",
    active: "active:bg-blue-gray-500/30",
    ..Theme::const_default()
  },
  "gray" => Theme {
    color: "text-gray-900",
    hover: "hover:bg-gray-900/10",
    active: "active:bg-gray-900/20",
    ..Theme::const_default()
  },
  "brown" => Theme {
    color: "text-brown-500",
    hover: "hover:bg-brown-500/10",
    active: "active:bg-brown-500/30",
    ..Theme::const_default()
  },
  "deep-orange" => Theme {
    color: "text-deep-orange-500",
    hover: "hover:bg-deep-orange-500/10",
    active: "active:bg-deep-orange-500/30",
    ..Theme::const_default()
  },
  "orange" => Theme {
    color: "text-orange-500",
    hover: "hover:bg-orange-500/10",
    active: "active:bg-orange-500/30",
    ..Theme::const_default()
  },
  "amber" => Theme {
    color: "text-amber-500",
    hover: "hover:bg-amber-500/10",
    active: "active:bg-amber-500/30",
    ..Theme::const_default()
  },
  "yellow" => Theme {
    color: "text-yellow-500",
    hover: "hover:bg-yellow-500/10",
    active: "active:bg-yellow-500/30",
    ..Theme::const_default()
  },
  "lime" => Theme {
    color: "text-lime-500",
    hover: "hover:bg-lime-500/10",
    active: "active:bg-lime-500/30",
    ..Theme::const_default()
  },
  "light-green" => Theme {
    color: "text-light-green-500",
    hover: "hover:bg-light-green-500/10",
    active: "active:bg-light-green-500/30",
    ..Theme::const_default()
  },
  "green" => Theme {
    color: "text-green-500",
    hover: "hover:bg-green-500/10",
    active: "active:bg-green-500/30",
    ..Theme::const_default()
  },
  "teal" => Theme {
    color: "text-teal-500",
    hover: "hover:bg-teal-500/10",
    active: "active:bg-teal-500/30",
    ..Theme::const_default()
  },
  "cyan" => Theme {
    color: "text-cyan-500",
    hover: "hover:bg-cyan-500/10",
    active: "active:bg-cyan-500/30",
    ..Theme::const_default()
  },
  "light-blue" => Theme {
    color: "text-light-blue-500",
    hover: "hover:bg-light-blue-500/10",
    active: "active:bg-light-blue-500/30",
    ..Theme::const_default()
  },
  "blue" => Theme {
    color: "text-blue-500",
    hover: "hover:bg-blue-500/10",
    active: "active:bg-blue-500/30",
    ..Theme::const_default()
  },
  "indigo" => Theme {
    color: "text-indigo-500",
    hover: "hover:bg-indigo-500/10",
    active: "active:bg-indigo-500/30",
    ..Theme::const_default()
  },
  "deep-purple" => Theme {
    color: "text-deep-purple-500",
    hover: "hover:bg-deep-purple-500/10",
    active: "active:bg-deep-purple-500/30",
    ..Theme::const_default()
  },
  "purple" => Theme {
    color: "text-purple-500",
    hover: "hover:bg-purple-500/10",
    active: "active:bg-purple-500/30",
    ..Theme::const_default()
  },
  "pink" => Theme {
    color: "text-pink-500",
    hover: "hover:bg-pink-500/10",
    active: "active:bg-pink-500/30",
    ..Theme::const_default()
  },
  "red" => Theme {
    color: "text-red-500",
    hover: "hover:bg-red-500/10",
    active: "active:bg-red-500/30",
    ..Theme::const_default()
  },
};

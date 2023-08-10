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

use leptos::*;
use phf::phf_map;

use super::{
    ButtonFilled::BUTTON_FILLED, ButtonGradient::BUTTON_GRADIENT, ButtonOutlined::BUTTON_OUTLINED,
    ButtonText::BUTTON_TEXT, Sizes::SIZES,
};
use crate::JoinFields;

pub(crate) struct Theme {
    pub(crate) background: &'static str,
    pub(crate) border: &'static str,
    pub(crate) color: &'static str,
    pub(crate) shadow: &'static str,
    pub(crate) hover: &'static str,
    pub(crate) focus: &'static str,
    pub(crate) active: &'static str,
}

impl Theme {
    pub(crate) const fn const_default() -> Self {
        Self {
            background: "",
            border: "",
            color: "",
            shadow: "",
            hover: "",
            focus: "",
            active: "",
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::const_default()
    }
}

impl JoinFields for Theme {
    fn fields_to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {} {}",
            self.background,
            self.border,
            self.color,
            self.shadow,
            self.hover,
            self.focus,
            self.active
        )
    }
}

static VARIANTS: phf::Map<&'static str, &phf::Map<&'static str, Theme>> = phf_map! {
    "filled" => &BUTTON_FILLED,
    "gradient" => &BUTTON_GRADIENT,
    "outlined" => &BUTTON_OUTLINED,
    "text" => &BUTTON_TEXT,
};

const DEFAULT_VARIANT: &str = "filled";
const DEFAULT_COLOR: &str = "light-blue";
const DEFAULT_SIZE: &str = "md";

#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<html::Button>>,
    #[prop(optional, into)] variant: Option<MaybeSignal<String>>,
    #[prop(optional, into)] color: Option<MaybeSignal<String>>,
    #[prop(optional, into)] size: Option<MaybeSignal<String>>,
    // #[prop(default = "md".into())] size: String,
) -> impl IntoView {
    let variant = move || {
        variant
            .as_ref()
            .map(|v| v())
            .unwrap_or_else(|| DEFAULT_VARIANT.into())
    };

    let button_type = move || {
        let v = variant();
        VARIANTS.get(&v).unwrap_or_else(|| {
            log::warn!("variant {v} not found");
            &VARIANTS[DEFAULT_VARIANT]
        })
    };

    let color = move || {
        color
            .as_ref()
            .map(|c| c())
            .unwrap_or_else(|| DEFAULT_COLOR.into())
    };

    let theme = move || {
        let c = color();
        let b = button_type();
        b.get(&c)
            .unwrap_or_else(|| {
                log::warn!("color {c} not found");
                &b[DEFAULT_COLOR]
            })
            .fields_to_string()
    };

    let size = move || {
        size.as_ref()
            .map(|s| s())
            .unwrap_or_else(|| DEFAULT_SIZE.into())
    };

    let size = move || {
        let s = size();
        SIZES
            .get(&s)
            .unwrap_or_else(|| {
                log::warn!("size {s} not found");
                &SIZES[DEFAULT_SIZE]
            })
            .fields_to_string()
    };

    let final_class = move || {
        format!("align-middle select-none font-sans font-bold text-center uppercase transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none {} {}", theme(), size())
    };

    let reference = create_node_ref::<html::Button>();

    let view = view! {
      <button node_ref=reference class=final_class>

        {children()}
      </button>
    };

    if let Some(r) = node_ref {
        reference.get_untracked().expect("ok").node_ref(r);
    }

    view
}

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

use super::{ButtonFilled, ButtonGradient, ButtonOutlined, ButtonSizes::get_size, ButtonText};
use crate::{tailwind_merge, JoinFields};

#[derive(Default)]
pub(crate) struct Theme {
    pub(crate) background: &'static str,
    pub(crate) border: &'static str,
    pub(crate) color: &'static str,
    pub(crate) shadow: &'static str,
    pub(crate) hover: &'static str,
    pub(crate) focus: &'static str,
    pub(crate) active: &'static str,
}

impl JoinFields for Theme {
    fn join_fields(&self) -> String {
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

#[derive(Clone, Copy)]
pub enum ButtonVariants {
    Filled,
    Gradient,
    Outlined,
    Text,
}

#[derive(Clone, Copy)]
pub enum ButtonColors {
    White,
    BlueGray,
    Gray,
    Brown,
    DeepOrange,
    Orange,
    Amber,
    Yellow,
    Lime,
    LightGreen,
    Green,
    Teal,
    Cyan,
    LightBlue,
    Blue,
    Indigo,
    DeepPurple,
    Purple,
    Pink,
    Red,
}

#[derive(Clone, Copy)]
pub enum ButtonSizes {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<html::Button>>,
    #[prop(optional, into)] variant: Option<MaybeSignal<ButtonVariants>>,
    #[prop(optional, into)] color: Option<MaybeSignal<ButtonColors>>,
    #[prop(optional, into)] size: Option<MaybeSignal<ButtonSizes>>,
    #[prop(optional, into)] class: Option<MaybeSignal<String>>,
) -> impl IntoView {
    let theme = move || {
        let theme_getter = match variant {
            Some(v) => match v() {
                ButtonVariants::Filled => ButtonFilled::get_theme,
                ButtonVariants::Gradient => ButtonGradient::get_theme,
                ButtonVariants::Outlined => ButtonOutlined::get_theme,
                ButtonVariants::Text => ButtonText::get_theme,
            },
            None => ButtonFilled::get_theme,
        };

        match color {
            Some(c) => theme_getter(c()),
            None => theme_getter(ButtonColors::Blue),
        }
        .join_fields()
    };

    let size = move || {
        match size {
            Some(s) => get_size(s()),
            None => get_size(ButtonSizes::Md),
        }
        .join_fields()
    };

    let final_class = move || {
        let build = format!("align-middle select-none font-sans font-bold text-center uppercase transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none {} {}", theme(), size());

        match class.as_ref() {
            Some(c) => tailwind_merge(build, &c()),
            None => build,
        }
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

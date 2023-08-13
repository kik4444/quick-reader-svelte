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

// Credit https://github.com/sajadevo/material-ripple-effects/blob/main/ripple.js

use std::time::Duration;

use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{CssStyleDeclaration, DomRect, HtmlButtonElement, HtmlSpanElement, MouseEvent};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonRipple {
    #[default]
    Light,
    Dark,
    None,
}

fn find_furthest_point(
    click_point_x: f64,
    element_width: f64,
    offset_x: f64,
    click_point_y: f64,
    element_height: f64,
    offset_y: f64,
) -> f64 {
    let x = if click_point_x - offset_x > element_width / 2.0 {
        0.0
    } else {
        element_width
    };

    let y = if click_point_y - offset_y > element_height / 2.0 {
        0.0
    } else {
        element_height
    };

    (x - (click_point_x - offset_x)).hypot(y - (click_point_y - offset_y))
}

fn apply_styles(
    circle_style: CssStyleDeclaration,
    ripple: ButtonRipple,
    rect: DomRect,
    radius: f64,
    event: &MouseEvent,
) {
    _ = circle_style.set_property(
        "background-color",
        match ripple {
            ButtonRipple::Light => "rgba(255,255,255, 0.3)",
            ButtonRipple::Dark => "rgba(0,0,0, 0.2)",
            ButtonRipple::None => unreachable!(),
        },
    );
    _ = circle_style.set_property("border-radius", "50%");
    _ = circle_style.set_property("pointer-events", "none");
    _ = circle_style.set_property("position", "absolute");
    _ = circle_style.set_property(
        "left",
        &format!("{}px", event.client_x() as f64 - rect.left() - radius),
    );
    _ = circle_style.set_property(
        "top",
        &format!("{}px", event.client_y() as f64 - rect.top() - radius),
    );

    let width_height = &format!("{}px", radius * 2.0);
    _ = circle_style.set_property("width", width_height);
    _ = circle_style.set_property("height", width_height);
}

// We use JS to animate the ripple because the Animation feature in web-sys requires web_sys_unstable_apis
#[wasm_bindgen(inline_js = r#"
export function animate_ripple(span_element, duration) {
    span_element.animate(
        [
            {
                transform: "scale(0)",
                opacity: 1,
            },
            {
                transform: "scale(1.5)",
                opacity: 0,
            },
        ],
        {
            duration,
            easing: "linear",
        },
    );
}
"#)]
extern "C" {
    fn animate_ripple(span_element: &HtmlSpanElement, duration: f64);
}

#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<html::Button>>,
    #[prop(optional, into)] disabled: Option<MaybeSignal<bool>>,
    #[prop(optional, into)] class: Option<MaybeSignal<String>>,
    #[prop(optional, into)] ripple: Option<MaybeSignal<ButtonRipple>>,
    #[prop(optional, into)] ripple_duration: Option<MaybeSignal<u64>>,
) -> impl IntoView {
    let create_ripple = move |ev: MouseEvent| {
        let ripple = ripple.map(|r| r()).unwrap_or_default();

        if ripple == ButtonRipple::None {
            return;
        }

        let target = ev.target().expect("ok");
        let button = target.dyn_ref::<HtmlButtonElement>().expect("ok");

        let style = button.style();
        _ = style.set_property("position", "relative");
        _ = style.set_property("overflow", "hidden");

        let rect = button.get_bounding_client_rect();

        let radius = find_furthest_point(
            ev.client_x() as f64,
            button.offset_width() as f64,
            rect.left(),
            ev.client_y() as f64,
            button.offset_height() as f64,
            rect.top(),
        );

        let element = document().create_element("span").expect("ok");
        let circle = element.dyn_into::<HtmlSpanElement>().expect("ok");
        let circle_style = circle.style();

        apply_styles(circle_style, ripple, rect, radius, &ev);

        let duration = ripple_duration.map(|d| d()).unwrap_or(500);
        animate_ripple(&circle, duration as f64);

        _ = button.append_child(&circle);
        set_timeout(move || circle.remove(), Duration::from_millis(duration));
    };

    let reference = create_node_ref::<html::Button>();
    let view = view! {
      <button
        class=move || class.as_ref().map(|c| c()).unwrap_or_default()
        on:mouseup=create_ripple
        disabled=move || disabled.map(|d| d()).unwrap_or_default()
      >
        {children()}
      </button>
    };

    if let Some(r) = node_ref {
        reference.get_untracked().expect("ok").node_ref(r);
    }

    view
}

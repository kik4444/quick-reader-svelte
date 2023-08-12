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
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsCast;
use web_sys::{
    CssStyleDeclaration, DomRect, HtmlButtonElement, HtmlSpanElement, KeyframeAnimationOptions,
    MouseEvent,
};

fn find_furthest_point(
    click_point_x: f64,
    element_width: f64,
    offset_x: f64,
    click_point_y: f64,
    element_height: f64,
    offset_y: f64,
) -> f64 {
    let x = click_point_x
        - if offset_x > (element_width / 2.0) {
            0.0
        } else {
            element_width
        };

    let y = click_point_y
        - if offset_y > (element_height / 2.0) {
            0.0
        } else {
            element_height
        };

    js_sys::Math::hypot(
        x - (click_point_x - offset_x),
        y - (click_point_y - offset_y),
    )
}

fn apply_styles(
    circle_style: CssStyleDeclaration,
    color: &str,
    rect: DomRect,
    radius: f64,
    event: MouseEvent,
) {
    _ = circle_style.set_property("background-color", "rgba(255,255,255, 0.3)");
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

#[derive(Serialize)]
struct AnimationTransformation {
    transform: &'static str,
    opacity: f64,
}

#[cfg(web_sys_unstable_apis)]
fn apply_animation(circle: &HtmlSpanElement) {
    let transformation = js_sys::Object::from(
        to_value(&[
            AnimationTransformation {
                transform: "scale(0)",
                opacity: 1.0,
            },
            AnimationTransformation {
                transform: "scale(1.5)",
                opacity: 0.0,
            },
        ])
        .expect("ok"),
    );

    let mut keyframe = KeyframeAnimationOptions::new();
    keyframe.duration(&to_value(&500).expect("ok"));
    keyframe.easing("linear");

    circle.animate_with_keyframe_animation_options(Some(&transformation), &keyframe);
}

#[component]
pub fn Button(#[prop(default = true)] ripple: bool) -> impl IntoView {
    let create_ripple = move |ev: MouseEvent| {
        if !ripple {
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

        apply_styles(circle_style, "dark", rect, radius, ev);
        apply_animation(&circle);

        _ = button.append_child(&circle);
        set_timeout(move || circle.remove(), Duration::from_millis(500));
    };

    view! {
      <button
        class="rounded-lg bg-blue-500 py-3 px-6 font-sans text-sm font-bold uppercase text-white shadow-md shadow-blue-500/20 transition-all hover:shadow-lg hover:shadow-blue-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
        on:mouseup=create_ripple
      >
        "Hello"
      </button>
    }
}

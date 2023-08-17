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

use common::AppSettings;
use leptos::*;
use leptos_router::*;

use crate::app::{FontError, Fonts};

#[component]
pub fn FontChooser() -> impl IntoView {
    let settings = expect_context::<RwSignal<AppSettings>>();
    let fonts = expect_context::<Resource<(), Result<Fonts, FontError>>>();

    let query = use_query_map();

    let actions = move || {
        query.with(|q| match q.get("choice") {
            Some(choice) => match choice.as_str() {
                "display" => Ok((
                    create_rw_signal(settings.with(|s| s.display_font_style.clone())),
                    Box::new(move |style| settings.update(|s| s.display_font_style = style))
                        as Box<dyn Fn(String)>,
                )),
                "textarea" => Ok((
                    create_rw_signal(settings.with(|s| s.textarea_font_style.clone())),
                    Box::new(move |style| settings.update(|s| s.textarea_font_style = style))
                        as Box<dyn Fn(String)>,
                )),
                s => Err(FontError(format!("invalid font choice {s}"))),
            },
            None => Err(FontError("no font chosen to change".into())),
        })
    };

    let label_text = move || query.with(|q| q.get("choice").cloned());

    view! {
      <div class="w-full h-full pt-10 px-5 grid grid-rows-3 gap-5 place-items-center">
        {move || match actions() {
            Err(e) => view! { <p class="paragraph m-5">{e.to_string()}</p> }.into_view(),
            Ok((current_font, setter)) => {
                match fonts.read() {
                    None => view! { <p class="paragraph m-5">"Loading fonts"</p> }.into_view(),
                    Some(fonts) => {
                        match fonts {
                            Err(e) => {
                                view! {
                                  <p class="paragraph m-5">
                                    "Error loading fonts: " {e.to_string()}
                                  </p>
                                }
                                    .into_view()
                            }
                            Ok(fonts) => {
                                view! {
                                  <div class="self-start relative h-fit w-full">
                                    <select
                                      class="peer select"
                                      on:input=move |ev| setter(event_target_value(&ev))
                                    >
                                      <For
                                        each=move || fonts.clone()
                                        key=|font| font.clone()
                                        view=move |font| {
                                            view! {
                                              <option selected={
                                                  let font = font.clone();
                                                  move || font == current_font()
                                              }>{font}</option>
                                            }
                                        }
                                      />

                                    </select>
                                    <label class="label">{label_text}</label>
                                  </div>

                                  <p
                                    class="paragraph self-center text-3xl"
                                    style:font-family=current_font
                                  >

                                    "The quick brown fox jumps over the lazy dog"
                                  </p>
                                }
                                    .into_view()
                            }
                        }
                    }
                }
            }
        }}

      </div>
    }
}

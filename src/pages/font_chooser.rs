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

use std::rc::Rc;

use common::AppSettings;
use leptos::*;
use leptos_router::*;

use crate::{IntoValue, ToJsValue};

#[derive(Debug, thiserror::Error, Clone)]
#[error("{0}")]
struct FontError(String);

struct Action {
    getter: Box<dyn Fn() -> String>,
    setter: Box<dyn Fn(String)>,
}

async fn get_fonts() -> Result<Vec<String>, FontError> {
    #[cfg(feature = "tauri")]
    {
        use crate::js_bindings::invoke;

        match invoke("get_system_fonts", ().to_js_value().expect("ok")).await {
            Ok(js_value) => Ok(js_value.into_value::<Vec<String>>().expect("ok")),
            Err(e) => Err(FontError(e.as_string().expect("to be string"))),
        }
    }

    #[cfg(not(feature = "tauri"))]
    {
        Err(FontError(
            "choosing fonts is not supported on the web".into(),
        ))
    }
}

#[component]
pub fn FontChooser() -> impl IntoView {
    let settings = expect_context::<RwSignal<AppSettings>>();
    let query = use_query_map();

    let fonts = create_local_resource(|| (), |_| async move { get_fonts().await });

    let action = move || {
        query.with(|q| match q.get("choice") {
            None => Err(FontError("no font chosen to change".into())),
            Some(choice) => match choice.as_str() {
                "display" => Ok(Rc::new(Action {
                    getter: Box::new(move || settings.with(|s| s.display_font_style.clone())),
                    setter: Box::new(move |style| {
                        settings.update(|s| s.display_font_style = style)
                    }),
                })),
                "textarea" => Ok(Rc::new(Action {
                    getter: Box::new(move || settings.with(|s| s.textarea_font_style.clone())),
                    setter: Box::new(move |style| {
                        settings.update(|s| s.textarea_font_style = style)
                    }),
                })),
                s => Err(FontError(format!("invalid font choice {s}"))),
            },
        })
    };

    let label_text = move || query.with(|q| q.get("choice").cloned());

    view! {
      <div class="w-full h-full pt-10 px-5 grid grid-rows-3 gap-5 place-items-center">
        {move || match action() {
            Err(e) => view! { <p class="paragraph m-5">{e.to_string()}</p> }.into_view(),
            Ok(action) => {
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
                                      // We do this Rc magic because closures are not Clone or Copy
                                      prop:value={
                                          let action = Rc::clone(&action);
                                          move || (action.getter)()
                                      }

                                      on:input={
                                          let action = Rc::clone(&action);
                                          move |ev| (action.setter)(event_target_value(&ev))
                                      }
                                    >

                                      <For
                                        each=move || fonts.clone()
                                        key=|font| font.clone()
                                        view=move |font| view! { <option>{font}</option> }
                                      />
                                    </select>
                                    <label class="label">{label_text}</label>
                                  </div>

                                  <p
                                    class="paragraph self-center text-3xl"
                                    style={
                                        let action = Rc::clone(&action);
                                        move || { format!("font-family: {}", (action.getter) ()) }
                                    }
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

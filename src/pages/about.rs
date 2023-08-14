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
use wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, MouseEvent};

#[component]
pub fn About() -> impl IntoView {
    const SOURCE_CODE_LINK: &str = "https://github.com/kik4444/quick-reader/tree/leptos";
    const LICENSE_LINK: &str = "https://www.gnu.org/licenses";

    let clicked_link = move |ev: MouseEvent| {
        #[cfg(feature = "tauri")]
        {
            use crate::js_bindings::open;

            ev.prevent_default();
            let link = ev
                .target()
                .expect("ok")
                .dyn_ref::<HtmlAnchorElement>()
                .expect("ok")
                .href();

            spawn_local(async move {
                open(&link).await;
            });
        }
    };

    view! {
      <div class="flex flex-col gap-5 mt-10 mx-5 items-center ">
        <img class="h-[20%]" src="/public/quickreader-icon.svg"/>
        <p class="paragraph">
          "Quick Reader is a program that enables its user to quickly read through text
          by encouraging the silencing of their subvocalization. Quick Reader is
          free/libre software released under the GPL version 3. The source code is
          available at "
          <a class="underline text-blue-500" href=SOURCE_CODE_LINK on:click=clicked_link>
            {SOURCE_CODE_LINK}
          </a>
        </p>
        <p class="paragraph">
          "You should have received a copy of the GNU General Public License along with
          this program. If not, see "
          <a class="underline text-blue-500" href=LICENSE_LINK on:click=clicked_link>
            {LICENSE_LINK}
          </a>
        </p>
      </div>
    }
}

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

use leptos::{html::Textarea, *};

// #[cfg(feature = "web")]
// async fn get_data() -> String {
//     "From web".into()
// }

// #[cfg(not(feature = "web"))]
// async fn get_data() -> String {
//     #[derive(Serialize, Deserialize)]
//     struct GreetArgs<'a> {
//         name: &'a str,
//     }

//     invoke("greet", GreetArgs { name: "Name" }.to_js_value().unwrap())
//         .await
//         .as_string()
//         .unwrap()
// }

#[component]
pub fn App() -> impl IntoView {
    let area = create_node_ref::<Textarea>();

    view! {
      <textarea cols="30" rows="10" node_ref=area></textarea>
      <button on:click=move |_| {
          area().unwrap().set_selection_range(0, 5).unwrap();
      }>"click me"</button>
    }
}

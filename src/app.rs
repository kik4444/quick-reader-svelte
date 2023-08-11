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
use leptos_router::*;

use crate::pages::quick_reader::QuickReader;

pub struct ReaderState {
    pub text: String,
    pub chunk_size: usize,
    pub current_index: usize,
    pub words_per_minute: usize,
}

#[component]
pub fn App() -> impl IntoView {
    // Global state for the reader which should persist across page navigation
    provide_context(create_rw_signal(ReaderState {
        text: r#"Welcome to "Quick Reader". Press start to begin reading quickly."#.into(),
        chunk_size: 1,
        current_index: 0,
        words_per_minute: 300,
    }));

    view! {
      <Router>

        <main class="h-screen pt-5 grid grid-rows-[5%_90%] place-items-center">

          <nav class="w-full grid grid-cols-3 px-5 gap-5">

            {[("/settings", "Settings"), ("/", "Quick Reader"), ("/about", "About")]
                .map(|(href, text)| {
                    view! {
                      <a href=href>
                        <button
                          class="w-full rounded-lg bg-blue-500 py-2 px-4 font-sans text-xs font-bold uppercase text-white shadow-md shadow-blue-500/20 transition-all hover:shadow-lg hover:shadow-blue-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
                          data-ripple-light="true"
                        >
                          {text}
                        </button>
                      </a>
                    }
                })}

          </nav>

          <Routes>
            <Route path="/" view=QuickReader/>
          </Routes>
        </main>
      </Router>
    }
}

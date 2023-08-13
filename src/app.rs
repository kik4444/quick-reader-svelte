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

use crate::{
    components::{Button::Button, MatIcon::MatIcon},
    pages::quick_reader::QuickReader,
};

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

        <main class="h-screen dark:bg-gray-900 pt-5 grid grid-rows-[5%_90%]">

          <nav class="w-full grid grid-cols-3 px-5 gap-5">

            {[
                ("/settings", "settings", "Settings"),
                ("/", "speed", "Quick Reader"),
                ("/about", "help", "About"),
            ]
                .map(|(href, icon, text)| {
                    view! {
                      <a href=href>
                        <Button class="btn-main w-full">
                          <MatIcon>{icon}</MatIcon>
                          {text}
                        </Button>
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

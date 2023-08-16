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

use crate::{
    components::{Button::Button, MatIcon::MatIcon},
    pages::{
        about::About, font_chooser::FontChooser, quick_reader::QuickReader, settings::Settings,
    },
};

pub struct ReaderState {
    pub text: String,
    pub chunk_size: usize,
    pub current_index: usize,
    pub words_per_minute: usize,
}

async fn load_settings() -> Result<AppSettings, Box<dyn std::error::Error>> {
    #[cfg(feature = "tauri")]
    {
        // todo!()
        Ok(AppSettings::default())
    }

    #[cfg(not(feature = "tauri"))]
    {
        let storage = window()
            .local_storage()
            .map_err(|_| "error getting storage".to_string())?
            .ok_or_else(|| "error getting storage".to_string())?;

        match storage.get_item("app_settings") {
            Ok(Some(app_settings)) => Ok(serde_json::from_str(&app_settings)?),
            res => {
                log::warn!("Found {res:?}, expected app settings. Returning default");
                Ok(AppSettings::default())
            }
        }
    }
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

    let provide_settings = create_action(|()| async move {
        provide_context(create_rw_signal(load_settings().await.unwrap_or_default()))
    });
    provide_settings.dispatch(());

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
            <Route
              path="/"
              view=move || {
                  view! {
                    <Show
                      fallback=|| view! { <p class="paragraph m-5">"Loading"</p> }
                      when=move || provide_settings.version().with(|v| *v > 0)
                    >
                      <Outlet/>
                    </Show>
                  }
              }
            >

              <Route path="" view=QuickReader/>
              <Route path="settings" view=Settings/>
              <Route path="about" view=About/>
              <Route path="font-chooser" view=FontChooser/>

            </Route>

          </Routes>

        </main>
      </Router>
    }
}

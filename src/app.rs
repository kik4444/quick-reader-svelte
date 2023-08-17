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

use common::{AppSettings, Theme};
use leptos::*;
use leptos_router::*;

use crate::{
    components::{Button::Button, MatIcon::MatIcon},
    pages::{
        about::About, font_chooser::FontChooser, quick_reader::QuickReader, settings::Settings,
    },
};

#[cfg(feature = "tauri")]
use crate::{IntoValue, ToJsValue};

pub struct ReaderState {
    pub text: String,
    pub chunk_size: usize,
    pub current_index: usize,
    pub words_per_minute: usize,
}

#[derive(Debug, thiserror::Error, Clone)]
#[error("{0}")]
pub struct FontError(pub String);

pub type Fonts = Vec<String>;

async fn load_settings() -> Result<AppSettings, Box<dyn std::error::Error>> {
    #[cfg(feature = "tauri")]
    {
        use crate::js_bindings::invoke;

        match invoke("load_settings", ().to_js_value().expect("ok")).await {
            Ok(js_value) => Ok(js_value.into_value::<AppSettings>().expect("ok")),
            Err(e) => Err(e.as_string().expect("to be string").into()),
        }
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

async fn save_settings(settings_serialized: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "tauri")]
    {
        use crate::js_bindings::invoke;

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SaveArgs<'a> {
            settings_serialized: &'a str,
        }

        _ = invoke(
            "save_settings",
            SaveArgs {
                settings_serialized,
            }
            .to_js_value()
            .expect("ok"),
        )
        .await;
    }

    #[cfg(not(feature = "tauri"))]
    {
        let storage = window()
            .local_storage()
            .map_err(|_| "error getting storage".to_string())?
            .ok_or_else(|| "error getting storage".to_string())?;

        storage
            .set_item(
                "app_settings",
                &serde_json::to_string(&settings_serialized).expect("ok"),
            )
            .map_err(|_| "error saving settings".to_string())?;
    }

    Ok(())
}

async fn get_fonts() -> Result<Fonts, FontError> {
    #[cfg(feature = "tauri")]
    {
        use crate::js_bindings::invoke;

        match invoke("get_system_fonts", ().to_js_value().expect("ok")).await {
            Ok(js_value) => Ok(js_value.into_value::<Fonts>().expect("ok")),
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

    // Start loading the fonts once the app starts to avoid showing a loading screen in the font chooser page
    let fonts = create_local_resource(|| (), |_| async move { get_fonts().await });
    provide_context(fonts);

    create_effect(move |_| {
        if provide_settings.version().with(|v| *v > 0) {
            let settings = expect_context::<RwSignal<AppSettings>>();
            let serialized = settings.with(serde_json::to_string_pretty).expect("ok");
            spawn_local(async move {
                if let Err(e) = save_settings(&serialized).await {
                    log::error!("{e:#?}")
                }
            });
        }
    });

    let (dark_mode, _) = create_signal(move || {
        if provide_settings.version().with(|v| *v > 0) {
            let settings = expect_context::<RwSignal<AppSettings>>();
            match settings.with(|s| s.theme) {
                Theme::Auto => match window().match_media("(prefers-color-scheme: dark)") {
                    Ok(Some(media_query_list)) => media_query_list.matches(),
                    Err(e) => {
                        log::debug!(
                            "error querying prefers-color-scheme: {}",
                            e.as_string().expect("to be string")
                        );
                        false
                    }
                    _ => false,
                },
                Theme::Dark => true,
                Theme::Light => false,
            }
        } else {
            false
        }
    });

    view! {
      <Router>
        <main class:dark=move || dark_mode()()>
          <div class="h-screen dark:bg-gray-900 pt-5 grid grid-rows-[5%_90%]">
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
          </div>
        </main>
      </Router>
    }
}

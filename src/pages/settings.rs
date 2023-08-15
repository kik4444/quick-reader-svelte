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

use crate::components::Button::Button;

async fn save_settings(settings: &AppSettings) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "tauri")]
    {
        todo!()
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
                &serde_json::to_string(settings).expect("ok"),
            )
            .map_err(|_| "error saving settings".to_string())?;
    }

    Ok(())
}

#[component]
pub fn Settings() -> impl IntoView {
    let settings = expect_context::<RwSignal<AppSettings>>();

    let display_font = create_read_slice(settings, |s| s.display_font_style.clone());
    let textarea_font = create_read_slice(settings, |s| s.textarea_font_style.clone());

    on_cleanup(move || {
        spawn_local(async move {
            if let Err(e) = save_settings(&settings.get_untracked()).await {
                log::error!("{e:#?}")
            }
        })
    });

    view! {
      <main class="flex flex-col gap-5 mt-5 mx-5">

        <div class="flex gap-5 place-items-center">
          <Button class="btn-main">"Choose display font family"</Button>
          <p class="paragraph">{display_font}</p>
        </div>

        <div class="w-fit relative h-10">
          <input type="number" class="peer input" placeholder=" " min="1"/>
          <label class="label">"Display font size"</label>
        </div>

        <div class="flex gap-5 place-items-center">
          <Button class="btn-main">"Choose textarea font family"</Button>
          <p class="paragraph">{textarea_font}</p>
        </div>

        {[("Textarea font size"), ("Jump back chunks"), ("Jump forward chunks")]
            .map(|name| {
                view! {
                  <div class="w-fit relative h-10">
                    <input type="number" class="peer input" placeholder=" " min="1"/>
                    <label class="label">{name}</label>
                  </div>
                }
            })}

      </main>
    }
}

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

async fn save_settings(settings_serialized: &str) -> Result<(), Box<dyn std::error::Error>> {
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
            .set_item("app_settings", settings_serialized)
            .map_err(|_| "error saving settings".to_string())?;
    }

    Ok(())
}

#[component]
pub fn Settings() -> impl IntoView {
    let settings = expect_context::<RwSignal<AppSettings>>();

    let display_font = create_read_slice(settings, |s| s.display_font_style.clone());
    let textarea_font = create_read_slice(settings, |s| s.textarea_font_style.clone());

    let (display_font_size, set_display_font_size) = create_slice(
        settings,
        |s| s.display_font_size,
        |s, new| s.display_font_size = new,
    );

    let (textarea_font_size, set_textarea_font_size) = create_slice(
        settings,
        |s| s.textarea_font_size,
        |s, new| s.textarea_font_size = new,
    );

    let (jump_back_chunks, set_jump_back_chunks) = create_slice(
        settings,
        |s| s.jump_back_chunks,
        |s, new| s.jump_back_chunks = new,
    );

    let (jump_forward_chunks, set_jump_forward_chunks) = create_slice(
        settings,
        |s| s.jump_forward_chunks,
        |s, new| s.jump_forward_chunks = new,
    );

    create_effect(move |_| {
        let serialized = settings.with(serde_json::to_string).expect("ok");
        spawn_local(async move {
            if let Err(e) = save_settings(&serialized).await {
                log::error!("{e:#?}")
            }
        });
    });

    view! {
      <main class="flex flex-col gap-5 mt-5 mx-5">

        <div class="flex gap-5 place-items-center">

          <a href="/font-chooser?choice=display">
            <Button class="btn-main">"Choose display font family"</Button>
          </a>
          <p class="paragraph">{display_font}</p>
        </div>

        <div class="w-fit relative h-10">
          <input
            type="number"
            class="peer input"
            placeholder=" "
            min="1"
            prop:value=display_font_size
            on:input=move |ev| set_display_font_size(event_target_value(&ev).parse().unwrap_or(35))
          />
          <label class="label">"Display font size"</label>
        </div>

        <div class="flex gap-5 place-items-center">
          <a href="/font-chooser?choice=textarea">
            <Button class="btn-main">"Choose textarea font family"</Button>
          </a>
          <p class="paragraph">{textarea_font}</p>
        </div>

        {[
            ("Textarea font size", textarea_font_size, set_textarea_font_size, 12),
            ("Jump back chunks", jump_back_chunks, set_jump_back_chunks, 5),
            ("Jump forward chunks", jump_forward_chunks, set_jump_forward_chunks, 5),
        ]
            .map(|(name, value, setter, default)| {
                view! {
                  <div class="w-fit relative h-10">
                    <input
                      type="number"
                      class="peer input"
                      placeholder=" "
                      min="1"
                      prop:value=value
                      on:input=move |ev| setter(event_target_value(&ev).parse().unwrap_or(default))
                    />
                    <label class="label">{name}</label>
                  </div>
                }
            })}

      </main>
    }
}

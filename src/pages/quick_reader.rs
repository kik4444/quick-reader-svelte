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

use std::time::Duration;

use leptos::{
    leptos_dom::helpers::{debounce, IntervalHandle},
    *,
};
use wasm_bindgen::JsValue;

use crate::{
    app::ReaderState,
    components::{Button::Button, MatIcon::MatIcon},
    splitter,
};

#[component]
pub fn QuickReader() -> impl IntoView {
    let reader_state = expect_context::<RwSignal<ReaderState>>();

    let playing = create_rw_signal(false);

    let (text, set_text) = create_slice(reader_state, |s| s.text.clone(), |s, new| s.text = new);

    let (chunk_size, set_chunk_size) = create_slice(
        reader_state,
        |s| s.chunk_size,
        |s, new_chunk_size| {
            // Recalculate what the new chunk index should be after recreating the text chunks with a different size
            s.current_index = (s.current_index * s.chunk_size) / new_chunk_size;

            s.chunk_size = new_chunk_size;
        },
    );

    let (current_index, set_current_index) = create_slice(
        reader_state,
        |s| s.current_index,
        |s, new| s.current_index = new,
    );

    let (words_per_minute, set_words_per_minute) = create_slice(
        reader_state,
        |s| s.words_per_minute,
        |s, new| {
            if new > 1000 {
                s.words_per_minute = 1000
            } else if new < 100 {
                s.words_per_minute = 100
            } else {
                s.words_per_minute = new
            }
        },
    );

    let text_chunks = create_memo(move |_| splitter::split(&text(), chunk_size()));

    let textarea = create_node_ref::<html::Textarea>();
    let textarea_locked = create_rw_signal(false);

    let reset = move || {
        set_words_per_minute(300);
        set_chunk_size(1);
    };

    let stop = move || {
        playing.set(false);
        textarea_locked.set(false);
        set_current_index(0);

        if let Ok(Some(selection)) = window().get_selection() {
            _ = selection.empty();
        }

        // We must unfocus the textfield after stopping play to avoid the user accidentally
        // deleting text if they press space after playing has finished
        _ = textarea().expect("to exist").blur();
    };

    let restart = move || {
        set_current_index(0);
        _ = textarea().expect("to exist").focus();
    };

    let toggle_playing = move || {
        if playing() {
            playing.set(false);
        } else {
            playing.set(true);
            textarea_locked.set(true);

            // On Windows we must focus the textfield otherwise the highlight is not visible
            _ = textarea().expect("to exist").focus();
        }
    };

    let step = move || {
        if playing() {
            if current_index() < text_chunks.with(|t| t.len() - 1) {
                set_current_index(current_index() + 1);
            } else {
                stop()
            }

            let (start, stop) = text_chunks.with(|t| {
                let curr = current_index();
                (t[curr].start_offset, t[curr].stop_offset)
            });

            _ = textarea()
                .expect("to exist")
                .set_selection_range(start as u32, stop as u32);
        }
    };

    let speed = create_memo(move |_| (1000 / (words_per_minute() / 60)) * chunk_size());

    let (duration, _) = create_signal(move || {
        let chunk_size = chunk_size();

        let duration_seconds =
            (speed() * chunk_size * (text_chunks.with(|t| t.len()) - current_index()))
                / 1000
                / chunk_size;

        format!(
            "{}m {}s",
            duration_seconds % 3600 / 60,
            duration_seconds % 60
        )
    });

    create_effect(move |prev: Option<Result<IntervalHandle, JsValue>>| {
        if let Some(Ok(interval)) = prev {
            interval.clear();
        }

        let interval = set_interval_with_handle(step, Duration::from_millis(speed() as u64));

        // Remove the interval when this page is destroyed otherwise it'll continue running and trying to access inaccessible signals and spam the console with panics
        if let Ok(interval) = interval {
            on_cleanup(move || interval.clear());
        }

        interval
    });

    let handle = window_event_listener(ev::keydown, move |ev| match ev.code().as_str() {
        "ArrowLeft" => set_current_index(current_index().saturating_sub(5)),
        "ArrowRight" => set_current_index(std::cmp::min(
            current_index() + 5,
            text_chunks.with(|t| t.len() - 1),
        )),
        "Space" => {
            if textarea_locked()
                || document().active_element().expect("ok") != ***textarea().expect("ok")
            {
                ev.prevent_default();
                toggle_playing();
            }
        }
        _ => {}
    });

    on_cleanup(move || handle.remove());

    view! {
      <main class="w-full h-full grid grid-rows-3 place-items-center">

        <div
          id="textarea"
          class="relative w-full h-full min-w-[200px] pt-5 px-5 [&>label]:pt-5 [&>label]:px-5"
        >
          <textarea
            class="peer textarea"
            placeholder=" "
            node_ref=textarea
            readonly=textarea_locked
            on:input=move |ev| set_text(event_target_value(&ev))
          >
            {text}
          </textarea>
          <label class="label">"Quick Reader"</label>
        </div>

        <p
          id="display"
          class="paragraph block font-sans text-4xl font-semibold leading-[1.3] tracking-normal antialiased"
        >
          {move || text_chunks.with(|t| t[current_index()].chunk.clone())}
        </p>

        <div id="controls" class="w-full flex flex-col gap-5 items-center">

          <div id="chunking" class="w-full flex gap-5 place-content-center">

            {
            [
                (
                    "Words per minute",
                    "100",
                    "1000",
                    "50",
                    words_per_minute,
                    Box::new(move |ev| set_words_per_minute(
                        event_target_value(&ev).parse().unwrap_or(300),
                    )) as Box<dyn Fn(_)>,
                ),
                (
                    "Chunk size",
                    "1",
                    "10",
                    "1",
                    chunk_size,
                    Box::new(move |ev| set_chunk_size(event_target_value(&ev).parse().unwrap_or(1))),
                ),
            ]
                .map(|(name, min, max, step, value, on_input)| {
                    view! {
                      <div class="w-32">
                        <div class="relative h-10 w-full">
                          <input
                            class="peer input"
                            placeholder=" "
                            type="number"
                            min=min
                            max=max
                            step=step
                            prop:value=value
                            on:input=debounce(Duration::from_millis(500), on_input)
                          />
                          <label class="label">{name}</label>
                        </div>
                      </div>
                    }
                })}

            <Button class="btn-main" on:click=move |_| reset()>
              <MatIcon>"loop"</MatIcon>
              "Reset"
            </Button>

          </div>

          <div id="progress" class="w-full flex gap-3 px-5">
            <p class="paragraph">
              "Chunk " {move || current_index() + 1} " of " {move || text_chunks.with(|t| t.len())}
            </p>
            <div class="w-[1px] h-[30px] self-center bg-black"></div>
            <p class="paragraph">"Duration: " {duration}</p>
            <div class="w-[1px] h-[30px] self-center bg-black"></div>
            <input
              type="range"
              min="0"
              max=move || text_chunks.with(|t| t.len() - 1)
              prop:value=current_index
              on:input=move |ev| set_current_index(
                  event_target_value(&ev).parse().unwrap_or_default(),
              )

              class="flex-grow-[2]"
            />
          </div>

          <div id="playback" class="w-full grid grid-cols-3 px-5 gap-5">

            {
            [
                (
                    Box::new(|| "Stop") as Box<dyn Fn() -> &'static str>,
                    Box::new(|| "stop") as Box<dyn Fn() -> &'static str>,
                    Box::new(move |_| stop()) as Box<dyn Fn(_)>,
                    Box::new(move || !textarea_locked()) as Box<dyn Fn() -> bool>,
                ),
                (
                    Box::new(|| "Restart"),
                    Box::new(|| "restart_alt"),
                    Box::new(move |_| restart()),
                    Box::new(move || !textarea_locked()),
                ),
                (
                    Box::new(move || if playing() { "Pause" } else { "Play" }),
                    Box::new(move || if playing() { "pause" } else { "play_arrow" }),
                    Box::new(move |_| toggle_playing()),
                    Box::new(|| false),
                ),
            ]
                .map(|(text, icon, on_click, disabled)| {
                    view! {
                      <Button class="btn-main" disabled=disabled.derive_signal() on:click=on_click>
                        <MatIcon>{icon}</MatIcon>
                        {text}
                      </Button>
                    }
                })}

          </div>

        </div>

      </main>
    }
}

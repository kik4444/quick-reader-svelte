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

use leptos::{html::Textarea, leptos_dom::helpers::IntervalHandle, *};
use wasm_bindgen::JsValue;

use crate::splitter;

#[derive(Debug)]
pub struct ReaderState {
    pub text: String,
    pub chunk_size: usize,
    pub current_index: usize,
    pub words_per_minute: usize,
}

#[component]
pub fn QuickReader() -> impl IntoView {
    let reader_state = create_rw_signal(ReaderState {
        text: r#"Welcome to "Quick Reader". Press start to begin reading quickly."#.into(),
        chunk_size: 1,
        current_index: 0,
        words_per_minute: 300,
    });

    let playing = create_rw_signal(false);

    let (text, set_text) = create_slice(
        reader_state,
        |s| s.text.clone(),
        move |s, new| {
            s.text = new;
            s.current_index = 0;
            playing.set(false);
        },
    );

    let (chunk_size, set_chunk_size) = create_slice(
        reader_state,
        |s| s.chunk_size,
        |s, new| {
            s.chunk_size = new;
            // TODO recalculate new index
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

    let textarea = create_node_ref::<Textarea>();
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

    create_effect(move |prev: Option<Result<IntervalHandle, JsValue>>| {
        if let Some(Ok(handle)) = prev {
            handle.clear();
        }

        set_interval_with_handle(step, Duration::from_millis(words_per_minute() as u64))
    });

    window_event_listener(ev::keydown, move |ev| match ev.code().as_str() {
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

    view! {
      <main class="w-full h-screen grid grid-rows-[35%_45%_20%] place-items-center">

        <div
          id="textarea"
          class="relative w-full h-full min-w-[200px] pt-5 px-5 [&>label]:pt-5 [&>label]:px-5"
        >
          <textarea
            class="peer h-full min-h-[100px] w-full resize-none rounded-[7px] border border-blue-gray-200 border-t-transparent bg-transparent px-3 py-2.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border placeholder-shown:border-blue-gray-200 placeholder-shown:border-t-blue-gray-200 focus:border-2 focus:border-red-500 focus:border-t-transparent focus:outline-0 disabled:resize-none disabled:border-0 disabled:bg-blue-gray-50"
            placeholder=" "
            node_ref=textarea
            on:input=move |ev| set_text(event_target_value(&ev))
          >
            {text}
          </textarea>
          <label class="before:content[' '] after:content[' '] pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-[11px] font-normal leading-tight text-blue-gray-400 transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-blue-gray-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-blue-gray-200 after:transition-all peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[3.75] peer-placeholder-shown:text-blue-gray-500 peer-placeholder-shown:before:border-transparent peer-placeholder-shown:after:border-transparent peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-red-500 peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:border-red-500 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:border-red-500 peer-disabled:text-transparent peer-disabled:before:border-transparent peer-disabled:after:border-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
            "Quick Reader"
          </label>
        </div>

        <p
          id="display"
          class="block font-sans text-4xl font-semibold leading-[1.3] tracking-normal text-inherit antialiased"
        >
          {move || text_chunks.with(|t| t[current_index()].chunk.clone())}
        </p>

        <div id="controls" class="w-full flex flex-col items-center gap-5">

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
                      <div class="w-72">
                        <div class="relative h-10 w-full min-w-[200px]">
                          <input
                            class="peer h-full w-full rounded-[7px] border border-blue-gray-200 border-t-transparent bg-transparent px-3 py-2.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border placeholder-shown:border-blue-gray-200 placeholder-shown:border-t-blue-gray-200 focus:border-2 focus:border-pink-500 focus:border-t-transparent focus:outline-0 disabled:border-0 disabled:bg-blue-gray-50"
                            placeholder=" "
                            type="number"
                            min=min
                            max=max
                            step=step
                            prop:value=value
                            on:input=on_input
                          />
                          <label class="before:content[' '] after:content[' '] pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-[11px] font-normal leading-tight text-blue-gray-400 transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-blue-gray-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-blue-gray-200 after:transition-all peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[3.75] peer-placeholder-shown:text-blue-gray-500 peer-placeholder-shown:before:border-transparent peer-placeholder-shown:after:border-transparent peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-pink-500 peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:border-pink-500 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:border-pink-500 peer-disabled:text-transparent peer-disabled:before:border-transparent peer-disabled:after:border-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
                            {name}
                          </label>
                        </div>
                      </div>
                    }
                })}

            <button
              class="middle none center rounded-lg bg-blue-500 py-3 px-6 font-sans text-xs font-bold uppercase text-white shadow-md shadow-blue-500/20 transition-all hover:shadow-lg hover:shadow-blue-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
              on:click=move |_| reset()
            >
              "Reset"
            </button>

          </div>

          <div id="playback" class="w-full grid grid-cols-3 px-5 gap-5">

            {
            [
                (
                    Box::new(|| "Stop") as Box<dyn Fn() -> &'static str>,
                    Box::new(move |_| stop()) as Box<dyn Fn(_)>,
                    Box::new(move || !textarea_locked()) as Box<dyn Fn() -> bool>,
                ),
                (
                    Box::new(|| "Restart"),
                    Box::new(move |_| restart()),
                    Box::new(move || !textarea_locked()),
                ),
                (
                    Box::new(move || if playing() { "Pause" } else { "Play" }),
                    Box::new(move |_| toggle_playing()),
                    Box::new(|| false),
                ),
            ]
                .map(|(text, on_click, disabled)| {
                    view! {
                      <button
                        class="middle none center rounded-lg bg-blue-500 py-3 px-6 font-sans text-xs font-bold uppercase text-white shadow-md shadow-blue-500/20 transition-all hover:shadow-lg hover:shadow-blue-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
                        disabled=disabled
                        on:click=on_click
                      >
                        {text}
                      </button>
                    }
                })}

          </div>

        </div>

      </main>
    }
}

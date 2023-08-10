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
    pub words_per_minute: u64,
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

        set_interval_with_handle(step, Duration::from_millis(words_per_minute()))
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
        _ => (),
    });

    let v = create_rw_signal(ButtonVariants::Outlined);

    view! {
      <main>
        // Textarea

        // <Textarea
        // readonly=textarea_locked
        // textarea_node_ref=textarea
        // label="Quick Reader"
        // on:input=move |ev| set_text(event_target_value(&ev))
        // >
        // {text}
        // </Textarea>

        // display
        <p>{move || text_chunks.with(|t| t[current_index()].chunk.clone())}</p>

        // Button testing

        <Button variant=v on:click=move |_| v.set(ButtonVariants::Filled)>
          "medium"
        </Button>

        <br/>

        // Original

        // <button on:click=move |_| stop() disabled=(move || !textarea_locked()).derive_signal()>
        // "Stop"
        // </button>
        // <br/>
        // <button on:click=move |_| restart() disabled=(move || !textarea_locked()).derive_signal()>
        // "Restart"
        // </button>
        // <br/>
        // <button on:click=move |_| toggle_playing()>
        // {move || if playing() { "Pause" } else { "Play" }}
        // </button>
        <br/>
        <input
          type="number"
          min="1"
          max="10"
          value=chunk_size
          on:input=move |ev| set_chunk_size(event_target_value(&ev).parse().unwrap_or(1))
        />
        <br/>
        <input
          type="number"
          step="50"
          min="100"
          max="1000"
          value=words_per_minute
          on:input=move |ev| set_words_per_minute(event_target_value(&ev).parse().unwrap_or(300))
        />
      </main>
    }
}

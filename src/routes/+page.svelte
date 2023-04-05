<!-- 
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
 -->
<script lang="ts">
  import app_data from "$lib/stores/app_data";
  import { onMount } from "svelte";
  import { split_text } from "$lib/splitter";
  import app_settings from "$lib/stores/app_settings";
  import Animated from "$lib/Animated.svelte";

  $: $app_data.chunks = split_text($app_data.text, $app_data.chunk_size);

  $: speed = (1000 / ($app_data.wpm / 60)) * $app_data.chunk_size;

  $: duration_seconds =
    (speed *
      $app_data.chunk_size *
      ($app_data.chunks.length - $app_data.current_index)) /
    1000 /
    $app_data.chunk_size;

  $: duration = `${Math.floor((duration_seconds % 3600) / 60)}m ${Math.floor(
    duration_seconds % 60
  )}s`;

  let playing = false;
  let textarea_locked = false;
  let play_pause_img: HTMLImageElement;
  let textarea: HTMLTextAreaElement;

  function reset() {
    $app_data.wpm = 300;
    $app_data.chunk_size = 1;
  }

  function stop() {
    playing = textarea_locked = false;
    $app_data.current_index = 0;
    play_pause_img.src = "/play.svg";
    window.getSelection()?.empty();
  }

  function restart() {
    $app_data.current_index = 0;
  }

  function toggle_playing() {
    if (playing) {
      play_pause_img.src = "/play.svg";
      playing = false;
    } else {
      play_pause_img.src = "/pause.svg";
      playing = true;
      textarea_locked = true;
    }
  }

  function advance_chunk() {
    if ($app_data.current_index < $app_data.chunks.length - 1) {
      ++$app_data.current_index;
    } else {
      stop();
    }
  }

  function step() {
    if (playing) {
      advance_chunk();

      let selection_start = $app_data.chunks[$app_data.current_index].start_pos;
      let selection_stop =
        $app_data.chunks[$app_data.current_index].stop_pos + 1;

      textarea.setSelectionRange(selection_start, selection_stop);
    }

    setTimeout(step, speed);
  }

  onMount(step);

  function changed_wpm(e: Event) {
    const input = e.target as HTMLInputElement;

    $app_data.wpm = Math.min(
      parseInt(input.max),
      Math.max(parseInt(input.min), input.valueAsNumber)
    );
  }

  function changed_chunk_size(e: Event) {
    const input = e.target as HTMLInputElement;

    let new_chunk_size = Math.min(
      parseInt(input.max),
      Math.max(parseInt(input.min), input.valueAsNumber)
    );

    // Recalculate what the new chunk index should be after recreating the text chunks with a different size
    $app_data.current_index = Math.floor(
      ($app_data.current_index * $app_data.chunk_size) / new_chunk_size
    );

    $app_data.chunk_size = new_chunk_size;
  }

  function keyboard_shortcut_pressed(event: KeyboardEvent) {
    switch (event.code) {
      case "ArrowLeft":
        $app_data.current_index = Math.max(
          $app_data.current_index - $app_settings.playback.jump_back_chunks,
          0
        );
        break;

      case "ArrowRight":
        $app_data.current_index = Math.min(
          $app_data.current_index + $app_settings.playback.jump_forward_chunks,
          $app_data.chunks.length - 1
        );
        break;

      case "Space":
        event.preventDefault();
        toggle_playing();
        break;
    }
  }
</script>

<svelte:window on:keydown="{keyboard_shortcut_pressed}" />

<Animated>
  <main>
    <textarea
      placeholder="Enter text to quick read."
      disabled="{textarea_locked}"
      bind:this="{textarea}"
      bind:value="{$app_data.text}"
      style="font-size: {$app_settings.fonts.textarea_font_size}pt;
    font-family: {$app_settings.fonts.textarea_font_style}"></textarea>

    <p
      class="display"
      style="font-size: {$app_settings.fonts.display_font_size}pt;
    font-family: {$app_settings.fonts.display_font_style}"
    >
      {$app_data.chunks[$app_data.current_index].chunk}
    </p>

    <div class="controls">
      <div class="chunking">
        <p>Words per minute:</p>
        <input
          type="number"
          min="60"
          max="1000"
          step="10"
          value="{$app_data.wpm}"
          on:change="{changed_wpm}"
        />
        <p>Chunk size:</p>
        <input
          type="number"
          min="1"
          max="10"
          value="{$app_data.chunk_size}"
          on:change="{changed_chunk_size}"
        />
        <button on:click="{reset}"><img src="/reset.svg" alt="" />Reset</button>
      </div>

      <div class="progress">
        <p>
          Chunk {$app_data.current_index + 1} of {Math.floor(
            $app_data.chunks.length / $app_data.chunk_size
          )}
        </p>
        <div class="vertical-separator"></div>
        <p>Duration: {duration}</p>
        <div class="vertical-separator"></div>
        <input
          class="progress"
          type="range"
          min="0"
          max="{$app_data.chunks.length - 1}"
          bind:value="{$app_data.current_index}"
        />
      </div>

      <div class="playback">
        <button on:click="{stop}" disabled="{!textarea_locked}"
          ><img src="/stop.svg" alt="" />Stop</button
        >

        <button on:click="{restart}" disabled="{!textarea_locked}"
          ><img src="/restart.svg" alt="" />Restart</button
        >

        <button on:click="{toggle_playing}"
          ><img src="/play.svg" alt="" bind:this="{play_pause_img}" />{playing
            ? "Pause"
            : "Start"}</button
        >
      </div>
    </div>
  </main>
</Animated>

<style>
  main {
    display: grid;
    grid-template-rows: 10%, repeat(3, 30%);
    height: 100%;
  }

  textarea {
    margin: 0;
    padding: 15px 0;
    text-align: center;
    width: 100%;
  }

  p.display {
    text-align: center;
    align-self: center;
  }

  div.controls {
    align-self: end;
    margin-bottom: 30pt;
    /* without this display, the div somehow becomes flex after going to the font settings and back */
    display: inherit;
  }

  div.controls p {
    font-weight: bold;
    font-size: 9pt;
  }

  div.chunking {
    display: flex;
    gap: 5pt;
    justify-content: center;
    align-items: center;
  }

  div.progress {
    display: flex;
    gap: 3pt;
  }

  input.progress {
    flex-grow: 2;
  }

  div.vertical-separator {
    width: 1px;
    height: 30px;
    align-self: center;
    background-color: rgba(252, 252, 252, 0.3);
  }

  input {
    font-size: 13pt;
  }

  div.playback {
    display: grid;
    grid-template-columns: repeat(3, 33%);
    justify-content: center;
    gap: 5pt;
  }

  button {
    font-size: 12pt;
    height: 22pt;
  }

  img {
    margin-right: 10px;
  }
</style>

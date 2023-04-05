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
  import { goto } from "$app/navigation";
  import app_settings from "$lib/stores/app_settings";
  import { invoke } from "@tauri-apps/api";
  import Animated from "$lib/Animated.svelte";

  // Save settings every time they are changed
  $: invoke("set_settings", { newSettings: $app_settings });
</script>

<Animated>
  <main>
    <div>
      <button on:click="{() => goto('/settings/font-chooser/display')}"
        >Choose display font family</button
      >
      <p>{$app_settings.fonts.display_font_style}</p>
    </div>

    <div>
      <input
        type="number"
        min="1"
        bind:value="{$app_settings.fonts.display_font_size}"
      />
      <p>Display font size</p>
    </div>

    <div>
      <button on:click="{() => goto('/settings/font-chooser/textarea')}"
        >Choose textarea font family</button
      >
      <p>{$app_settings.fonts.textarea_font_style}</p>
    </div>

    <div>
      <input
        type="number"
        min="1"
        bind:value="{$app_settings.fonts.textarea_font_size}"
      />
      <p>Text area font size</p>
    </div>

    <div>
      <input
        type="number"
        min="1"
        bind:value="{$app_settings.playback.jump_back_chunks}"
      />
      <p>Jump back chunks</p>
    </div>

    <div>
      <input
        type="number"
        min="1"
        bind:value="{$app_settings.playback.jump_forward_chunks}"
      />
      <p>Jump forward chunks</p>
    </div>

    <div>
      <select bind:value="{$app_settings.window.style}">
        {#each ["auto", "win32", "darwin", "linux"] as window_style}
          <option>{window_style}</option>
        {/each}
      </select>
      <p>Window style</p>
    </div>
  </main>
</Animated>

<style>
  main {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  div {
    display: flex;
    gap: 5pt;
    justify-content: start;
    align-items: center;
  }

  button {
    font-size: 12pt;
    padding: 5pt;
  }

  input {
    font-size: 13pt;
    width: 7%;
  }
</style>

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
  import { invoke } from "@tauri-apps/api";
  import Animated from "$lib/components/Animated.svelte";
  import router from "$stores/router";
  import appSettings from "$stores/app_settings";

  // Save settings every time they are changed
  $: invoke("set_settings", { newSettings: $appSettings });
</script>

<Animated>
  <main>
    <div>
      <button on:click="{() => router.push('FontsChooser/Display')}"
        >Choose display font family</button
      >
      <p>{$appSettings.fonts.displayFontStyle}</p>
    </div>

    <div>
      <input
        type="number"
        min="1"
        bind:value="{$appSettings.fonts.displayFontSize}"
      />
      <p>Display font size</p>
    </div>

    <div>
      <button on:click="{() => router.push('FontsChooser/Textarea')}"
        >Choose text area font family</button
      >
      <p>{$appSettings.fonts.textareaFontStyle}</p>
    </div>

    <div>
      <input
        type="number"
        min="1"
        bind:value="{$appSettings.fonts.textareaFontSize}"
      />
      <p>Text area font size</p>
    </div>

    <div>
      <input
        type="number"
        min="1"
        bind:value="{$appSettings.playback.jumpBackChunks}"
      />
      <p>Jump back chunks</p>
    </div>

    <div>
      <input
        type="number"
        min="1"
        bind:value="{$appSettings.playback.jumpForwardChunks}"
      />
      <p>Jump forward chunks</p>
    </div>

    <div>
      <select bind:value="{$appSettings.window.style}">
        {#each ["Auto", "Windows Mica", "MacOS Monterey", "Linux Breeze"] as windowStyle}
          <option>{windowStyle}</option>
        {/each}
      </select>
      <p>Window style</p>
    </div>

    <div>
      <select bind:value="{$appSettings.window.theme}">
        {#each ["Auto", "Dark", "Light"] as windowTheme}
          <option>{windowTheme}</option>
        {/each}
      </select>
      <p>Window theme</p>
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

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
  import { page } from "$app/stores";
  import app_settings from "$lib/stores/app_settings";
  import fonts from "$lib/stores/fonts";
  import Animated from "$lib/Animated.svelte";

  async function get_fonts(): Promise<string[]> {
    if (!$fonts.fonts_loaded) {
      await fonts.load();
    }

    return $fonts.fonts;
  }

  let selected_font_family =
    $page.params.target === "display"
      ? $app_settings.fonts.display_font_style
      : $app_settings.fonts.textarea_font_style;

  function chose_font() {
    switch ($page.params.target) {
      case "display":
        $app_settings.fonts.display_font_style = selected_font_family;
        break;
      case "textarea":
        $app_settings.fonts.textarea_font_style = selected_font_family;
        break;
    }

    history.back();
  }
</script>

{#await get_fonts()}
  <Animated>
    <p class="loading">Reading system fonts</p>
  </Animated>
{:then fonts}
  <Animated>
    <main>
      <select bind:value="{selected_font_family}">
        {#each fonts as font, index (index)}
          <option>{font}</option>
        {/each}
      </select>

      <p style="font-family: {selected_font_family}">
        The quick brown fox jumps over the lazy dog
      </p>

      <div class="buttons">
        <button on:click="{() => history.back()}">Back</button>
        <button on:click="{chose_font}">Save</button>
      </div>
    </main>
  </Animated>
{:catch error}
  <Animated>
    <p>Error loading system fonts: {error}</p>
  </Animated>
{/await}

<style>
  main {
    height: 100%;
    display: grid;
    grid-template-rows: 10% 70% 20%;
    align-items: center;
  }

  p.loading {
    text-align: center;
  }

  p {
    font-size: 20pt;
    word-wrap: normal;
    justify-self: center;
  }

  div.buttons {
    display: flex;
    justify-content: center;
    gap: 5pt;
  }

  button {
    flex-grow: 1;
    font-size: 12pt;
  }
</style>

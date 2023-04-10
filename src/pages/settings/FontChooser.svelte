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
  import fonts from "$stores/fonts";
  import router from "$stores/router";
  import { fade } from "svelte/transition";
  import { animationDuration as duration } from "$lib/consts";

  async function getFonts(): Promise<string[]> {
    if (!$fonts.fontsLoaded) {
      await fonts.load();
    }

    return $fonts.fonts;
  }

  let [currentFontFamily, saveFont] = $router.at(-1)?.data as [
    string,
    (fontFamily: string) => void
  ];

  function save() {
    saveFont(currentFontFamily);
    router.pop();
  }
</script>

{#await getFonts()}
  <p class="loading">Reading system fonts</p>
{:then fonts}
  <main in:fade="{{ duration }}">
    <select bind:value="{currentFontFamily}">
      {#each fonts as font, index (index)}
        <option>{font}</option>
      {/each}
    </select>

    <p style="font-family: {currentFontFamily}">
      The quick brown fox jumps over the lazy dog
    </p>

    <div class="buttons">
      <button on:click="{() => router.pop()}">Back</button>
      <button on:click="{save}">Save</button>
    </div>
  </main>
{:catch error}
  <p>Error loading system fonts: {error}</p>
{/await}

<style>
  main {
    height: 100%;
    display: grid;
    grid-template-rows: 10% 80% 10%;
    align-items: center;
  }

  p.loading {
    text-align: center;
  }

  p {
    font-size: 20pt;
    align-self: center;
    justify-self: center;
  }

  div.buttons {
    align-self: end;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 5pt;
  }

  button {
    font-size: 12pt;
  }
</style>

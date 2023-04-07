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
  import Animated from "$lib/components/Animated.svelte";
  import fonts from "$stores/fonts";
  import router from "$stores/router";

  async function getFonts(): Promise<string[]> {
    if (!$fonts.fontsLoaded) {
      await fonts.load();
    }

    return $fonts.fonts;
  }

  export let currentFontFamily: string;
  export let saveFont: (fontFamily: string) => void;

  function save() {
    saveFont(currentFontFamily);
    router.pop();
  }
</script>

{#await getFonts()}
  <Animated>
    <p class="loading">Reading system fonts</p>
  </Animated>
{:then fonts}
  <Animated>
    <main>
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

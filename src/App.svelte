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
  import router from "$stores/router";
  import appSettings from "$stores/app_settings";

  import Settings from "$/pages/settings/Settings.svelte";
  import FontChooser from "$/pages/settings/FontChooser.svelte";
  import QuickReader from "$/pages/QuickReader.svelte";
  import About from "$/pages/About.svelte";

  $: currentPage = $router.history.at(-1);
</script>

<Animated>
  <main>
    <nav>
      <button on:click="{() => router.push('Settings')}">Settings</button>
      <button on:click="{() => router.push('QuickReader')}">Quick Reader</button
      >
      <button on:click="{() => router.push('About')}">About</button>
    </nav>

    {#if currentPage === "QuickReader"}
      <QuickReader />
    {:else if currentPage === "Settings"}
      <Settings />
    {:else if currentPage === "About"}
      <About />
    {:else if currentPage === "FontsChooser/Display"}
      <FontChooser
        currentFontFamily="{$appSettings.fonts.displayFontStyle}"
        saveFont="{(fontFamily) =>
          ($appSettings.fonts.displayFontStyle = fontFamily)}"
      />
    {:else if currentPage === "FontsChooser/Textarea"}
      <FontChooser
        currentFontFamily="{$appSettings.fonts.textareaFontStyle}"
        saveFont="{(fontFamily) =>
          ($appSettings.fonts.textareaFontStyle = fontFamily)}"
      />
    {:else}
      <p>Unknown page. How did we get here?</p>
    {/if}
  </main>
</Animated>

<style>
  main {
    height: 100vh;
    margin: 5pt 10pt;
    display: grid;
    gap: 10pt;
    grid-template-rows: 5% 95%;
  }

  nav {
    display: flex;
    gap: 5pt;
    align-items: center;
  }

  nav button {
    flex-grow: 1;
  }
</style>

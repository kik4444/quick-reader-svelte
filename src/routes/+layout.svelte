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
  import "$lib/css/base.css";
  import appSettings from "$lib/stores/app_settings";
  import Animated from "$lib/Animated.svelte";
  import platformInfo from "$lib/stores/platform_info";
  import router from "$lib/stores/router";

  import Display from "./+page.svelte";
  import Settings from "./settings/+page.svelte";
  import About from "./about/+page.svelte";
  import FontChooser from "./settings/font-chooser/[target]/+page.svelte";

  async function init() {
    await Promise.all([appSettings.load(), platformInfo.load()]);
  }

  // The style and theme can be changed from the settings page.
  // We react to changes in the style or theme here
  // and update the html element's data tags accordingly.
  // This is followed by the browser changing the CSS variables in use for colors
  $: {
    let windowStyle: string;

    if ($appSettings.window?.style !== "Auto") {
      windowStyle = $appSettings.window?.style;
    } else {
      switch ($platformInfo.platformName) {
        case "win32":
          windowStyle = "Windows Mica";
          break;

        case "darwin":
        case "ios":
          windowStyle = "MacOS Monterey";
          break;

        default:
          windowStyle = "Linux Breeze";
      }
    }

    document.documentElement.setAttribute("data-style", windowStyle);
  }

  $: {
    let windowTheme: string;

    if ($appSettings.window?.theme !== "Auto") {
      windowTheme = $appSettings.window?.theme;
    } else {
      windowTheme = window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "Dark"
        : "Light";
    }

    document.documentElement.setAttribute("data-theme", windowTheme);
  }

  $: latestPage = $router.history.at(-1)?.page;

  function shortcut_pressed(event: KeyboardEvent) {
    switch (event.code) {
      case "Escape":
        router.pop();
        break;
    }
  }
</script>

<svelte:window on:keydown="{shortcut_pressed}" />

{#await init()}
  <Animated>
    <p>Loading</p>
  </Animated>
{:then}
  <Animated>
    <main>
      <nav>
        <button on:click="{() => router.push('Settings')}">Settings</button>
        <button on:click="{() => router.push('Display')}">Quick Reader</button>
        <button on:click="{() => router.push('About')}">About</button>
      </nav>

      {#if latestPage === "Display"}
        <Display />
      {:else if latestPage === "Settings"}
        <Settings />
      {:else if latestPage === "About"}
        <About />
      {:else if latestPage === "FontsChooser/Display"}
        <FontChooser
          currentFontFamily="{$appSettings.fonts.displayFontStyle}"
          saveFont="{(fontFamily) =>
            ($appSettings.fonts.displayFontStyle = fontFamily)}"
        />
      {:else if latestPage === "FontsChooser/Textarea"}
        <FontChooser
          currentFontFamily="{$appSettings.fonts.textareaFontStyle}"
          saveFont="{(fontFamily) =>
            ($appSettings.fonts.textareaFontStyle = fontFamily)}"
        />
      {/if}
    </main>
  </Animated>
{:catch error}
  <Animated>
    <p>Error initializing application: {error}</p>
  </Animated>
{/await}

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

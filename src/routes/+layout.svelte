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
  import { goto } from "$app/navigation";
  import appSettings from "$lib/stores/app_settings";
  import Animated from "$lib/Animated.svelte";

  // The style and theme can be changed from the settings page.
  // We react to changes in the style or theme here
  // and update the html element's data tags accordingly.
  // This is followed by the browser changing the CSS variables in use for colors
  $: {
    let windowStyle: string;

    if ($appSettings.window?.style !== "Auto") {
      windowStyle = $appSettings.window?.style;
    } else {
      switch (import.meta.env.TAURIPLATFORM) {
        case "win32":
          windowStyle = "Windows Mica";

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
</script>

{#await appSettings.load()}
  <Animated>
    <p>Loading</p>
  </Animated>
{:then}
  <Animated>
    <main>
      <nav>
        <button on:click="{() => goto('/settings')}">Settings</button>
        <button on:click="{() => goto('/')}">Quick Reader</button>
        <button on:click="{() => goto('/about')}">About</button>
      </nav>
      <slot />
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

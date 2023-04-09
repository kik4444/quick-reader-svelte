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
  import "$/styles/base.css";
  import appSettings from "$stores/app_settings";
  import platformInfo from "$stores/platform_info";
  import App from "$/App.svelte";

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
</script>

{#await init()}
  <p>Loading</p>
{:then}
  <App />
{:catch error}
  <p>Error initializing application: {error}</p>
{/await}

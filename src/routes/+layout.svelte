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

  $: {
    let windowStyle: string;

    if ($appSettings.window?.style !== "auto") {
      windowStyle = $appSettings.window?.style;
    } else {
      switch (import.meta.env.TAURIPLATFORM) {
        case "win32":
        case "darwin":
          windowStyle = import.meta.env.TAURIPLATFORM;
          break;

        default:
          windowStyle = "linux";
      }
    }

    document.documentElement.setAttribute("data-style", windowStyle);
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

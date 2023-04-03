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
  import appsettings from "$lib/stores/appsettings";
  import Animated from "$lib/Animated.svelte";

  async function init() {
    switch (import.meta.env.TAURI_PLATFORM) {
      case "win32":
        console.log("TODO");
        break;

      case "darwin":
        console.log("TODO");
        break;

      default:
        await import("$lib/css/linux.css");
        break;
    }

    await import("$lib/css/base.css");

    await appsettings.load();
  }
</script>

{#await init()}
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

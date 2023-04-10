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
  import router from "$stores/router";
  import { fade } from "svelte/transition";
  import { animationDuration as duration } from "$/lib/constants";

  import Settings from "$/pages/settings/Settings.svelte";
  import QuickReader from "$/pages/QuickReader.svelte";
  import About from "$/pages/About.svelte";

  // Non-null assertion, because the router is guaranteed to always have at least one page
  $: currentPage = $router.at(-1)!.page;

  function shortcut_pressed(event: KeyboardEvent) {
    switch (event.code) {
      case "Escape":
        router.pop();
        break;
    }
  }
</script>

<svelte:window on:keydown="{shortcut_pressed}" />

<main>
  <nav in:fade="{{ duration }}">
    <button on:click="{() => router.push(Settings)}">Settings</button>
    <button on:click="{() => router.push(QuickReader)}">Quick Reader</button>
    <button on:click="{() => router.push(About)}">About</button>
  </nav>

  {#key currentPage}
    <div in:fade="{{ duration }}">
      <svelte:component this="{currentPage}" />
    </div>
  {/key}
</main>

<style>
  main {
    height: 95vh;
    margin: 5pt 10pt;
    display: grid;
    grid-template-rows: 3fr 97fr;
    gap: 5pt;
  }

  nav {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 5pt;
  }
</style>

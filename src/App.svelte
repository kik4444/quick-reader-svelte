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
  import router, { Page } from "$stores/router";
  import { fade } from "svelte/transition";

  import Settings from "$/pages/settings/Settings.svelte";
  import FontChooser from "$/pages/settings/FontChooser.svelte";
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
  <nav in:fade="{{ duration: 300 }}">
    <button on:click="{() => router.push(Page.Settings)}">Settings</button>
    <button on:click="{() => router.push(Page.QuickReader)}"
      >Quick Reader</button
    >
    <button on:click="{() => router.push(Page.About)}">About</button>
  </nav>

  {#key currentPage}
    <div in:fade="{{ duration: 300 }}">
      {#if currentPage === Page.QuickReader}
        <QuickReader />
      {:else if currentPage === Page.Settings}
        <Settings />
      {:else if currentPage === Page.About}
        <About />
      {:else if currentPage === Page.FontChooser}
        <FontChooser />
      {:else}
        <p>Unknown page. How did we get here?</p>
      {/if}
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

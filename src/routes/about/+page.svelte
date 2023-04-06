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
  import Animated from "$lib/Animated.svelte";
  import { open } from "@tauri-apps/api/shell";

  const source_code_link = "https://github.com/kik4444/quick-reader";
  const license_link = "https://www.gnu.org/licenses";

  async function clicked_link(e: Event) {
    e.preventDefault();
    const anchor = e.target as HTMLAnchorElement;
    await open(anchor.href);
  }
</script>

<Animated>
  <main>
    <img src="/original-icon.svg" alt="" />
    <p>
      Quick Reader is a program that enables its user to quickly read through
      text by encouraging the silencing of their subvocalization. Quick Reader
      is free/libre software released under the GPL version 3. The source code
      is available at <a href="{source_code_link}" on:click="{clicked_link}"
        >{source_code_link}</a
      >
    </p>
    <p>
      You should have received a copy of the GNU General Public License along
      with this program. If not, see <a
        href="{license_link}"
        on:click="{clicked_link}">{license_link}</a
      >
    </p>
  </main>
</Animated>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10pt;
  }

  p {
    margin: 0;
  }

  img {
    height: 10vh;
  }
</style>

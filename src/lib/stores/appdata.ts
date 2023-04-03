/*    
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
 */

import { writable, type Writable } from "svelte/store";
import type { TextChunk } from "../splitter";

interface AppData {
  text: string;
  current_index: number;
  chunks: TextChunk[];
  wpm: number;
  chunk_size: number;
}

const appdata: Writable<AppData> = writable({
  text: 'Welcome to "Quick Reader". Press start to begin reading quickly.',
  current_index: 0,
  chunks: [
    {
      chunk: "",
      start_pos: 0,
      stop_pos: 0
    }
  ],
  wpm: 300,
  chunk_size: 1,
});

export default appdata;

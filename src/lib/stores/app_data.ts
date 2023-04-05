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
  currentIndex: number;
  chunks: TextChunk[];
  wpm: number;
  chunkSize: number;
  textareaLocked: boolean;
}

const appData: Writable<AppData> = writable({
  text: 'Welcome to "Quick Reader". Press start to begin reading quickly.',
  currentIndex: 0,
  chunks: [
    {
      chunk: "",
      startPos: 0,
      stopPos: 0
    }
  ],
  wpm: 300,
  chunkSize: 1,
  textareaLocked: false
});

export default appData;

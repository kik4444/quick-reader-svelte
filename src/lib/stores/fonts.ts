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

import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";

interface Fonts {
    fontsLoaded: boolean;
    fonts: string[];
}

function createStore() {
    let initialValue = { fontsLoaded: false } as Fonts;

    const { subscribe, update, set } = writable(initialValue);

    return {
        async load() {
            const fonts = await invoke<string[]>("get_system_fonts");
            set({ fontsLoaded: true, fonts });
        },
        subscribe,
        set,
        update,
    };
}

export default createStore();

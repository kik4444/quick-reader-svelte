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

interface AppSettings {
    fonts: {
        display_font_style: string;
        display_font_size: number;
        textarea_font_style: string;
        textarea_font_size: number;
    };
    playback: {
        jump_back_chunks: number;
        jump_forward_chunks: number;
    };
}

function create_store() {
    let initial_value = {} as AppSettings;

    const { subscribe, update, set } = writable(initial_value);

    return {
        async load() {
            const app_settings = await invoke<AppSettings>("get_settings");
            set(app_settings);
        },
        subscribe,
        set,
        update,
    };
}

export default create_store();

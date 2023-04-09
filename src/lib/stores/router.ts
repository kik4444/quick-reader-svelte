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

import type { ComponentType } from "svelte";
import { writable } from "svelte/store";

import FontChooser from "$/pages/settings/FontChooser.svelte";
import QuickReader from "$/pages/QuickReader.svelte";

interface Route {
    page: ComponentType,
    data: any;
}

type Router = Route[];

// Pages you can only exit from. These pages cannot remain in the router history
const FinalPages: [ComponentType] = [FontChooser];

function createRouter() {
    const initialValue = [{ page: QuickReader }] as Router;
    const { subscribe, update } = writable(initialValue);

    function push(page: ComponentType, data: any = null) {
        update(router => {
            const currentPage = router.at(-1)!.page;

            if (page !== currentPage && !FinalPages.includes(currentPage)) {
                router.push({ page, data });
            }

            return router;
        });
    }

    function pop() {
        update(router => {
            if (router.length > 1) {
                router.pop();
            }
            return router;
        });
    }

    return {
        subscribe,
        push,
        pop,

        pushFontChooser(data: [string, (fontFamily: string) => void]) {
            push(FontChooser, data);
        },
    };
};

export default createRouter();

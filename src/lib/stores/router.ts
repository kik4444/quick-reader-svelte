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

import { writable } from "svelte/store";

export enum Page {
    QuickReader,
    Settings,
    About,
    FontChooser
}

interface Route {
    page: Page,
    data: any;
}

type Router = Route[];

const FinalPages = [Page.FontChooser];

function createRouter() {
    const initialValue = [{ page: Page.QuickReader }] as Router;
    const { subscribe, update } = writable(initialValue);

    function push(page: Page, data: any = null) {
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
            push(Page.FontChooser, data);
        },
    };
};

export default createRouter();

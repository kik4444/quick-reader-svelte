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

interface Router {
    history: {
        page: string;
    }[];
}

function createRouter() {
    let initialValue = { history: [{ page: "Display" }] } as Router;
    const { subscribe, set, update } = writable(initialValue);
    set(initialValue);

    return {
        subscribe,

        push(page: string) {
            update(router => {
                if (page !== router.history.at(-1)?.page) {
                    // We force Svelte to notify this store's subscribers of a change by using the assignment operator
                    let ref = router;
                    ref.history.push({ page });
                }
                return router;
            });
        },

        pop() {
            update(router => {
                if (router.history.length >= 2) {
                    let ref = router;
                    ref.history.pop();
                }
                return router;
            });
        }
    };
};

export default createRouter();

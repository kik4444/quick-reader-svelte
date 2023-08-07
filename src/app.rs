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

use leptos::*;
use leptos_router::*;

use crate::pages::quick_reader::QuickReader;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <nav></nav>
        <main>
          <Routes>
            <Route path="/" view=QuickReader/>
          </Routes>
        </main>
      </Router>
    }
}

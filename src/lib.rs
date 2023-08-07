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

use serde_wasm_bindgen::{Deserializer, Serializer};
use wasm_bindgen::JsValue;

pub mod app;
pub mod errors;
pub mod js_bindings;
pub mod splitter;

pub trait ToJsValue {
    fn to_js_value(&self) -> core::result::Result<JsValue, serde_wasm_bindgen::Error>;
}

pub trait IntoValue {
    fn into_value<T>(self) -> core::result::Result<T, serde_wasm_bindgen::Error>
    where
        T: serde::de::DeserializeOwned;
}

impl<T> ToJsValue for T
where
    T: serde::ser::Serialize + ?Sized,
{
    fn to_js_value(&self) -> core::result::Result<JsValue, serde_wasm_bindgen::Error> {
        self.serialize(&Serializer::new())
    }
}

impl IntoValue for JsValue {
    fn into_value<T>(self) -> core::result::Result<T, serde_wasm_bindgen::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        T::deserialize(Deserializer::from(self))
    }
}

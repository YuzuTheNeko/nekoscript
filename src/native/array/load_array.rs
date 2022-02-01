use std::collections::HashMap;
use crate::core::native_function::PrototypeNativeFunction;
use crate::native::array::at::at;
use crate::native::array::length::length;
use crate::native::array::map::map;
use crate::native::array::set::set;

pub fn load_array() -> HashMap<String, PrototypeNativeFunction> {
    HashMap::from(
        [
            (String::from("map"), map()),
            (String::from("at"), at()),
            (String::from("set"), set()),
            (String::from("length"), length())
        ]
    )
}
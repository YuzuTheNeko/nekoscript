use std::collections::HashMap;
use crate::constants::data_types::{ARRAY_TYPE, INT_TYPE};
use crate::core::native_function::PrototypeNativeFunction;
use crate::native::array::load_array::load_array;
use crate::native::int::load_int::load_int;

pub fn get_prototypes(kind: &str) -> HashMap<String, PrototypeNativeFunction> {
    if kind.eq(INT_TYPE) {
        load_int()
    } else if kind.eq(ARRAY_TYPE) {
        load_array()
    } else {
        HashMap::default()
    }
}
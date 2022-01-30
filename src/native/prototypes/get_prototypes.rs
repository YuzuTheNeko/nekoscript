use crate::constants::data_types::{ARRAY_TYPE, INT_TYPE};
use crate::core::native_function::PrototypeNativeFunction;
use crate::native::prototypes::array::load_array::load_array;
use crate::native::prototypes::int::load_int::load_int;

pub fn get_prototypes(kind: &str) -> Vec<PrototypeNativeFunction> {
    if kind.eq(INT_TYPE) {
        load_int()
    } else if kind.eq(ARRAY_TYPE) {
        load_array()
    } else {
        vec![]
    }
}
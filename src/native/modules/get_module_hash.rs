use std::collections::HashMap;
use crate::core::native_function::NativeFunction;
use crate::native::modules::util::util::{native_util, UTIL_MODULE};

pub fn get_module_hash(name: String) -> HashMap<String, NativeFunction> {
    if name.eq(UTIL_MODULE) {
        native_util()
    } else {
        HashMap::default()
    }
}
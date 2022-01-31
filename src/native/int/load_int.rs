use std::collections::HashMap;
use crate::core::native_function::PrototypeNativeFunction;
use crate::native::int::to_string::to_string;

pub fn load_int() -> HashMap<String, PrototypeNativeFunction> {
    HashMap::from(
        [
            (String::from("to_string"), to_string())
        ]
    )
}
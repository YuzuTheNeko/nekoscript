use crate::core::native_function::PrototypeNativeFunction;
use crate::native::prototypes::int::to_string::to_string;

pub fn load_int() -> Vec<PrototypeNativeFunction> {
    vec![
        to_string()
    ]
}
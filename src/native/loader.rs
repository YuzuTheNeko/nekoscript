use crate::core::native_function::NativeFunction;
use crate::native::log::log;

pub fn load_native_functions() -> Vec<NativeFunction> {
    vec![
        log()
    ]
}
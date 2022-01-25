use crate::core::native_function::NativeFunction;
use crate::native::_typeof::_typeof;
use crate::native::log::log;

pub fn load_native_functions() -> Vec<NativeFunction> {
    vec![
        log(),
        _typeof()
    ]
}
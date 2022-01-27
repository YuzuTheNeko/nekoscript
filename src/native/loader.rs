use crate::core::native_function::NativeFunction;
use crate::native::_typeof::_typeof;
use crate::native::log::log;
use crate::native::system_time::system_time;

pub fn load_native_functions() -> Vec<NativeFunction> {
    vec![log(), _typeof(), system_time()]
}

use crate::core::native_function::PrototypeNativeFunction;
use crate::native::prototypes::array::at::at;

pub fn load_array() -> Vec<PrototypeNativeFunction> {
    vec![
        at()
    ]
}
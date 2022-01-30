use crate::core::native_function::PrototypeNativeFunction;
use crate::native::prototypes::array::at::at;
use crate::native::prototypes::array::length::length;
use crate::native::prototypes::array::set::set;

pub fn load_array() -> Vec<PrototypeNativeFunction> {
    vec![
        at(),
        set(),
        length()
    ]
}
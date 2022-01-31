use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::core::native_function::NativeFunction;
use crate::native::functions::_typeof::_typeof;
use crate::native::functions::load_file::load_file;
use crate::native::functions::log::log;
use crate::native::functions::system_time::system_time;

pub fn wrap(f: NativeFunction) -> Rc<RefCell<NativeFunction>> {
    Rc::new(RefCell::new(f))
}
pub fn load_native_functions() -> HashMap<String, Rc<RefCell<NativeFunction>>> {
    HashMap::from(
        [
            (String::from("log"), wrap(log())),
            (String::from("typeof"), wrap(_typeof())),
            (String::from("system_time"), wrap(system_time())),
            (String::from("load_file"), wrap(load_file()))
        ]
    )
}

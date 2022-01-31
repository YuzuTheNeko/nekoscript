use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::{DataTypes, NativeFnType};
use crate::core::native_function::NativeFunction;
use crate::native::modules::util::constants::FORMAT;
use crate::native::modules::util::util::UTIL_MODULE;

pub fn format() -> Rc<RefCell<DataTypes>> {
    DataTypes::wrap(
        DataTypes::NativeFn {
            name: FORMAT.to_owned(),
            params: None,
            kind: NativeFnType::Module,
            module: Some(UTIL_MODULE.to_owned())
        }
    )
}

pub fn format_this() -> NativeFunction {
    NativeFunction(
        Box::new(| scope, args | {
            let mut vc = vec![];

            for i in args {
                let i = i.borrow();
                vc.push(i.to_string())
            }

            Ok(
                DataTypes::wrap(DataTypes::Text(vc.join(" ")))
            )
        })
    )
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;
use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;
use crate::native::modules::util::constants::FORMAT;
use crate::native::modules::util::format::{format, format_this};

pub const UTIL_MODULE: &str = "util";

pub fn util() -> Rc<RefCell<DataTypes>> {
    DataTypes::wrap(
        DataTypes::Object(
            Rc::new(RwLock::new(
                HashMap::from(
                    [
                        (FORMAT.to_owned(), format())
                    ]
                )
            ))
        )
    )
}

pub fn native_util() -> HashMap<String, NativeFunction> {
    HashMap::from(
        [
            (FORMAT.to_owned(), format_this())
        ]
    )
}
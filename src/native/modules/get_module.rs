use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;
use crate::core::data_types::DataTypes;
use crate::native::modules::util::util::{util, UTIL_MODULE};

pub fn get_module(name: String) -> Option<Rc<RefCell<DataTypes>>> {
    if name.eq(UTIL_MODULE) {
        Some(util())
    } else {
        None
    }
}
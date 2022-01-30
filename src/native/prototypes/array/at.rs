use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::DataTypes;
use crate::core::native_function::{PropType, PrototypeNativeFunction};

pub fn at() -> PrototypeNativeFunction {
    PrototypeNativeFunction {
        name: String::from("at"),
        kind: PropType::Method,
        body: Box::new(| scope, this, args| {
            let pos: &Rc<RefCell<DataTypes>> = args.get(0).unwrap();
            let pos = pos.borrow();
            let pos = pos.to_int();

            let arr = this.to_array();
            let arr = arr.read().unwrap();

            if let Some(val) = arr.get(*pos as usize) {
                Ok(val.clone() as Rc<RefCell<DataTypes>>)
            } else {
                Ok(DataTypes::null())
            }
        })
    }
}
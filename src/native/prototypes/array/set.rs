use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::DataTypes;
use crate::core::native_function::{PropType, PrototypeNativeFunction};

pub fn set() -> PrototypeNativeFunction {
    PrototypeNativeFunction {
        name: String::from("set"),
        kind: PropType::Method,
        body: Box::new(| scope, this, args | {
            {
                let arr = this.to_array();
                let mut arr = arr.write().unwrap();

                let arg: &Rc<RefCell<DataTypes>> = args.get(0).unwrap();
                let arg = arg.borrow();
                let arg = arg.to_int();

                let value: &Rc<RefCell<DataTypes>> = args.get(1).unwrap();

                arr.insert(*arg as usize, value.clone());
            }

            Ok(DataTypes::null())
        })
    }
}
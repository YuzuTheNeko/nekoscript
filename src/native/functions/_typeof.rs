use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;

pub fn _typeof() -> NativeFunction {
    NativeFunction(
        Box::new(| _, scope, args| {
            let arg: &Rc<RefCell<DataTypes>> = args.get(0).unwrap();
            let arg = arg.borrow();

            Ok(DataTypes::wrap(DataTypes::Text(arg.kind().to_owned())))
        })
    )
}

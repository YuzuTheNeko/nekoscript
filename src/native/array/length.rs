use crate::core::data_types::DataTypes;
use crate::core::native_function::{PropType, PrototypeNativeFunction};

pub fn length() -> PrototypeNativeFunction {
    PrototypeNativeFunction {
        kind: PropType::Property,
        body: Box::new(| scope, this, _ | {
            let arr = this.to_array();
            let arr = arr.read().unwrap();
            Ok(DataTypes::wrap(DataTypes::Int(arr.len() as i64)))
        })
    }
}
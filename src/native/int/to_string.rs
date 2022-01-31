use crate::core::data_types::DataTypes;
use crate::core::native_function::{PropType, PrototypeNativeFunction};

pub fn to_string() -> PrototypeNativeFunction {
    PrototypeNativeFunction {
        kind: PropType::Method,
        body: Box::new(| scope, this, _ | {
            Ok(DataTypes::wrap(DataTypes::Text(this.to_string())))
        })
    }
}
use std::time::{SystemTime, UNIX_EPOCH};
use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;

pub fn system_time() -> NativeFunction {
    NativeFunction {
        name: String::from("system_time"),
        body: Box::new(| _, _ | {
            Ok(DataTypes::wrap(DataTypes::Int(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64
            )))
        })
    }
}
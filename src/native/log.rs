use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;

pub fn log() -> NativeFunction {
    NativeFunction {
        name: String::from("log"),
        body: Box::new(| scope, args | {
            println!("uwu");
            DataTypes::null()
        })
    }
}
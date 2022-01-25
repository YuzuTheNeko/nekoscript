use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;

pub fn log() -> NativeFunction {
    NativeFunction {
        name: String::from("log"),
        body: Box::new(| scope, args | {
            let mut str = String::new();

            for i in args.into_iter() {
                let i = i.borrow();
                str.push_str(&i.to_string());
                str.push(' ')
            }

            println!("{}", str);

            Ok(DataTypes::null())
        })
    }
}
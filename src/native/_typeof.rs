use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;

pub fn _typeof() -> NativeFunction {
    NativeFunction {
        name: String::from("typeof"),
        body: Box::new(|scope, args| {
            let arg = args.get(0).unwrap();
            let arg = arg.borrow();

            Ok(DataTypes::wrap(DataTypes::Text(arg.kind().to_owned())))
        }),
    }
}

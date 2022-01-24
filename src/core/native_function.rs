use crate::core::data_types::DataTypes;
use crate::core::scope::Scope;

pub struct NativeFunction {
    pub name: String,
    pub body: Box<dyn Fn(&Scope, &Vec<Box<&DataTypes>>) -> DataTypes>
}
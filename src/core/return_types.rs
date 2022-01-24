use crate::core::data_types::DataTypes;

pub enum ReturnTypes {
    RuntimeError(String),
    Return(DataTypes),
    Break,
    Continue
}
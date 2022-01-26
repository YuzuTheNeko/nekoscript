use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::DataTypes;

pub enum ReturnTypes {
    RuntimeError(String),
    Return(Rc<RefCell<DataTypes>>),
    Break,
    Continue
}
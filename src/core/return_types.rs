use crate::core::data_types::DataTypes;
use std::cell::RefCell;
use std::rc::Rc;

pub enum ReturnTypes {
    RuntimeError(String),
    Return(Rc<RefCell<DataTypes>>),
    Break,
    Continue,
}

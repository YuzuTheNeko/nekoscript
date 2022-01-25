use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::scope::Scope;

pub struct NativeFunction {
    pub name: String,
    pub body: Box<dyn Fn(&Scope, &Vec<Rc<RefCell<DataTypes>>>) -> IReturn>
}
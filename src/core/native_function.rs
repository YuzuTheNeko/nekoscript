use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::scope::Scope;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::Interpreter;

pub type PropReturn = Box<dyn Fn(&Interpreter, &Scope, &Vec<Rc<RefCell<DataTypes>>>) -> IReturn>;

#[derive(PartialOrd, PartialEq)]
pub enum PropType {
    Method,
    Property
}

pub struct PrototypeNativeFunction {
    pub kind: PropType,
    pub body: Box<
        dyn Fn(
            &Interpreter, &Scope, RefMut<DataTypes>, &Vec<Rc<RefCell<DataTypes>>>
        ) -> IReturn
    >
}
pub struct NativeFunction(pub PropReturn);

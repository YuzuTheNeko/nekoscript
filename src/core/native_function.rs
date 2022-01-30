use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::scope::Scope;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub type PropReturn = Box<dyn Fn(&Scope, &Vec<Rc<RefCell<DataTypes>>>) -> IReturn>;

#[derive(PartialOrd, PartialEq)]
pub enum PropType {
    Method,
    Property
}

pub struct PrototypeNativeFunction {
    pub name: String,
    pub kind: PropType,
    pub body: Box<
        dyn Fn(
            &Scope, RefMut<DataTypes>, &Vec<Rc<RefCell<DataTypes>>>
        ) -> IReturn
    >
}
pub struct NativeFunction {
    pub name: String,
    pub body: PropReturn
}

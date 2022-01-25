use std::cell::RefCell;
use std::ops::{Add, Sub};
use std::rc::Rc;
use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::operator_types::OperatorTypes;

pub fn apply_bin(op: &OperatorTypes, left: Rc<RefCell<DataTypes>>, right: Rc<RefCell<DataTypes>>) -> IReturn {
    let lhs = &mut left.borrow_mut();
    let rhs = &mut right.borrow_mut();

    match op {
        OperatorTypes::Equals => {
            Ok(DataTypes::wrap(DataTypes::Bool(lhs.is_equal(rhs))))
        },
        OperatorTypes::Add => {
            if !lhs.is_int() || !rhs.is_int() {
                let mut str = lhs.to_string();
                str.push_str(&rhs.to_string());
                Ok(DataTypes::wrap(DataTypes::Text(str)))
            } else {
                Ok(DataTypes::wrap(DataTypes::Int(lhs.to_int().add(rhs.to_int()))))
            }
        },
        OperatorTypes::Sub => {
            Ok(DataTypes::wrap(DataTypes::Int(lhs.to_int().sub(rhs.to_int()))))
        },
        _ => Ok(DataTypes::wrap(DataTypes::Bool(false)))
    }
}
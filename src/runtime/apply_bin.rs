use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::operator_types::OperatorTypes;
use std::cell::RefCell;
use std::ops::{Add, Sub};
use std::rc::Rc;

pub fn apply_bin(
    op: &OperatorTypes,
    left: Rc<RefCell<DataTypes>>,
    right: Rc<RefCell<DataTypes>>,
) -> IReturn {
    let mut lhs = left.borrow_mut();
    let mut rhs = right.borrow_mut();

    let lhs = &mut *lhs;
    let rhs = &mut *rhs;

    match op {
        OperatorTypes::AddAssign => {
            *lhs = DataTypes::Int(lhs.to_int().add(rhs.to_int()));
            Ok(DataTypes::null())
        },
        OperatorTypes::SubAssign => {
            *lhs = DataTypes::Int(lhs.to_int().sub(rhs.to_int()));
            Ok(DataTypes::null())
        },
        OperatorTypes::Assign => {
            *lhs = rhs.clone();
            Ok(DataTypes::null())
        },
        OperatorTypes::Equals => Ok(DataTypes::wrap(DataTypes::Bool(lhs.is_equal(rhs)))),
        OperatorTypes::Add => {
            if !lhs.is_int() || !rhs.is_int() {
                let mut str = lhs.to_string();
                str.push_str(&rhs.to_string());
                Ok(DataTypes::wrap(DataTypes::Text(str)))
            } else {
                Ok(DataTypes::wrap(DataTypes::Int(
                    lhs.to_int().add(rhs.to_int()),
                )))
            }
        }
        OperatorTypes::Sub => Ok(DataTypes::wrap(DataTypes::Int(
            lhs.to_int().sub(rhs.to_int()),
        ))),
        OperatorTypes::NotEquals => Ok(DataTypes::wrap(DataTypes::Bool(!lhs.is_equal(rhs)))),
        _ => Ok(DataTypes::wrap(DataTypes::Bool(false))),
    }
}

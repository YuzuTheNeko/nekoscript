use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::operator_types::OperatorTypes;
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

pub fn apply_bin(
    op: &OperatorTypes,
    left: Rc<RefCell<DataTypes>>,
    right: Rc<RefCell<DataTypes>>,
) -> IReturn {
    let mut lhs = left.borrow_mut();
    let mut rhs = right.borrow_mut();

    match op {
        OperatorTypes::Lesser => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            Ok(
                DataTypes::wrap(
                    DataTypes::Bool(lhs.to_int().lt(rhs.to_int()))
                )
            )
        },
        OperatorTypes::LesserEq => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            Ok(
                DataTypes::wrap(
                    DataTypes::Bool(lhs.to_int().le(rhs.to_int()))
                )
            )
        }
        OperatorTypes::Greater => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            Ok(
                DataTypes::wrap(
                    DataTypes::Bool(lhs.to_int().gt(rhs.to_int()))
                )
            )
        }
        OperatorTypes::GreaterEq => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            Ok(
                DataTypes::wrap(
                    DataTypes::Bool(lhs.to_int().ge(rhs.to_int()))
                )
            )
        }
        OperatorTypes::AddAssign => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            *lhs = DataTypes::Int(lhs.to_int().add(rhs.to_int()));
            Ok(DataTypes::null())
        },
        OperatorTypes::SubAssign => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            *lhs = DataTypes::Int(lhs.to_int().sub(rhs.to_int()));
            Ok(DataTypes::null())
        },
        OperatorTypes::Assign => {
            *lhs = (*rhs).clone();

            Ok(DataTypes::null())
        },
        OperatorTypes::Equals => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            Ok(DataTypes::wrap(DataTypes::Bool(lhs.is_equal(rhs))))
        },
        OperatorTypes::Add => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            if !lhs.is_int() || !rhs.is_int() {
                let mut str = lhs.to_string();
                str.push_str(&rhs.to_string());
                Ok(DataTypes::wrap(DataTypes::Text(str)))
            } else {
                let lhs = &mut *lhs;
                let rhs = &mut *rhs;

                Ok(DataTypes::wrap(DataTypes::Int(
                    lhs.to_int().add(rhs.to_int()),
                )))
            }
        }
        OperatorTypes::Multi => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            Ok(DataTypes::wrap(DataTypes::Int(
                lhs.to_int().mul(rhs.to_int())
            )))
        }
        OperatorTypes::Div => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            Ok(DataTypes::wrap(DataTypes::Int(
                lhs.to_int().div(rhs.to_int())
            )))
        }
        OperatorTypes::Sub => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            Ok(DataTypes::wrap(DataTypes::Int(
                lhs.to_int().sub(rhs.to_int()),
            )))
        }
        OperatorTypes::NotEquals => {
            let lhs = &mut *lhs;
            let rhs = &mut *rhs;

            Ok(DataTypes::wrap(DataTypes::Bool(!lhs.is_equal(rhs))))
        },
        _ => Ok(DataTypes::wrap(DataTypes::Bool(false))),
    }
}

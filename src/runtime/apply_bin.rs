use std::ops::{Add, Sub};
use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::operator_types::OperatorTypes;

pub fn apply_bin<'a>(op: &OperatorTypes, left: &DataTypes, right: &DataTypes) -> IReturn<'a> {
    match op {
        OperatorTypes::Equals => {
            Ok(&DataTypes::Bool(left.is_equal(&right)))
        },
        OperatorTypes::Add => {
            if !left.is_int() || !right.is_int() {
                let mut str = left.to_string();
                str.push_str(&right.to_string());
                Ok(&DataTypes::Text(str))
            } else {
                Ok(&DataTypes::Int(left.to_int().add(right.to_int())))
            }
        },
        OperatorTypes::Sub => {
            Ok(&DataTypes::Int(left.to_int().sub(right.to_int())))
        },
        OperatorTypes::Assign => {
            panic!("Unsupported")
        }
        _ => Ok(&DataTypes::Bool(false))
    }
}
use std::ops::{Add, AddAssign, Rem, RemAssign, Sub};
use crate::core::data_types::DataTypes;
use crate::core::interpreter::{Interpreter, IReturn};
use crate::core::nodes::Nodes;
use crate::core::operator_types::SpecialOperatorTypes;
use crate::core::return_types::ReturnTypes;
use crate::core::scope::Scope;

pub fn resolve_special_assignment(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let (op, key, val) = node.to_special_assignment();

    match itr.execute(scope, val) {
        Ok(mut value) => {
            let mut var = scope.variables.write().unwrap();
            let mut var = var.get(key);

            match var {
                None => Err(ReturnTypes::RuntimeError(format!("Attempted to read unexisting variable {}", key))),
                Some(mut var) => {
                    let mut v = var.borrow_mut();
                    let v = &mut *v;

                    let mut other = value.borrow_mut();
                    let other = &mut *other;

                    match op {
                        SpecialOperatorTypes::Assign => {
                            *v = other.clone();
                            Ok(DataTypes::null())
                        }
                        SpecialOperatorTypes::AddAssign => {
                            *v = DataTypes::Int(v.to_int().add(other.to_int()));
                            Ok(DataTypes::null())
                        }
                        SpecialOperatorTypes::SubAssign => {
                            *v = DataTypes::Int(v.to_int().rem(other.to_int()));
                            Ok(DataTypes::null())
                        }
                    }
                }
            }
        },
        Err(e) => Err(e)
    }
}
use std::ops::Deref;
use crate::core::data_types::DataTypes;
use crate::core::interpreter::{Interpreter, IReturn};
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;

pub fn resolve_variable(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let (name, value) = node.to_variable();

    match itr.execute(scope, value) {
        Ok(val) => {
            scope.set(name.clone(), val);
            Ok(DataTypes::null())
        },
        Err(e) => Err(e)
    }
}
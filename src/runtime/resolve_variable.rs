use crate::core::data_types::DataTypes;
use crate::core::interpreter::{IReturn, Interpreter};
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;
use std::ops::Deref;

pub fn resolve_variable(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let (name, value) = node.to_variable();

    match itr.execute(scope, value) {
        Ok(val) => {
            scope.set(name.clone(), val);
            Ok(DataTypes::null())
        }
        Err(e) => Err(e),
    }
}

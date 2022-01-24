use crate::core::data_types::DataTypes;
use crate::core::interpreter::{Interpreter, IReturn};
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;

pub fn resolve_variable<'a>(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn<'a> {
    let (name, value) = node.to_variable();

    let got = match itr.execute(scope, value) {
        Ok(val) => {
            scope.set(name.clone(), val.clone());
            Ok(&DataTypes::null())
        },
        Err(e) => Err(e)
    };

    got
}
use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::{Node, Nodes};
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_scope(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let n = node.value.to_scope();

    let scope = Scope::from(scope);

    for i in n.iter() {
        match itr.execute(&scope, i) {
            Err(e) => return Err(e),
            _ => {}
        }
    }

    Ok(DataTypes::null())
}

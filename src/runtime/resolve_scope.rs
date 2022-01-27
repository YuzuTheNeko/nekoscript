use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_scope(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let n = node.to_scope();

    for i in n.iter() {
        match itr.execute(scope, i) {
            Err(e) => return Err(e),
            _ => {}
        }
    }

    Ok(DataTypes::null())
}

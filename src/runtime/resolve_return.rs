use crate::core::interpreter::IReturn;
use crate::core::nodes::Nodes;
use crate::core::return_types::ReturnTypes;
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_return(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let data = node.to_return();

    match itr.execute(scope, data) {
        Ok(res) => Err(ReturnTypes::Return(res)),
        Err(d) => Err(d)
    }
}
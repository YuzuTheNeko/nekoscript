use crate::core::interpreter::IReturn;
use crate::core::nodes::{Node, Nodes};
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::Interpreter;
use std::cell::RefCell;
use std::rc::Rc;

pub fn resolve_keyword(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let n = node.value.to_kw();

    match scope.variables.read().unwrap().get(n) {
        Some(val) => Ok(val.clone()),
        None => Err(RuntimeError(node.display(
            &format!(
                "Attempted to read undefined variable {}",
                n
            )
        ))),
    }
}

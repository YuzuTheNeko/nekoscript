use std::cell::RefCell;
use std::rc::Rc;
use crate::core::interpreter::IReturn;
use crate::core::nodes::Nodes;
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_keyword(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let n = node.to_kw();

    match scope.variables.read().unwrap().get(n) {
        Some(val) => {
            Ok(val.clone())
        },
        None => Err(RuntimeError(format!("Attempted to read undefined variable {}", n)))
    }
}
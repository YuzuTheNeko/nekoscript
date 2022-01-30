use crate::core::interpreter::IReturn;
use crate::core::nodes::{Node, Nodes};
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::runtime::call_function::call_function;
use crate::Interpreter;

pub fn resolve_dyn_call(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let (left, args) = node.value.to_dyn_fn_call();
    match itr.execute(scope, left) {
        Ok(val) => {
            let val = val.borrow();
            if !val.is_dyn_fn() {
                return Err(RuntimeError(left.display(&format!("Attempted to call a non dynamic function"))))
            }

            let val = val.to_dyn_fn();
            call_function(itr, scope, args, val.0, val.1)
        },
        Err(e) => return Err(e)
    }
}

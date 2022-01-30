use crate::core::interpreter::IReturn;
use crate::core::nodes::{Node, Nodes};
use crate::core::scope::Scope;
use crate::runtime::call_function::call_function;
use crate::Interpreter;

pub fn resolve_dyn_call(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let (params, args, body) = node.value.to_dyn_fn_call();
    call_function(itr, scope, args, params, body)
}

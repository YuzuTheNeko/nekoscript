use crate::core::interpreter::IReturn;
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;
use crate::Interpreter;
use crate::runtime::call_function::call_function;

pub fn resolve_dyn_call(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let (params, args, body) = node.to_dyn_fn_call();
    call_function(itr, scope, args, params, body)
}
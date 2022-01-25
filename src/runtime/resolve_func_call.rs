use crate::core::interpreter::{Interpreter, IReturn};
use crate::core::nodes::Nodes;
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::runtime::call_native_fn::call_native_fn;

pub fn resolve_func_call(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let (name, args, func) = node.to_fn_c();

    if name.is_some() {
        let name = name.as_ref().unwrap();
        if scope.is_def_fn(&name) {
            unimplemented!()
        } else if scope.is_native_fn(&name) {
            call_native_fn(itr, scope, node)
        } else {
            Err(RuntimeError(format!("Attempted to call undefined function {}", name)))
        }
    } else {
        let func = func.as_ref().unwrap();

        Err(RuntimeError(format!("These functions are not supported.")))
    }
}
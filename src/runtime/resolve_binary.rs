use crate::core::interpreter::{IReturn, Interpreter};
use crate::core::nodes::{Node, Nodes};
use crate::core::scope::Scope;
use crate::runtime::apply_bin::apply_bin;

pub fn resolve_binary(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let (op, left, right) = node.value.to_bin();

    match itr.execute(scope, left) {
        Ok(left) => match itr.execute(scope, right) {
            Ok(right) => apply_bin(op, left, right),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

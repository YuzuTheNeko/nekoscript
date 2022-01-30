
use crate::core::interpreter::IReturn;
use crate::core::nodes::{Node, Nodes};
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_ternary(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let (condition, when1, when2) = node.value.to_ternary();

    match itr.execute(scope, condition) {
        Ok(val) => {
            let got = val.borrow();
            if got.is_truthy() {
                itr.execute(scope, when1)
            } else {
                itr.execute(scope, when2)
            }
        },
        Err(e) => Err(e)
    }
}
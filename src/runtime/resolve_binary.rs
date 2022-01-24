use crate::core::interpreter::{Interpreter, IReturn};
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;
use crate::runtime::apply_bin::apply_bin;

pub fn resolve_binary<'a>(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn<'a> {
    let (op, left, right) = node.to_bin();

    match itr.execute(scope, left) {
        Ok(left) => {
            match itr.execute(scope, right) {
                Ok(right) => {
                    apply_bin(
                        op,
                        left,
                        right
                    )
                },
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}
use crate::core::interpreter::IReturn;
use crate::core::nodes::{Node, Nodes};
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::Interpreter;
use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::{DataTypes, NativeFnType};

pub fn resolve_keyword(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let n = node.value.to_kw();

    match scope.variables.read().unwrap().get(n) {
        Some(val) => Ok(val.clone()),
        None => {
            {
                let borrow = scope.process.borrow();
                if let Some(f) = borrow.native.get(n) {
                    return Ok(
                        DataTypes::wrap(
                            DataTypes::NativeFn {
                                params: None,
                                name: n.to_string(),
                                module: None,
                                kind: NativeFnType::Global
                            }
                        )
                    )
                }
            }

            Err(RuntimeError(node.display(
                &format!(
                    "Attempted to read undefined variable {}",
                    n
                )
            )))
        },
    }
}

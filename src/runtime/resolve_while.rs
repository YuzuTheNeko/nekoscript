use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::{Node, Nodes};
use crate::core::return_types::ReturnTypes;
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_while(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let (condition, code) = node.value.to_while();

    loop {
        match itr.execute(scope, condition) {
            Ok(val) => {
                let val = val.borrow();
                if !val.to_bool().eq(&true) {
                    break;
                } else {
                    match itr.execute(scope, code) {
                        Err(e) => return Err(e),
                        _ => {}
                    }
                }
            }
            Err(e) => match e {
                ReturnTypes::Break => return Ok(DataTypes::null()),
                _ => return Err(e)
            },
        }
    }

    Ok(DataTypes::null())
}

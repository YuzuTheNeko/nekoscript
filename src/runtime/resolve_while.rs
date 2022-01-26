use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_while(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let (condition, code) = node.to_while();

    loop {
        match itr.execute(scope, condition) {
            Ok(val) => {
                let val = val.borrow();
                if !val.to_bool().eq(&true) {
                    break
                } else {
                    match itr.execute(scope, code) {
                        Err(e) => return Err(e),
                        _ => {}
                    }
                }
            },
            Err(e) => return Err(e)
        }
    }

    Ok(DataTypes::null())
}
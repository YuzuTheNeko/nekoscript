use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_if(
    itr: &Interpreter,
    scope: &Scope,
    node: &Nodes
) -> IReturn {
    let (
        condition,
        when_true,
        when_false,
        races
    ) = node.to_if();

    match itr.execute(scope, condition) {
        Ok(val) => {
            let v = val.borrow();

            if v.is_truthy() {
                match itr.execute(scope, when_true) {
                    Err(e) => return Err(e),
                    _ => {}
                }
            } else {
                let mut one = false;

                for (condition, node) in races.iter() {
                    match itr.execute(scope, condition) {
                        Ok(v) => {
                            let val = v.borrow();

                            if val.is_truthy() {
                                match itr.execute(scope, node) {
                                    Err(e) => return Err(e),
                                    _ => {
                                        one = true;
                                        break;
                                    }
                                }
                            }
                        }
                        Err(e) => return Err(e)
                    }
                }

                if !one && when_false.is_some() {
                    let when_false = when_false.as_ref().unwrap();
                    match itr.execute(scope, when_false) {
                        Err(e) => return Err(e),
                        _ => {}
                    }
                }
            }

            Ok(DataTypes::null())
        }
        Err(e) => Err(e)
    }
}
use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::Nodes;
use crate::core::return_types::ReturnTypes;
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::Interpreter;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLockWriteGuard;

pub fn call_function(
    itr: &Interpreter,
    scope: &Scope,
    args: &Vec<Box<Nodes>>,
    params: &Vec<String>,
    body: &Box<Nodes>,
) -> IReturn {
    let mut y = 0;

    {
        let mut writer = scope.variables.write().unwrap();

        for i in params {
            let arg: Option<&Box<Nodes>> = args.get(y);

            if !arg.is_some() {
                return Err(RuntimeError("Function is missing parameters.".to_owned()));
            }

            let arg = arg.unwrap();

            match itr.execute(scope, arg) {
                Ok(val) => {
                    writer.insert(i.to_string(), val);
                    y += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }

    match itr.execute(scope, body) {
        Ok(val) => Ok(DataTypes::null()),
        Err(e) => match e {
            ReturnTypes::Return(val) => Ok(val),
            _ => Err(e),
        },
    }
}

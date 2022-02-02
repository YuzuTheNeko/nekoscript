use crate::core::data_types::DataTypes;
use crate::core::native_function::{PropType, PrototypeNativeFunction};
use crate::core::return_types::ReturnTypes;
use crate::core::scope::Scope;

pub fn map() -> PrototypeNativeFunction {
    PrototypeNativeFunction {
        kind: PropType::Method,
        body: Box::new(| itr, scope, this, args | {
            {
                let arr = this.to_array();
                let arr = arr.read().unwrap();

                let arg = args.get(0).unwrap();
                let arg = arg.borrow();

                let (params, body) = arg.to_dyn_fn();

                let param_name = params.get(0).cloned().unwrap_or(String::new());
                let param_name2 = params.get(1).cloned().unwrap_or(String::new());

                let scope = Scope::from(scope);

                let mut y = 0;

                for i in arr.iter() {
                    if !param_name.is_empty() {
                        {
                            let mut writer = scope.variables.write().unwrap();
                            writer.insert(param_name.to_string(), i.clone());
                        }
                    }

                    if !param_name2.is_empty() {
                        {
                            let mut writer = scope.variables.write().unwrap();
                            writer.insert(param_name2.to_string(), DataTypes::wrap(DataTypes::Int(y)));
                        }
                    }

                    match itr.execute(&scope, body) {
                        Err(e) => match e {
                            ReturnTypes::Break => break,
                            ReturnTypes::Return(val) => {
                                let mut lhs = i.borrow_mut();
                                let mut rhs = val.borrow_mut();

                                *lhs = (*rhs).clone();

                                y += 1;
                            },
                            _ => return Err(e)
                        },
                        _ => {
                            y += 1;
                        }
                    }
                }
            }

            Ok(DataTypes::null())
        })
    }
}
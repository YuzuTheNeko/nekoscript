
use crate::core::data_types::{DataTypes, NativeFnType};
use crate::core::interpreter::{IReturn, Interpreter};
use crate::core::nodes::{Node, Nodes};
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::runtime::call_function::call_function;
use crate::runtime::internals::call_native_fn::call_native_fn;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::core::native_function::NativeFunction;
use crate::native::modules::get_module_hash::get_module_hash;
use crate::runtime::internals::resolve_native_call::resolve_native_call;
use crate::runtime::internals::resolve_native_module_call::resolve_native_module_call;

pub fn resolve_func_call(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let (name, args, func) = node.value.to_fn_c();

    if name.is_some() {
        let name = name.as_ref().unwrap();

        if scope.is_def_fn(&name) {
            let mut node: Option<Node> = None;

            {
                let reader = scope.functions.read().unwrap();
                let got = reader.get(&name.to_string()).unwrap();
                node = Some(got.clone());
            }

            let node = node.unwrap();

            let (name, params, body) = node.value.to_fn_def();

            call_function(itr, scope, args, params, body)
        } else {
            if let Some(v) = resolve_native_call(
                itr,
                scope,
                args,
                name.clone()
            ) {
                return v
            }

            let mut val: Option<Rc<RefCell<DataTypes>>> = None;

            {
                let writer = scope.variables.read().unwrap();
                let var = writer.get(&name.to_string());
                if !var.is_some() {
                    return Err(RuntimeError(format!(
                        "Attempted to call undefined function {}",
                        name
                    )));
                }
                val = Some(var.unwrap().clone());
            }

            let val = val.unwrap();

            {
                let val = val.borrow();
                if !val.is_dyn_fn() && !val.is_native_fn() {
                    Err(RuntimeError(format!("{} is not a function", name)))
                } else {
                    if val.is_native_fn() {
                        return resolve_native_module_call(
                            itr,
                            scope,
                            val,
                            args
                        )
                    }

                    let val = val.to_dyn_fn();
                    call_function(itr, scope, args, val.0, val.1)
                }
            }
        }
    } else {
        let func = func.as_ref().unwrap();

        Err(RuntimeError(format!("These functions are not supported.")))
    }
}

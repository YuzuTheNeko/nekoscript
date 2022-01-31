
use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::{DataTypes, NativeFnType};
use crate::core::interpreter::IReturn;
use crate::core::native_function::PropType;
use crate::core::nodes::{Accessor, Node, Nodes, Position};
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::Interpreter;
use crate::native::get_prototypes::get_prototypes;
use crate::runtime::call_function::call_function;
use crate::runtime::internals::call_native_fn::call_native_fn;
use crate::runtime::internals::resolve_native_module_call::resolve_native_module_call;
use crate::runtime::resolve_func_call::resolve_func_call;
use crate::runtime::resolve_params::resolve_params;

pub fn resolve_object_accessor(itr: &Interpreter, scope: &Scope, old: &Node) -> IReturn {
    let node = old.value.to_obj_accessor();

    match itr.execute(scope, node.0) {
        Ok(mut val) => {
            for i in node.1 {
                let mut next: Option<Rc<RefCell<DataTypes>>> = None;

                match i {
                    Accessor::Property(name) => {
                        let mut borrow = val.borrow_mut();
                        if !borrow.is_obj() {
                            let protos = get_prototypes(borrow.kind());

                            let native = protos.get(name);

                            if !native.is_some() {
                                return Ok(DataTypes::null())
                            }

                            let native = native.unwrap();

                            if native.kind.eq(&PropType::Method) {
                                return Ok(DataTypes::wrap(DataTypes::Text(format!("define {}(scope, args) {{ [ native code ] }};", name))))
                            }

                            match (native.body)(scope, borrow, &vec![]) {
                                Ok(v) => {
                                    next = Some(v)
                                },
                                Err(e) => return Err(e)
                            }
                        } else {
                            let mut borrow = borrow.to_object();
                            let mut borrow = borrow.write().unwrap();

                            if !borrow.contains_key(name) {
                                let dt = DataTypes::null();
                                borrow.insert(name.to_string(), dt.clone());
                                return Ok(dt)
                            }

                            let got = borrow.get(name).unwrap();

                            next = Some(got.clone())
                        }
                    },
                    Accessor::Method(name, params) => {
                        let mut borrow = val.borrow_mut();
                        if !borrow.is_obj() {
                            let protos = get_prototypes(borrow.kind());

                            let got = protos.get(name);

                            if !got.is_some() {
                                return Err(RuntimeError(old.display(&format!("Attempted to call nulled function {}", name))))
                            }

                            let got = got.unwrap();

                            if got.kind.eq(&PropType::Property) {
                                return Err(RuntimeError(old.display(&format!("Attempted to call non function {}", name))))
                            }

                            match resolve_params(itr, scope, params) {
                                Ok(params) => match (got.body)(scope, borrow, &params) {
                                    Ok(e) => next = Some(e),
                                    Err(e) => return Err(e)
                                },
                                Err(e) => return Err(e)
                            }
                        } else {
                            let mut borrow = borrow.to_object();
                            let mut borrow = borrow.read().unwrap();

                            if !borrow.contains_key(name) {
                                return Err(RuntimeError(old.display(&format!("Property {} does not exist", name))))
                            }

                            let mut res = borrow.get(name).unwrap();

                            let res = res.borrow();

                            if !res.is_dyn_fn() && !res.is_native_fn() {
                                return Err(RuntimeError(old.display(&format!("Property {} is not a function", name))))
                            }

                            if res.is_native_fn() {
                                let data = res.to_native_fn();

                                if data.1.eq(&NativeFnType::Module) {
                                    return resolve_native_module_call(
                                        itr,
                                        scope,
                                        res,
                                        &params.iter().map(| n | Box::new(n.clone())).collect()
                                    )
                                }
                            }

                            let data = res.to_dyn_fn();

                            let mut vc: Vec<Box<Node>> = vec![];

                            for i in params {
                                vc.push(Box::new(i.clone()))
                            }

                            match call_function(itr, scope, &vc, data.0, data.1) {
                                Ok(val) => {
                                    next = Some(val)
                                },
                                Err(e) => return Err(e)
                            }
                        }
                    }
                }

                val = next.unwrap();
            }

            Ok(val)
        },
        Err(e) => Err(e)
    }
}
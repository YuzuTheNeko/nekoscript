
use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::{Accessor, Node, Nodes, Position};
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::Interpreter;
use crate::runtime::call_function::call_function;

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
                            return Ok(DataTypes::null())
                        }

                        let mut borrow = borrow.to_obj();
                        let mut borrow = borrow.write().unwrap();

                        if !borrow.contains_key(name) {
                            let dt = DataTypes::null();
                            borrow.insert(name.to_string(), Nodes::create(Nodes::Value(dt.clone()), Position::default()));
                            return Ok(dt)
                        }

                        let got = borrow.get(name).unwrap();

                        match itr.execute(scope, got) {
                            Ok(mut v) => {
                                next = Some(v)
                            },
                            Err(e) => return Err(e)
                        }
                    },
                    Accessor::Method(name, params) => {
                        let mut borrow = val.borrow_mut();
                        if !borrow.is_obj() {
                            return Ok(DataTypes::null())
                        }

                        let mut borrow = borrow.to_obj();
                        let mut borrow = borrow.read().unwrap();

                        if !borrow.contains_key(name) {
                            return Err(RuntimeError(format!("Property {} does not exist", name)))
                        }

                        let mut borrow = borrow.get(name).unwrap();

                        match itr.execute(scope, borrow) {
                            Ok(res) => {
                                let res = res.borrow();

                                if !res.is_dyn_fn() {
                                    return Err(RuntimeError(format!("Property {} is not a function", name)))
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
                            },
                            Err(e) => return Err(e)
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
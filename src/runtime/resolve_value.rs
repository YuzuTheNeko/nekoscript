
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::RwLock;
use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::{Node, Nodes, Position};
use crate::core::scope::Scope;
use crate::Interpreter;
use crate::runtime::resolve_dyn_call::resolve_dyn_call;

pub fn resolve_value(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    if node.value.is_dyn_call() {
        return match itr.execute(scope, node) {
            Ok(v) => resolve_value(itr, scope, &Nodes::create(
                Nodes::Value(v),
                node.pos.clone()
            )),
            Err(e) => Err(e)
        }
    }

    let val = node.value.to_value();

    let v = val.borrow();

    match v.deref() {
        DataTypes::RawArray(v) => {
            let mut vc = vec![];

            let v = v.read().unwrap();

            for i in v.iter() {
                match itr.execute(scope, &i) {
                    Ok(v) => {
                        vc.push(v)
                    },
                    Err(e) => return Err(e)
                }
            }

            Ok(DataTypes::wrap(DataTypes::Array(Rc::new(RwLock::new(vc)))))
        }
        DataTypes::RawObject(obj) => {
            let obj = obj.read().unwrap();

            let mut newer = HashMap::new();

            for (key, value) in obj.iter() {
                match itr.execute(scope, &value) {
                    Ok(v) => {
                        newer.insert(key.clone(), v.clone());
                    }
                    Err(e) => return Err(e)
                }
            }

            Ok(DataTypes::wrap(DataTypes::Object(Rc::new(RwLock::new(newer)))))
        }
        _ => Ok(val.clone())
    }
}
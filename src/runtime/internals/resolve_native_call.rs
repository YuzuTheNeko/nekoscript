use std::cell::RefCell;
use std::rc::Rc;
use crate::core::interpreter::IReturn;
use crate::core::native_function::NativeFunction;
use crate::core::nodes::Node;
use crate::core::scope::Scope;
use crate::Interpreter;
use crate::runtime::internals::call_native_fn::call_native_fn;

pub fn resolve_native_call(
    itr: &Interpreter,
    scope: &Scope,
    args: &Vec<Box<Node>>,
    name: String
) -> Option<IReturn> {
    let mut res: Option<Rc<RefCell<NativeFunction>>> = None;

    {
        let borrow = scope.process.borrow();
        if borrow.native.contains_key(&name) {
            let f = borrow.native.get(&name).unwrap();
            res = Some(f.clone())
        }
    }

    if let Some(f) = res {
        let f = f.borrow();
        return Some(call_native_fn(itr, scope, args, &*f))
    }

    None
}
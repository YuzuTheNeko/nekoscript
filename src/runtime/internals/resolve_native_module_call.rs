use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::core::data_types::{DataTypes, NativeFnType};
use crate::core::interpreter::IReturn;
use crate::core::native_function::NativeFunction;
use crate::core::nodes::Node;
use crate::core::scope::Scope;
use crate::Interpreter;
use crate::native::modules::get_module_hash::get_module_hash;
use crate::runtime::internals::call_native_fn::call_native_fn;

pub fn resolve_native_module_call(itr: &Interpreter, scope: &Scope, val: Ref<DataTypes>, args: &Vec<Box<Node>>) -> IReturn {
    let val = val.to_native_fn();
    if val.1.eq(&NativeFnType::Global) {
        let mut res: Option<Rc<RefCell<NativeFunction>>> = None;

        {
            let borrow = scope.process.borrow();
            let f = borrow.native.get(val.0).unwrap();
            res = Some(f.clone())
        }

        let f = res.unwrap();
        let f = f.borrow();

        return call_native_fn(itr, scope, args, &*f)
    } else {
        let got = val.2.as_ref().unwrap();

        let module = get_module_hash(got.clone());
        let f = module.get(val.0).unwrap();
        return call_native_fn(itr, scope, args, f)
    }
}
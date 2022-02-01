use crate::core::interpreter::{IReturn, Interpreter};
use crate::core::native_function::NativeFunction;
use crate::core::nodes::{Node, Nodes};
use crate::core::scope::Scope;

pub fn call_native_fn(itr: &Interpreter, scope: &Scope, args: &Vec<Box<Node>>, f: &NativeFunction) -> IReturn {
    let mut params = vec![];

    for i in args.iter() {
        match itr.execute(scope, i) {
            Ok(val) => params.push(val),
            Err(e) => return Err(e),
        }
    }

    f.0(itr, scope, &params)
}

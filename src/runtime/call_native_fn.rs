use crate::core::interpreter::{Interpreter, IReturn};
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;

pub fn call_native_fn(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let (name, args, _) = node.to_fn_c();
    let name = name.as_ref().unwrap();

    let f = scope.native.iter().find(| n | n.name.eq(name)).unwrap();

    let mut params = vec![];

    for i in args.iter() {
        match itr.execute(scope, i) {
            Ok(val) => {
                params.push(val)
            },
            Err(e) => return Err(e)
        }
    }

    ((*f).body)(scope, &params)
}
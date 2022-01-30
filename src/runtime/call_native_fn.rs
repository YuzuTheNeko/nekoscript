use crate::core::interpreter::{IReturn, Interpreter};
use crate::core::nodes::{Node, Nodes};
use crate::core::scope::Scope;

pub fn call_native_fn(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let (name, args, _) = node.value.to_fn_c();
    let name = name.as_ref().unwrap();

    let f = scope.native.iter().find(|n| n.name.eq(name)).unwrap();

    let mut params = vec![];

    for i in args.iter() {
        match itr.execute(scope, i) {
            Ok(val) => params.push(val),
            Err(e) => return Err(e),
        }
    }

    ((*f).body)(scope, &params)
}

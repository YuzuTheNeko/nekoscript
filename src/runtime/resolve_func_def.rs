use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::{Node, Nodes};
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_func_def(itr: &Interpreter, scope: &Scope, node: &Node) -> IReturn {
    let (name, _, _) = node.value.to_fn_def();

    {
        let mut writer = scope.functions.write().unwrap();
        writer.insert(name.to_string(), node.clone());
    }

    Ok(DataTypes::null())
}

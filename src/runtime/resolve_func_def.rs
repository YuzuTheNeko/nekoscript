use crate::core::data_types::DataTypes;
use crate::core::interpreter::IReturn;
use crate::core::nodes::Nodes;
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_func_def(itr: &Interpreter, scope: &Scope, node: &Nodes) -> IReturn {
    let (name, _, _) = node.to_fn_def();

    {
        let mut writer = scope.functions.write().unwrap();
        writer.insert(name.to_string(), node.clone());
    }

    Ok(DataTypes::null())
}

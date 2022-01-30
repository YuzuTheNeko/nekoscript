use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::DataTypes;
use crate::core::nodes::Node;
use crate::core::return_types::ReturnTypes;
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_params(itr: &Interpreter, scope: &Scope, params: &Vec<Node>) -> Result<Vec<Rc<RefCell<DataTypes>>>, ReturnTypes> {
    let mut vc = vec![];

    for i in params {
        match itr.execute(scope, i) {
            Ok(v) => vc.push(v),
            Err(e) => return Err(e)
        }
    }

    Ok(vc)
}
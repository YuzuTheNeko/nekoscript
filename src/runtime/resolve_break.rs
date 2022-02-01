use crate::core::interpreter::IReturn;
use crate::core::return_types::ReturnTypes;
use crate::core::scope::Scope;
use crate::Interpreter;

pub fn resolve_break(itr: &Interpreter, scope: &Scope) -> IReturn {
    Err(ReturnTypes::Break)
}
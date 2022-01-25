use std::cell::RefCell;
use std::rc::Rc;
use crate::core::data_types::DataTypes;
use crate::core::nodes::Nodes;
use crate::core::return_types::ReturnTypes;
use crate::core::scope::Scope;
use crate::runtime::resolve_binary::resolve_binary;
use crate::runtime::resolve_func_call::resolve_func_call;
use crate::runtime::resolve_if::resolve_if;
use crate::runtime::resolve_keyword::resolve_keyword;
use crate::runtime::resolve_scope::resolve_scope;
use crate::runtime::resolve_special_assignment::resolve_special_assignment;
use crate::runtime::resolve_variable::resolve_variable;

pub struct Interpreter {
    pub nodes: Vec<Nodes>
}

pub type IReturn= Result<Rc<RefCell<DataTypes>>, ReturnTypes>;

impl Interpreter {
    pub fn run(&self) {
        let scope = Scope::new();

        for node in self.nodes.iter() {
            match self.execute(&scope, &Box::new(node)) {
                Err(err) => match err {
                    ReturnTypes::RuntimeError(str) => {
                        panic!("{}", str);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn new(nodes: Vec<Nodes>) -> Self {
        Self {
            nodes
        }
    }

    pub fn execute(&self, scope: &Scope, node: &Nodes) -> IReturn {
        match node {
            Nodes::VariableDef { .. } => resolve_variable(self, scope, node),
            Nodes::FnCall { .. } => resolve_func_call(self, scope, node),
            Nodes::Keyword(_) => resolve_keyword(self, scope, node),
            Nodes::Scope { .. } => resolve_scope(self, scope, node),
            Nodes::BinaryExpr { .. } => resolve_binary(self, scope, node),
            Nodes::SpecialAssignment { .. } => resolve_special_assignment(self, scope, node),
            Nodes::Value(value) => Ok(value.clone()),
            Nodes::If { .. } => resolve_if(self, scope, node),
            _ => Ok(DataTypes::null())
        }
    }
}
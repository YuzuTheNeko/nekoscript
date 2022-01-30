use crate::core::data_types::DataTypes;
use crate::core::nodes::{Node, Nodes};
use crate::core::return_types::ReturnTypes;
use crate::core::scope::Scope;
use crate::runtime::resolve_binary::resolve_binary;
use crate::runtime::resolve_dyn_call::resolve_dyn_call;
use crate::runtime::resolve_func_call::resolve_func_call;
use crate::runtime::resolve_func_def::resolve_func_def;
use crate::runtime::resolve_if::resolve_if;
use crate::runtime::resolve_keyword::resolve_keyword;
use crate::runtime::resolve_scope::resolve_scope;
use crate::runtime::resolve_variable::resolve_variable;
use crate::runtime::resolve_while::resolve_while;
use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::resolve_object_accessor::resolve_object_accessor;
use crate::runtime::resolve_return::resolve_return;
use crate::runtime::resolve_ternary::resolve_ternary;
use crate::runtime::resolve_value::resolve_value;

pub struct Interpreter {
    pub path: String,
    pub nodes: Vec<Node>,
}

pub type IReturn = Result<Rc<RefCell<DataTypes>>, ReturnTypes>;

impl Interpreter {
    pub fn run(&self) {
        let scope = Scope::new();

        for node in self.nodes.iter() {
            match self.execute(&scope, node) {
                Err(err) => match err {
                    ReturnTypes::RuntimeError(str) => {
                        println!("RuntimeError: {}\nFile: {}", str, self.path);
                        std::process::exit(0)
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn new(nodes: Vec<Node>, path: String) -> Self {
        Self {
            nodes,
            path
        }
    }

    pub fn execute(&self, scope: &Scope, node: &Node) -> IReturn {
        match &node.value {
            Nodes::ObjectAccessor { .. } => resolve_object_accessor(self, scope, node),
            Nodes::Return(_) => resolve_return(self, scope, node),
            Nodes::DynFnCall { .. } => resolve_dyn_call(self, scope, node),
            Nodes::VariableDef { .. } => resolve_variable(self, scope, node),
            Nodes::Ternary { .. } => resolve_ternary(self, scope, node),
            Nodes::FnCall { .. } => resolve_func_call(self, scope, node),
            Nodes::While { .. } => resolve_while(self, scope, node),
            Nodes::Keyword(_) => resolve_keyword(self, scope, node),
            Nodes::FnDef { .. } => resolve_func_def(self, scope, node),
            Nodes::Scope { .. } => resolve_scope(self, scope, node),
            Nodes::BinaryExpr { .. } => resolve_binary(self, scope, node),
            Nodes::Value(value) => resolve_value(self, scope, node),
            Nodes::If { .. } => resolve_if(self, scope, node),
            _ => Ok(DataTypes::null()),
        }
    }
}

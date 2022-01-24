use crate::core::data_types::DataTypes;
use crate::core::nodes::Nodes;
use crate::core::return_types::ReturnTypes;
use crate::core::scope::Scope;
use crate::runtime::resolve_variable::resolve_variable;

pub struct Interpreter {
    pub nodes: Vec<Nodes>
}

pub type IReturn<'a> = Result<&'a DataTypes, ReturnTypes>;

impl Interpreter {
    pub fn run(&self) {
        let scope = Scope::new();

        let got = scope.variables.read().unwrap().get("ok").unwrap();

        for node in self.nodes.iter() {
            match self.execute(&scope, node) {
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

    pub fn execute<'a>(&self, scope: &Scope, node: &Nodes) -> IReturn<'a> {
        match node {
            Nodes::VariableDef { .. } => resolve_variable(self, scope, node),
            _ => Ok(&DataTypes::null())
        }
    }
}
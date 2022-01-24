use crate::core::data_types::DataTypes;
use crate::core::operator_types::OperatorTypes;

pub enum Nodes {
    Keyword(String),
    Value(DataTypes),
    Operator(OperatorTypes),
    VariableDef {
        name: String,
        value: Box<Nodes>
    },
    FnDef {
        name: String,
        params: Vec<String>,
        body: Box<Nodes>
    },
    FnCall {
        name: String,
        args: Vec<Box<Nodes>>
    },
    BinaryExpr {
        op: OperatorTypes,
        left: Box<Nodes>,
        right: Box<Nodes>
    },
    Punc(String),
    Scope(Vec<Box<Nodes>>)
}

impl Nodes {
    pub fn is_op(&self) -> bool {
        match self {
            Nodes::Operator(_) => true,
            _ => false
        }
    }

    pub fn is_kw(&self) -> bool {
        match self {
            Nodes::Keyword(_) => true,
            _ => false
        }
    }
}

impl Nodes {
    pub fn to_op(&self) -> &OperatorTypes {
        match self {
            Nodes::Operator(op) => op,
            _ => panic!("Node not an operator.")
        }
    }

    pub fn to_bin(&self) -> (&OperatorTypes, &Box<Nodes>, &Box<Nodes>) {
        match self {
            Nodes::BinaryExpr { op, left, right } => (op, left, right),
            _ => panic!("Node not binary.")
        }
    }

    pub fn to_variable(&self) -> (&String, &Box<Nodes>) {
        match self {
            Nodes::VariableDef { name, value } => (name, value),
            _ => panic!("Node is not variable.")
        }
    }

    pub fn to_kw(&self) -> &String {
        match self {
            Nodes::Keyword(s) => s,
            _ => panic!("Node is not keyword.")
        }
    }
}
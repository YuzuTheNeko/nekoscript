use crate::constants::keywords::WHILE_KEYWORD;
use crate::core::data_types::DataTypes;
use crate::core::operator_types::{OperatorTypes};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub enum Accessor {
    Property(String),
    Method(String, Vec<Node>)
}

#[derive(Clone, Default)]
pub struct Position {
    pub line: usize,
    pub column: usize
}

#[derive(Clone)]
pub struct Node {
    pub value: Nodes,
    pub pos: Position
}

#[derive(Clone)]
pub enum Nodes {
    Ternary {
        when_true: Box<Node>,
        when_false: Box<Node>,
        condition: Box<Node>
    },
    Keyword(String),
    Value(Rc<RefCell<DataTypes>>),
    Operator(OperatorTypes),
    VariableDef {
        name: String,
        value: Box<Node>,
    },
    If {
        condition: Box<Node>,
        when_true: Box<Node>,
        when_false: Option<Box<Node>>,
        races: Vec<(Node, Node)>,
    },
    FnDef {
        name: String,
        params: Vec<String>,
        body: Box<Node>,
    },
    FnCall {
        name: Option<String>,
        func: Option<Box<Node>>,
        args: Vec<Box<Node>>,
    },
    ObjectAccessor {
        value: Box<Node>,
        accessors: Vec<Accessor>
    },
    BinaryExpr {
        op: OperatorTypes,
        left: Box<Node>,
        right: Box<Node>,
    },
    DynFnCall {
        left: Box<Node>,
        args: Vec<Box<Node>>,
    },
    Return(Box<Node>),
    While {
        condition: Box<Node>,
        scope: Box<Node>,
    },
    Punc(String),
    Scope(Vec<Node>),
}

impl Nodes {
    pub fn is_op(&self) -> bool {
        match self {
            Nodes::Operator(_) => true,
            _ => false,
        }
    }

    pub fn is_kw(&self) -> bool {
        match self {
            Nodes::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn is_dyn_fn(&self) -> bool {
        match self {
            Nodes::Value(v) => (*v.borrow()).is_dyn_fn(),
            _ => false,
        }
    }

    pub fn is_dyn_call(&self) -> bool {
        match self {
            Nodes::DynFnCall { .. } => true,
            _ => false
        }
    }

    pub fn is_if(&self) -> bool {
        match self {
            Nodes::If { .. } => true,
            _ => false,
        }
    }

    pub fn is_fn(&self) -> bool {
        match self {
            Nodes::FnDef { .. } => true,
            _ => false,
        }
    }
}

impl Nodes {
    pub fn kind(&self) -> &str {
        match self {
            Nodes::ObjectAccessor { .. } => "Accessor",
            Nodes::DynFnCall { .. } => "dyn fn call",
            Nodes::If { .. } => "If",
            Nodes::FnDef { .. } => "FnDef",
            Nodes::BinaryExpr { .. } => "BinExpr",
            Nodes::Scope(_) => "Scope",
            Nodes::Value(_) => "Value",
            Nodes::Return(_) => "Return",
            Nodes::FnCall { .. } => "FnCall",
            Nodes::Ternary { .. } => "Ternary",
            Nodes::While { .. } => WHILE_KEYWORD,
            Nodes::VariableDef { .. } => "VarDef",
            Nodes::Punc(_) => "Punc",
            Nodes::Operator(_) => "Op",
            Nodes::Keyword(_) => "Keyword",
        }
    }

    pub fn create(val: Nodes, pos: Position) -> Node {
        Node {
            value: val,
            pos
        }
    }
}

impl Nodes {
    pub fn to_ternary(&self) -> (&Box<Node>, &Box<Node>, &Box<Node>) {
        match self {
            Nodes::Ternary {
                condition,
                when_true,
                when_false
            } => (condition, when_true, when_false),
            _ => panic!("Node not ternary")
        }
    }

    pub fn to_value(&self) -> &Rc<RefCell<DataTypes>> {
        match self {
            Self::Value(v) => v,
            _ => panic!("Node not data type")
        }
    }
    pub fn to_op(&self) -> &OperatorTypes {
        match self {
            Nodes::Operator(op) => op,
            _ => panic!("Node not an operator."),
        }
    }

    pub fn to_obj_accessor(&self) -> (&Box<Node>, &Vec<Accessor>) {
        match self {
            Nodes::ObjectAccessor { value, accessors } => (value, accessors),
            _ => panic!("Node not object accessor.")
        }
    }

    pub fn to_return(&self) -> &Box<Node> {
        match self {
            Nodes::Return(v) => v,
            _ => panic!("Node not return")
        }
    }

    pub fn to_dyn_fn_call(&self) -> (&Box<Node>, &Vec<Box<Node>>) {
        match self {
            Nodes::DynFnCall { left, args } => (left, args),
            _ => panic!("Node not dyn call"),
        }
    }

    pub fn to_dyn_fn(&self) -> Rc<RefCell<DataTypes>> {
        match self {
            Nodes::Value(r) => r.clone(),
            _ => panic!("Node not dyn fn"),
        }
    }
    pub fn to_while(&self) -> (&Box<Node>, &Box<Node>) {
        match self {
            Nodes::While { condition, scope } => (condition, scope),
            _ => panic!("Node not while."),
        }
    }

    pub fn to_if(
        &self,
    ) -> (
        &Box<Node>,
        &Box<Node>,
        &Option<Box<Node>>,
        &Vec<(Node, Node)>,
    ) {
        match self {
            Nodes::If {
                condition,
                when_true,
                when_false,
                races,
            } => (condition, when_true, when_false, races),
            _ => panic!("Node not if."),
        }
    }

    pub fn to_scope(&self) -> &Vec<Node> {
        match self {
            Nodes::Scope(c) => c,
            _ => panic!("Node not a scope"),
        }
    }

    pub fn to_bin(&self) -> (&OperatorTypes, &Box<Node>, &Box<Node>) {
        match self {
            Nodes::BinaryExpr { op, left, right } => (op, left, right),
            _ => panic!("Node not binary."),
        }
    }

    pub fn to_variable(&self) -> (&String, &Box<Node>) {
        match self {
            Nodes::VariableDef { name, value } => (name, value),
            _ => panic!("Node is not variable."),
        }
    }

    pub fn to_fn_def(&self) -> (&String, &Vec<String>, &Box<Node>) {
        match self {
            Nodes::FnDef { name, params, body } => (name, params, body),
            _ => panic!("Node not fn def."),
        }
    }

    pub fn to_fn_c(&self) -> (&Option<String>, &Vec<Box<Node>>, &Option<Box<Node>>) {
        match self {
            Nodes::FnCall { name, args, func } => (name, args, func),
            _ => panic!("Node not func call."),
        }
    }

    pub fn to_kw(&self) -> &String {
        match self {
            Nodes::Keyword(s) => s,
            _ => panic!("Node is not keyword."),
        }
    }
}

impl Node {
    pub fn display(&self, s: &str) -> String {
        format!("{} at {}:{}", s, self.pos.line, self.pos.column)
    }
}
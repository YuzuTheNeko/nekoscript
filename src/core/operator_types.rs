use crate::constants::operators::{ADD, ASSIGN, DIV, EQUALS, MULTI, NOT_EQUALS, SUB, SUB_EQUAL, SUM_EQUAL};

#[derive(Clone)]
pub enum OperatorTypes {
    Equals,
    Add,
    Sub,
    Assign,
    SubAssign,
    Multi,
    Div,
    AddAssign,
    NotEquals,
}

impl OperatorTypes {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Div => DIV,
            Self::Multi => MULTI,
            Self::SubAssign => SUB_EQUAL,
            Self::AddAssign => SUM_EQUAL,
            Self::Assign => ASSIGN,
            Self::Equals => EQUALS,
            Self::Sub => SUB,
            Self::Add => ADD,
            Self::NotEquals => NOT_EQUALS,
        }
    }

    pub fn prec(&self) -> u8 {
        match self {
            Self::AddAssign | Self::Assign | Self::SubAssign => 1,
            Self::Equals | Self::NotEquals => 7,
            Self::Sub | Self::Add => 10,
            Self::Multi | Self::Div => 20
        }
    }
}

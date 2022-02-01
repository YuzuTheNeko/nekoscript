use crate::constants::operators::{ADD, ASSIGN, DIV, EQUALS, GREATER, GREATER_EQ, LESSER, LESSER_EQ, MULTI, NOT_EQUALS, SUB, SUB_EQUAL, SUM_EQUAL};

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
    GreaterEq,
    LesserEq,
    Lesser,
    Greater
}

impl OperatorTypes {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Div => DIV,
            Self::Multi => MULTI,
            Self::SubAssign => SUB_EQUAL,
            Self::AddAssign => SUM_EQUAL,
            Self::Assign => ASSIGN,
            Self::Lesser => LESSER,
            Self::LesserEq => LESSER_EQ,
            Self::Greater => GREATER,
            Self::GreaterEq => GREATER_EQ,
            Self::Equals => EQUALS,
            Self::Sub => SUB,
            Self::Add => ADD,
            Self::NotEquals => NOT_EQUALS,
        }
    }

    pub fn prec(&self) -> u8 {
        match self {
            Self::AddAssign | Self::Assign | Self::SubAssign => 1,
            Self::Equals | Self::NotEquals | Self::Lesser | Self::LesserEq | Self::Greater | Self::GreaterEq => 7,
            Self::Sub | Self::Add => 10,
            Self::Multi | Self::Div => 20
        }
    }
}

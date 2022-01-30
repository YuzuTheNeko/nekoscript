use crate::constants::operators::{ADD, ASSIGN, EQUALS, NOT_EQUALS, SUB, SUB_EQUAL, SUM_EQUAL};

#[derive(Clone)]
pub enum OperatorTypes {
    Equals,
    Add,
    Sub,
    Assign,
    SubAssign,
    AddAssign,
    NotEquals,
}

impl OperatorTypes {
    pub fn to_string(&self) -> &str {
        match self {
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
        }
    }
}

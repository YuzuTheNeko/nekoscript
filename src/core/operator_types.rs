use crate::constants::operators::{ADD, ASSIGN, EQUALS, SUB};

pub enum OperatorTypes {
    Equals,
    Assign,
    Add,
    Sub
}

impl Clone for OperatorTypes {
    fn clone(&self) -> Self {
        match self {
            Self::Add => Self::Add,
            Self::Sub => Self::Sub,
            Self::Assign => Self::Assign,
            Self::Equals => Self::Equals
        }
    }
}

impl OperatorTypes {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Equals => EQUALS,
            Self::Sub => SUB,
            Self::Assign => ASSIGN,
            Self::Add => ADD
        }
    }

    pub fn prec(&self) -> u8 {
        match self {
            Self::Equals => 7,
            Self::Sub | Self::Add => 10,
            Self::Assign => 1
        }
    }
}
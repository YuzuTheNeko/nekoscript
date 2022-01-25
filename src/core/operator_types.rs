use crate::constants::operators::{ADD, ASSIGN, EQUALS, SUB};

#[derive(Clone)]
pub enum OperatorTypes {
    Equals,
    Add,
    Sub
}

#[derive(Clone)]
pub enum SpecialOperatorTypes {
    AddAssign,
    SubAssign,
    Assign
}

impl OperatorTypes {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Equals => EQUALS,
            Self::Sub => SUB,
            Self::Add => ADD
        }
    }

    pub fn prec(&self) -> u8 {
        match self {
            Self::Equals => 7,
            Self::Sub | Self::Add => 10
        }
    }
}
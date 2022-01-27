use crate::constants::data_types::{
    ARRAY_TYPE, BOOL_TYPE, FN_TYPE, INT_TYPE, NULL_TYPE, OBJECT_TYPE, TEXT_TYPE,
};
use crate::core::nodes::Nodes;
use std::borrow::Borrow;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

#[derive(Clone)]
pub enum DataTypes {
    Text(String),
    Int(i64),
    Fn {
        params: Vec<String>,
        body: Box<Nodes>,
    },
    Bool(bool),
    Null,
    Object(Rc<RwLock<HashMap<String, DataTypes>>>),
    Array(Rc<RwLock<Vec<DataTypes>>>),
}

impl DataTypes {
    pub fn to_array(&self) -> &Rc<RwLock<Vec<DataTypes>>> {
        match self {
            DataTypes::Array(v) => v,
            _ => panic!("Value is not array"),
        }
    }

    pub fn to_dyn_fn(&self) -> (&Vec<String>, &Box<Nodes>) {
        match self {
            DataTypes::Fn { params, body } => (params, body),
            _ => panic!("Node not dyn fn"),
        }
    }

    pub fn to_int(&self) -> &i64 {
        match self {
            DataTypes::Int(i) => i,
            _ => panic!("Value is not int"),
        }
    }

    pub fn to_bool(&self) -> &bool {
        match self {
            DataTypes::Bool(i) => i,
            _ => panic!("Value is not bool"),
        }
    }

    pub fn to_null(&self) -> () {
        match self {
            DataTypes::Null => (),
            _ => panic!("Value is not null"),
        }
    }

    pub fn to_mut_int(&mut self) -> &mut i64 {
        match self {
            DataTypes::Int(i) => i,
            _ => panic!("Value not an int."),
        }
    }

    pub fn to_text(&self) -> &String {
        match self {
            DataTypes::Text(s) => s,
            _ => panic!("Value is not text"),
        }
    }

    pub fn to_object(&self) -> &Rc<RwLock<HashMap<String, DataTypes>>> {
        match self {
            DataTypes::Object(v) => v,
            _ => panic!("Value is not object"),
        }
    }
}

impl DataTypes {
    pub fn kind(&self) -> &str {
        match self {
            DataTypes::Array(_) => ARRAY_TYPE,
            DataTypes::Null => NULL_TYPE,
            DataTypes::Bool(_) => BOOL_TYPE,
            DataTypes::Int(_) => INT_TYPE,
            DataTypes::Text(_) => TEXT_TYPE,
            DataTypes::Fn { .. } => FN_TYPE,
            DataTypes::Object(_) => OBJECT_TYPE,
        }
    }

    pub fn null() -> Rc<RefCell<DataTypes>> {
        Rc::new(RefCell::new(Self::Null))
    }

    pub fn wrap(data: DataTypes) -> Rc<RefCell<DataTypes>> {
        Rc::new(RefCell::new(data))
    }
}

impl DataTypes {
    pub fn is_int(&self) -> bool {
        match self {
            DataTypes::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            DataTypes::Bool(b) => b.eq(&true),
            DataTypes::Int(b) => b.gt(&0),
            DataTypes::Text(b) => !b.is_empty(),
            _ => true,
        }
    }

    pub fn is_dyn_fn(&self) -> bool {
        match self {
            DataTypes::Fn { .. } => true,
            _ => false,
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            DataTypes::Text(_) => true,
            _ => false,
        }
    }
}

impl DataTypes {
    pub fn is_type(&self, other: &str) -> bool {
        self.kind().eq(other)
    }

    pub fn is_equal(&self, other: &mut RefMut<DataTypes>) -> bool {
        if self.is_type(other.kind()) {
            match self {
                DataTypes::Text(s) => s.eq(other.to_text()),
                DataTypes::Bool(s) => s.eq(other.to_bool()),
                DataTypes::Int(s) => s.eq(other.to_int()),
                _ => false,
            }
        } else {
            false
        }
    }
}

impl DataTypes {
    pub fn to_string(&self) -> String {
        match self {
            Self::Text(s) => s.to_string(),
            Self::Int(s) => s.to_string(),
            Self::Bool(s) => s.to_string(),
            Self::Null => NULL_TYPE.to_string(),
            _ => panic!("Cannot deserialize {} to string.", self.kind()),
        }
    }
}

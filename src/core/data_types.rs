use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;
use crate::constants::data_types::{ARRAY_TYPE, BOOL_TYPE, INT_TYPE, NULL_TYPE, OBJECT_TYPE, TEXT_TYPE};

pub enum DataTypes {
    Text(String),
    Int(i64),
    Bool(bool),
    Null,
    Object(Rc<RwLock<HashMap<String, DataTypes>>>),
    Array(Rc<RwLock<Vec<DataTypes>>>)
}

impl Clone for DataTypes {
    fn clone(&self) -> Self {
        match self {
            Self::Text(d) => Self::Text(d.clone()),
            Self::Int(i) => Self::Int(i.clone()),
            Self::Null => Self::Null,
            Self::Bool(b) => Self::Bool(b.clone()),
            Self::Array(b) => {
                let mut clone = vec![];

                let reader = b.read().unwrap();

                for i in reader.iter() {
                    clone.push(i.clone())
                }

                Self::Array(Rc::new(RwLock::new(clone)))
            },
            _ => panic!("Object cloning is not supported.")
        }
    }
}

impl DataTypes {
    pub fn to_array(&self) -> &Rc<RwLock<Vec<DataTypes>>> {
        match self {
            DataTypes::Array(v) => v,
            _ => panic!("Value is not array")
        }
    }

    pub fn to_int(&self) -> &i64 {
        match self {
            DataTypes::Int(i) => i,
            _ => panic!("Value is not int")
        }
    }

    pub fn to_bool(&self) -> &bool {
        match self {
            DataTypes::Bool(i) => i,
            _ => panic!("Value is not bool")
        }
    }

    pub fn to_null(&self) -> () {
        match self {
            DataTypes::Null => (),
            _ => panic!("Value is not null")
        }
    }

    pub fn to_text(&self) -> &String {
        match self {
            DataTypes::Text(s) => s,
            _ => panic!("Value is not text")
        }
    }

    pub fn to_object(&self) -> &Rc<RwLock<HashMap<String, DataTypes>>> {
        match self {
            DataTypes::Object(v) => v,
            _ => panic!("Value is not object")
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
            DataTypes::Object(_) => OBJECT_TYPE
        }
    }

    pub fn null() -> Self {
        Self::Null
    }
}

impl DataTypes {
    pub fn is_int(&self) -> bool {
        match self {
            DataTypes::Int(_) => true,
            _ => false
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            DataTypes::Text(_) => true,
            _ => false
        }
    }
}

impl DataTypes {
    pub fn is_type(&self, other: &str) -> bool {
        self.kind().eq(other)
    }

    pub fn is_equal(&self, other: &DataTypes) -> bool {
        if self.is_type(other.kind()) {
            match self {
                DataTypes::Text(s) => s.eq(other.to_text()),
                DataTypes::Bool(s) => s.eq(other.to_bool()),
                DataTypes::Int(s) => s.eq(other.to_int()),
                _ => false
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
            _ => panic!("Cannot deserialize {} to string.", self.kind())
        }
    }
}
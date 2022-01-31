use crate::constants::data_types::{ARRAY_TYPE, BOOL_TYPE, FN_TYPE, INT_TYPE, NULL_TYPE, OBJECT_TYPE, RAW_ARRAY_TYPE, RAW_OBJECT_TYPE, TEXT_TYPE};
use crate::core::nodes::{Node, Nodes};

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;
use crate::core::native_function::PropReturn;

#[derive(Clone, PartialOrd, PartialEq)]
pub enum NativeFnType {
    Global,
    Module
}

#[derive(Clone)]
pub enum DataTypes {
    Text(String),
    NativeFn {
        name: String,
        kind: NativeFnType,
        module: Option<String>,
        params: Option<Vec<Node>>
    },
    Int(i64),
    Fn {
        params: Vec<String>,
        body: Box<Node>,
    },
    Bool(bool),
    Null,
    RawObject(Rc<RwLock<HashMap<String, Node>>>),
    RawArray(Rc<RwLock<Vec<Node>>>),
    Array(Rc<RwLock<Vec<Rc<RefCell<DataTypes>>>>>),
    Object(Rc<RwLock<HashMap<String, Rc<RefCell<DataTypes>>>>>)
}

impl DataTypes {
    pub fn to_raw_array(&self) -> &Rc<RwLock<Vec<Node>>> {
        match self {
            DataTypes::RawArray(v) => v,
            _ => panic!("Value is not array"),
        }
    }

    pub fn to_dyn_fn(&self) -> (&Vec<String>, &Box<Node>) {
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

    pub fn to_raw_obj(&self) -> &Rc<RwLock<HashMap<String, Node>>> {
        match self {
            DataTypes::RawObject(b) => b,
            _ => panic!("Value is not object")
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

    pub fn to_array(&self) -> &Rc<RwLock<Vec<Rc<RefCell<DataTypes>>>>> {
        match self {
            DataTypes::Array(v) => v,
            _ => panic!("Value is not array")
        }
    }

    pub fn to_native_fn(&self) -> (&String, &NativeFnType, &Option<String>, &Option<Vec<Node>>) {
        match self {
            DataTypes::NativeFn { name, kind, module, params } => (name, kind, module, params),
            _ => panic!("Value is not native func")
        }
    }

    pub fn to_object(&self) -> &Rc<RwLock<HashMap<String, Rc<RefCell<DataTypes>>>>> {
        match self {
            DataTypes::Object(o) => o,
            _ => panic!("Value is not object")
        }
    }
}

impl DataTypes {
    pub fn kind(&self) -> &str {
        match self {
            DataTypes::RawArray(_) => RAW_ARRAY_TYPE,
            DataTypes::RawObject(_) => RAW_OBJECT_TYPE,
            DataTypes::Array(_) => ARRAY_TYPE,
            DataTypes::Null => NULL_TYPE,
            DataTypes::Bool(_) => BOOL_TYPE,
            DataTypes::Int(_) => INT_TYPE,
            DataTypes::Text(_) => TEXT_TYPE,
            DataTypes::Fn { .. } | DataTypes::NativeFn { .. } => FN_TYPE,
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
    pub fn is_obj(&self) -> bool {
        match self {
            DataTypes::Object(_) => true,
            _ => false
        }
    }

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

    pub fn is_native_fn(&self) -> bool {
        match self {
            DataTypes::NativeFn { .. } => true,
            _ => false
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

    pub fn is_equal(&self, other: &mut DataTypes) -> bool {
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
            Self::Array(arr) => {
                let arr = arr.read().unwrap();

                let mut s = String::from('[');

                let mut y = 0;
                let last = arr.len();

                for i in arr.iter() {
                    let val = i.borrow();
                    s.push_str(&format!("{}{}", {
                        if val.is_text() {
                            format!("\"{}\"", val.to_string())
                        } else {
                            val.to_string()
                        }
                    }, {
                        if y + 1 == last {
                            String::new()
                        } else {
                            String::from(",")
                        }
                    }));

                    y += 1;
                }

                s.push(']');

                s
            }
            Self::Object(obj) => {
                let reader = obj.read().unwrap();
                let mut s = String::from('{');

                let last = reader.len();

                let mut y = 0;

                for (key, value) in reader.iter() {
                    let value = value.borrow();
                    let str = value.to_string();

                    s.push_str(&format!("\"{}\":{}{}", key, {
                        if value.is_text() {
                            format!("\"{}\"", str)
                        } else {
                            str
                        }
                    }, {
                        if y + 1 == last {
                            String::new()
                        } else {
                            String::from(",")
                        }
                    }));

                    y += 1;
                }

                s.push('}');

                s
            }
            _ => panic!("Cannot deserialize {} to string.", self.kind()),
        }
    }
}

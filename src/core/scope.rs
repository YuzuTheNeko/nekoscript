use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;
use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;
use crate::native::loader::load_native_functions;

#[derive(Default)]
pub struct Scope {
    pub variables: Rc<RwLock<HashMap<String, DataTypes>>>,
    pub functions: Rc<RwLock<HashMap<String, i64>>>,
    pub native: Vec<NativeFunction>
}

impl Scope {
    pub fn new() -> Self {
        Self {
            native: load_native_functions(),
            ..Self::default()
        }
    }

    pub fn set(&self, key: String, value: DataTypes) {
        self.variables.write().unwrap().insert(key, value);
    }

    pub fn has(&self, key: String, value: &DataTypes) -> bool {
        self.variables.read().unwrap().contains_key(&key)
    }
}
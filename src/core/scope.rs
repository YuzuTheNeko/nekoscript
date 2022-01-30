use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;
use crate::core::nodes::Nodes;
use crate::native::loader::load_native_functions;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

#[derive(Default)]
pub struct Scope {
    pub variables: Rc<RwLock<HashMap<String, Rc<RefCell<DataTypes>>>>>,
    pub functions: Rc<RwLock<HashMap<String, Nodes>>>,
    pub native: Vec<NativeFunction>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            native: load_native_functions(),
            ..Self::default()
        }
    }

    pub fn set(&self, key: String, value: Rc<RefCell<DataTypes>>) {
        self.variables.write().unwrap().insert(key, value);
    }

    pub fn is_native_fn(&self, key: &String) -> bool {
        self.native.iter().any(|f| f.name.eq(key))
    }

    pub fn is_def_fn(&self, key: &String) -> bool {
        self.functions.read().unwrap().contains_key(key)
    }

    pub fn has(&self, key: String, value: &DataTypes) -> bool {
        self.variables.read().unwrap().contains_key(&key)
    }

    pub fn from(scope: &Scope) -> Self {
        let mut sc = Self::new();

        {
            let mut writer = sc.variables.write().unwrap();
            for i in scope.variables.read().unwrap().iter() {
                writer.insert(i.0.to_string(), i.1.clone());
            }
        }

        {
            let mut writer = sc.functions.write().unwrap();
            for i in scope.functions.read().unwrap().iter() {
                writer.insert(i.0.to_string(), i.1.clone());
            }
        }

        sc
    }
}

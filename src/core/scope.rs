use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;
use crate::core::nodes::{Node, Nodes};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::RwLock;
use crate::core::process::Process;

#[derive(Default)]
pub struct Scope {
    pub variables: Rc<RwLock<HashMap<String, Rc<RefCell<DataTypes>>>>>,
    pub functions: Rc<RwLock<HashMap<String, Node>>>,
    pub process: Rc<RefCell<Process>>
}

impl Scope {
    pub fn new(process: Rc<RefCell<Process>>) -> Self {
        Self {
            process,
            ..Self::default()
        }
    }

    pub fn set_path(&self, path: String) {
        self.set("path".to_owned(), DataTypes::wrap(DataTypes::Text(path)));
    }

    pub fn set(&self, key: String, value: Rc<RefCell<DataTypes>>) {
        self.variables.write().unwrap().insert(key, value);
    }

    pub fn is_def_fn(&self, key: &String) -> bool {
        self.functions.read().unwrap().contains_key(key)
    }

    pub fn has(&self, key: String, value: &DataTypes) -> bool {
        self.variables.read().unwrap().contains_key(&key)
    }

    pub fn folder(&self) -> String {
        let got = self.variables.read().unwrap();
        let got = got.get("path").unwrap();
        let got = got.borrow();
        let got = got.to_string();

        let path = Path::new(&got);
        if let Some(path) = path.parent() {
            path.to_str().unwrap().to_owned()
        } else {
            String::from("./")
        }
    }

    pub fn from(scope: &Scope) -> Self {
        let mut sc = Self::new(scope.process.clone());

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

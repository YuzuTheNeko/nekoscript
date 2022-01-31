use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{canonicalize, File};
use std::io::Read;
use std::path::Path;
use std::rc::Rc;
use std::sync::RwLock;
use crate::core::data_types::DataTypes;
use crate::core::native_function::NativeFunction;
use crate::{Interpreter, TokenStream};
use crate::core::interpreter::IReturn;
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::core::scope::Scope;
use crate::native::functions::loader::load_native_functions;
use crate::native::modules::get_module::get_module;

#[derive(Default)]
pub struct Process {
    pub path: String,
    pub modules: HashMap<String, Rc<RefCell<DataTypes>>>,
    pub native: HashMap<String, Rc<RefCell<NativeFunction>>>
}

impl Process {
    pub fn folder(&self) -> String {
        if let Some(path) = Path::new(&self.path).parent() {
            path.to_str().unwrap().to_owned()
        } else {
            String::from("./")
        }
    }

    pub fn load_module(scope: &Scope, mut name: String) -> IReturn {
        if let Some(module) = get_module(name.clone()) {
            Ok(module)
        } else {
            panic!("Module {} not found", name);
        }
    }

    pub fn load_file(scope: &Scope, mut pth: String) -> IReturn {
        if !pth.contains(" ") && !pth.contains("/") {
            return Self::load_module(scope, pth)
        }

        {
            let borrow = scope.process.borrow();
            pth = format!("{}/{}", borrow.folder(), pth);
        }

        match canonicalize(&pth) {
            Ok(got) => {
                let path = got.to_str().unwrap();

                {
                    let borrow = scope.process.borrow();
                    if let Some(module) = borrow.modules.get(path) {
                        return Ok(module.clone())
                    }
                }

                let mut s = String::new();

                File::open(&path).unwrap().read_to_string(&mut s);

                let mut stream = TokenStream::new(&s, path.to_owned());

                stream.start();

                let itr = Interpreter::new(stream.nodes, path.to_owned());

                let scope = Scope::new(scope.process.clone());

                scope.set_path(pth);

                {
                    let mut writer = scope.variables.write().unwrap();
                    writer.insert("public".to_owned(), Rc::new(RefCell::new(DataTypes::Object(Rc::new(RwLock::new(HashMap::new()))))));
                }

                {
                    itr.run(Some(&scope));
                }

                let reader = scope.variables.read().unwrap();
                let reader = reader.get("public").unwrap();

                {
                    let mut borrow = scope.process.borrow_mut();
                    borrow.modules.insert(path.to_owned(), reader.clone());
                }

                Ok(reader.clone())
            },
            Err(e) => Err(RuntimeError(e.to_string()))
        }
    }

    pub fn new(path: String) -> Self {
        Self {
            path,
            modules: HashMap::new(),
            native: load_native_functions()
        }
    }
}
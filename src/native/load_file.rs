
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{canonicalize, File};
use std::io::Read;
use std::path::Path;
use std::rc::Rc;
use std::sync::RwLock;
use crate::core::native_function::NativeFunction;
use crate::core::return_types::ReturnTypes::RuntimeError;
use crate::{Interpreter, TokenStream};
use crate::core::data_types::DataTypes;
use crate::core::scope::Scope;

pub fn load_file() -> NativeFunction {
    NativeFunction {
        name: String::from("load_file"),
        body: Box::new(| scope, args | {
            let path: &Rc<RefCell<DataTypes>> = args.get(0).unwrap();
            let path = path.borrow();
            let path = path.to_string();

            match canonicalize(&path) {
                Ok(got) => {
                    let path = got.to_str().unwrap();

                    let mut s = String::new();

                    File::open(&path).unwrap().read_to_string(&mut s);

                    let mut stream = TokenStream::new(&s, path.to_owned());

                    stream.start();

                    let itr = Interpreter::new(stream.nodes, path.to_owned());

                    let scope = Scope::new();

                    {
                        let mut writer = scope.variables.write().unwrap();
                        writer.insert("public".to_owned(), Rc::new(RefCell::new(DataTypes::Object(Rc::new(RwLock::new(HashMap::new()))))));
                    }

                    {
                        itr.run(Some(&scope));
                    }

                    let reader = scope.variables.read().unwrap();
                    let reader = reader.get("public").unwrap();

                    Ok(reader.clone())
                },
                Err(e) => Err(RuntimeError(e.to_string()))
            }
        })
    }
}
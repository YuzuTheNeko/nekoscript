
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
use crate::core::process::Process;
use crate::core::scope::Scope;

pub fn load_file() -> NativeFunction {
    NativeFunction(
        Box::new(| _, scope, args | {
            let path: &Rc<RefCell<DataTypes>> = args.get(0).unwrap();
            let path = path.borrow();
            let mut path = path.to_string();

            Process::load_file(scope, path)
        })
    )
}
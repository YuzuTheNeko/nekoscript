use crate::core::data_types::DataTypes;
use crate::core::scope::Scope;

pub fn across(data: &DataTypes) -> &DataTypes {
    data
}

pub fn test() {
    let scope = Scope::new();

    let d = scope.variables.read().unwrap();

    if let Some(t) = d.get("") {
        across(t);
    }
}
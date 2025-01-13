use std::{cell::RefCell, rc::Rc};

use crate::core::{
    scope::{Scope, ValueType},
    PrimitiveTypes,
};
use lv8_common::error::{Error, Result};

pub fn evaluate_namespace_expression(
    scope: &Rc<RefCell<Scope>>,
    namespace: Vec<String>,
) -> Result<ValueType> {
    let mut value = scope.borrow().get(&namespace[0]);

    if value.is_none() {
        return Err(Error::r#type(&format!(
            "cannot read properties of {:#?}",
            namespace[0]
        )));
    }

    for ident in &namespace[1..] {
        value = read_property_of_value(value, ident)?
    }

    Ok(value.unwrap_or(ValueType::Variable(PrimitiveTypes::Undefined)))
}

fn read_property_of_value(value: Option<ValueType>, ident: &str) -> Result<Option<ValueType>> {
    if value.is_none() {
        return Err(Error::r#type(&format!(
            "cannot read properties of {:#?}",
            value
        )));
    }

    match value.unwrap() {
        ValueType::Function(_) => Ok(Some(ValueType::Variable(PrimitiveTypes::Undefined))),
        ValueType::InternalFunction(_) => Ok(Some(ValueType::Variable(PrimitiveTypes::Undefined))),
        ValueType::Module(module) => Ok(module.scope.borrow().get(ident)),
        ValueType::Variable(variable) => read_property_of_variable(variable, ident),
    }
}

fn read_property_of_variable(value: PrimitiveTypes, ident: &str) -> Result<Option<ValueType>> {
    match value {
        PrimitiveTypes::Object(map) => Ok(map.get(ident).cloned()),
        _ => Ok(Some(ValueType::Variable(PrimitiveTypes::Undefined))),
    }
}

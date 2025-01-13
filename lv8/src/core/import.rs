use std::path;
use std::{cell::RefCell, rc::Rc};

use crate::core::PrimitiveTypes;

use super::{execute_file, module};
use super::{
    scope::{Scope, ValueType},
    Metadata,
};

use lv8_common::error::Result;

pub fn import_statement(
    scope: &Rc<RefCell<Scope>>,
    path: &str,
    ident: &str,
    metadata: &Rc<Metadata>,
) -> Result<ValueType> {
    let path = path::Path::new(&metadata.pw).join(path);

    let evaluator = execute_file(path)?;

    let module = module::Module {
        name: ident.to_string(),
        scope: evaluator.scope,
    };

    scope.borrow_mut().set(ident, ValueType::Module(module));

    Ok(ValueType::Variable(PrimitiveTypes::Undefined))
}

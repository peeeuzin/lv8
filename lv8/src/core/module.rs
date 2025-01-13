use std::{cell::RefCell, rc::Rc};

use lv8_common::error::Result;
use lv8_parser::Block as BlockAST;

use super::{scope::Scope, statement::run_statement, Metadata};

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub scope: Rc<RefCell<Scope>>,
}

impl Module {
    pub fn new(
        name: &str,
        scope: Rc<RefCell<Scope>>,
        body: &BlockAST,
        metadata: &Rc<Metadata>,
    ) -> Result<Self> {
        let module_scope = Rc::new(RefCell::new(Scope::with_parent(name, Rc::clone(&scope))));

        for statement in &body.0 {
            run_statement(&module_scope, statement, metadata)?;
        }

        Ok(Self {
            name: name.to_owned(),
            scope: module_scope,
        })
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Module {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
